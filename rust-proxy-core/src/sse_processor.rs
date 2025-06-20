use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::warn;
use crate::dev_client::DevRequestOptions; // Needed for model name
use anyhow::Result;
use futures_util::stream::{self, Stream, StreamExt};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, info, error, trace};
use bytes::Bytes;
use std::str;
use std::pin::Pin;

// --- Dev API SSE Event Data Structures (Based on JS analysis) ---

// Represents the different types of actions Dev might send
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct DevAction {
    #[serde(rename = "type")] 
    pub action_type: u32,
    // Other fields based on actual action data...
    #[serde(flatten)] 
    pub extra: Value, // Capture unknown fields
}

// Represents source information
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct DevSource {
    pub title: Option<String>,
    pub url: Option<String>,
    // Other fields...
    #[serde(flatten)] 
    pub extra: Value, // Capture unknown fields
}

// Represents GitHub source information
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct DevGithubSource {
    pub repo: Option<String>,
    #[serde(rename = "filePath")]
    pub file_path: Option<String>,
    // Other fields...
    #[serde(flatten)] 
    pub extra: Value, // Capture unknown fields
}

// Main accumulator state, mirroring JS accumulator
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SseAccumulator {
    pub text: String,
    pub actions: Vec<DevAction>,
    pub sources: Vec<DevSource>,
    pub github_sources: Vec<DevGithubSource>,
    pub related_questions: Vec<String>,
    related_questions_raw: String, // Keep raw temporarily for parsing

    pub thread_id: Option<String>,
    pub query_message_id: Option<String>,
    pub answer_message_id: Option<String>,
    pub thread_title: Option<String>,
    pub reasoning: Option<String>,

    pub is_finished: bool,
    pub error: Option<String>,
}

impl SseAccumulator {
    // Helper to parse related questions, similar to JS logic
    fn update_related_questions(&mut self) {
        self.related_questions = self.related_questions_raw
            .split('\n')
            .map(|q| q.trim())
            .filter(|q| !q.is_empty())
            .map(String::from)
            .collect();
    }
}

// --- OpenAI Chat Completion Chunk Structures ---

#[derive(Debug, Serialize)]
pub struct ChatCompletionChunk {
    pub id: String, // Consider using nonce or generating new IDs
    pub object: String, // Typically "chat.completion.chunk"
    pub created: u64, // Unix timestamp
    pub model: String, // Model name from request or default
    pub choices: Vec<Choice>,
}

#[derive(Debug, Serialize)]
pub struct Choice {
    pub index: u32,
    pub delta: Delta,
    pub finish_reason: Option<String>, // e.g., "stop", "length"
}

#[derive(Debug, Serialize, Default)]
pub struct Delta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>, // e.g., "assistant"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

// Helper function to safely parse JSON from SSE data
fn safe_json_parse<'a, T>(data: &'a str) -> Option<T>
where
    T: Deserialize<'a>,
{
    match serde_json::from_str::<T>(data) {
        Ok(parsed) => Some(parsed),
        Err(e) => {
            warn!("Failed to parse JSON from SSE data: {}. Data: {}", e, data);
            None
        }
    }
}

// Enum to represent parsed SSE lines
#[derive(Debug, PartialEq, Eq)]
enum SseLine {
    Event(String),
    Data(String),
    Retry(String),
    Id(String),
    Comment,
    Empty, // End of an event
}

// Parses a single line according to SSE format
fn parse_sse_line(line: &str) -> SseLine {
    if line.is_empty() {
        SseLine::Empty
    } else if line.starts_with(':') {
        SseLine::Comment
    } else {
        let (field, value) = line.split_once(':').unwrap_or((line, ""));
        // Trim leading space from value if present
        let value = value.strip_prefix(' ').unwrap_or(value);
        match field {
            "event" => SseLine::Event(value.to_string()),
            "data" => SseLine::Data(value.to_string()),
            "id" => SseLine::Id(value.to_string()),
            "retry" => SseLine::Retry(value.to_string()),
            _ => SseLine::Comment, // Treat unknown fields as comments
        }
    }
}

/// Processes a stream of Dev Bytes and transforms it into a 
/// stream of OpenAI-compatible ChatCompletionChunks using stream::unfold.
pub fn process_dev_bytes_stream_unfold(
    byte_stream: impl Stream<Item = Result<Bytes, reqwest::Error>> + Send + 'static,
    options: DevRequestOptions, 
    request_id: String, 
) -> impl Stream<Item = Result<ChatCompletionChunk>> {
    let model_name = options.model.unwrap_or_else(|| "unknown-dev-model".to_string());

    // State for unfold
    struct State {
        byte_stream: Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send + 'static>>,
        decoder_buffer: String,
        accumulator: SseAccumulator,
        current_event_name: String,
        current_data_buffer: Vec<String>,
        model_name: String,
        request_id: String,
        final_chunk_sent: bool, // Flag to ensure unfold terminates correctly
    }

    let initial_state = State {
        byte_stream: Box::pin(byte_stream),
        decoder_buffer: String::new(),
        accumulator: SseAccumulator::default(),
        current_event_name: "message".to_string(),
        current_data_buffer: Vec::new(),
        model_name,
        request_id,
        final_chunk_sent: false, // Initialize the flag
    };

    stream::unfold(initial_state, |mut state| async move {
        // Check if the final chunk was already sent in the previous iteration
        if state.final_chunk_sent {
            return None; // Terminate the unfold stream
        }

        // Loop to read bytes and process lines until an event is dispatched or stream ends
        loop {
            let mut event_chunk: Option<Result<ChatCompletionChunk>> = None;

            // --- Process buffered lines first ---
            while let Some(newline_pos) = state.decoder_buffer.find('\n') {
                let line = state.decoder_buffer.drain(..=newline_pos).collect::<String>();
                let line = line.trim_end_matches('\n').trim_end_matches('\r');

                trace!(line = %line, "Processing buffered line");
                
                match parse_sse_line(line) {
                    SseLine::Event(event_name) => {
                        trace!(event_name = %event_name, "Parsed event name");
                        state.current_event_name = event_name;
                    }
                    SseLine::Data(data) => {
                        trace!(data = %data, "Parsed data line");
                        state.current_data_buffer.push(data);
                    }
                    SseLine::Empty => {
                        // End of event - process it
                        if !state.current_data_buffer.is_empty() {
                            let joined_data = state.current_data_buffer.join("\n");
                            trace!(event = %state.current_event_name, data = %joined_data, "Processing complete event");
                            
                            if let Some(chunk) = process_single_dev_event(
                                &mut state.accumulator,
                                state.current_event_name.clone(),
                                joined_data,
                                &state.request_id,
                                &state.model_name,
                            ) {
                                event_chunk = Some(Ok(chunk));
                                state.current_data_buffer.clear();
                                break; // Exit processing loop to yield this chunk
                            }
                            state.current_data_buffer.clear();
                        }
                        // Reset for next event
                        state.current_event_name = "message".to_string();
                    }
                    SseLine::Id(_) | SseLine::Retry(_) | SseLine::Comment => {
                        // These don't affect our processing
                        trace!("Ignoring SSE line: {:?}", parse_sse_line(line));
                    }
                }
            }

            // If we have a chunk to yield, return it
            if let Some(chunk) = event_chunk {
                return Some((chunk, state));
            }

            // --- Read more bytes from the stream ---
            match state.byte_stream.next().await {
                Some(Ok(bytes)) => {
                    // Convert bytes to string and append to buffer
                    match str::from_utf8(&bytes) {
                        Ok(text) => {
                            trace!(byte_count = bytes.len(), "Received bytes from stream");
                            state.decoder_buffer.push_str(text);
                        }
                        Err(e) => {
                            error!("Failed to decode bytes as UTF-8: {}", e);
                            let error_chunk = create_error_chunk(
                                state.request_id.clone(),
                                state.model_name.clone(),
                                format!("UTF-8 decode error: {}", e),
                            );
                            state.final_chunk_sent = true;
                            return Some((Ok(error_chunk), state));
                        }
                    }
                }
                Some(Err(e)) => {
                    error!("Stream error: {}", e);
                    let error_chunk = create_error_chunk(
                        state.request_id.clone(),
                        state.model_name.clone(),
                        format!("Stream error: {}", e),
                    );
                    state.final_chunk_sent = true;
                    return Some((Ok(error_chunk), state));
                }
                None => {
                    // Stream ended - process any remaining buffer and send final chunk
                    trace!("Stream ended, processing residual buffer");
                    
                    // Process any remaining lines in buffer
                    if !state.decoder_buffer.is_empty() {
                        let remaining_lines: Vec<&str> = state.decoder_buffer.lines().collect();
                        for line in remaining_lines {
                            trace!(line = %line, "Processing residual line");
                            match parse_sse_line(line) {
                                SseLine::Event(event_name) => {
                                    state.current_event_name = event_name;
                                }
                                SseLine::Data(data) => {
                                    state.current_data_buffer.push(data);
                                }
                                _ => {}
                            }
                        }
                        
                        // Process final event if we have data
                        if !state.current_data_buffer.is_empty() {
                            let joined_data = state.current_data_buffer.join("\n");
                            if let Some(_chunk) = process_single_dev_event(
                                &mut state.accumulator,
                                state.current_event_name.clone(),
                                joined_data,
                                &state.request_id,
                                &state.model_name,
                            ) {
                                // Note: We don't yield this chunk here as we need to send the final chunk
                                // The content is already accumulated in the accumulator
                            }
                        }
                        state.decoder_buffer.clear();
                    }

                    // Send final chunk or terminate
                    state.final_chunk_sent = true;
                    
                    if !state.accumulator.is_finished {
                        // Stream ended normally
                        state.accumulator.is_finished = true;
                        state.accumulator.update_related_questions();
                        let final_chunk = create_final_chunk(
                            state.request_id.clone(),
                            state.model_name.clone(),
                            "stop".to_string(),
                        );
                        debug!(request_id = %state.request_id, "Yielding final 'stop' chunk for normally finished stream.");
                        return Some((Ok(final_chunk), state));
                    } else {
                        // Stream ended, but error was already processed
                        debug!(request_id = %state.request_id, "Accumulator already marked as finished (likely due to prior error event). Terminating stream without final chunk.");
                        return None;
                    }
                }
            }
        }
    })
}

// Helper function to process a single parsed Dev event and potentially create a chunk
fn process_single_dev_event(
    accumulator: &mut SseAccumulator,
    event_name: String,
    data: String,
    request_id: &str,
    model_name: &str,
) -> Option<ChatCompletionChunk> {
    trace!(event = %event_name, data = %data, request_id = request_id, "Processing single Dev event");
    match event_name.as_str() {
        "message" | "content" | "c" => {
            if data.is_empty() {
                trace!("Skipping empty content/message event.");
                None
            } else {
                let delta_content = data;
                accumulator.text += &delta_content;
                Some(create_content_chunk(
                    request_id.to_string(),
                    model_name.to_string(),
                    delta_content,
                ))
            }
        }
        "action" => {
            match safe_json_parse::<DevAction>(&data) {
                Some(a) => {
                    trace!(action = ?a, "Parsed action event");
                    accumulator.actions.push(a);
                }
                None => warn!(data = %data, "Failed to parse action event data"),
            }
            None // Actions don't generate OpenAI chunks directly
        }
        "sources" => {
            match safe_json_parse::<Vec<DevSource>>(&data) {
                Some(s) => {
                    trace!(sources = ?s, "Parsed sources event");
                    accumulator.sources = s;
                }
                None => warn!(data = %data, "Failed to parse sources event data"),
            }
            None
        }
        "repoSources" => {
            match safe_json_parse::<Vec<DevGithubSource>>(&data) {
                Some(gs) => {
                    trace!(github_sources = ?gs, "Parsed repoSources event");
                    accumulator.github_sources = gs;
                }
                None => warn!(data = %data, "Failed to parse repoSources event data"),
            }
            None
        }
        "rlq" | "q" => {
            if !data.is_empty() {
                accumulator.related_questions_raw += &format!("\n{}", data.trim());
                trace!(raw_related = %accumulator.related_questions_raw, "Appended related question data");
            }
            None
        }
        "r" => {
            accumulator.reasoning.get_or_insert_with(String::new).push_str(&data);
            trace!(reasoning = ?accumulator.reasoning, "Appended reasoning data");
            None
        }
        "threadId" => { 
            accumulator.thread_id = Some(data); 
            trace!(thread_id = ?accumulator.thread_id, "Set thread ID"); 
            None 
        }
        "queryMessageId" => { 
            accumulator.query_message_id = Some(data); 
            trace!(query_message_id = ?accumulator.query_message_id, "Set query message ID"); 
            None 
        }
        "answerMessageId" => { 
            accumulator.answer_message_id = Some(data); 
            trace!(answer_message_id = ?accumulator.answer_message_id, "Set answer message ID"); 
            None 
        }
        "threadTitle" => { 
            accumulator.thread_title = Some(data); 
            trace!(thread_title = ?accumulator.thread_title, "Set thread title"); 
            None 
        }
        "error" => {
            error!(error_message = %data, request_id = request_id, "Received error event from Dev stream");
            accumulator.error = Some(data.clone());
            accumulator.is_finished = true;
            Some(create_error_chunk(
                request_id.to_string(),
                model_name.to_string(),
                data
            ))
        }
        "finish" => {
            info!(request_id = request_id, "Received explicit 'finish' event from Dev stream.");
            None
        }
        _ => {
            trace!(event_name = event_name, "Ignoring unknown or unhandled Dev event type.");
            None
        }
    }
}

// Helper to create a content chunk
fn create_content_chunk(id: String, model: String, content: String) -> ChatCompletionChunk {
    ChatCompletionChunk {
        id,
        object: "chat.completion.chunk".to_string(),
        created: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        model,
        choices: vec![Choice {
            index: 0,
            delta: Delta {
                role: Some("assistant".to_string()),
                content: Some(content),
            },
            finish_reason: None,
        }],
    }
}

// Helper to create the final chunk for normal stream completion
fn create_final_chunk(id: String, model: String, finish_reason: String) -> ChatCompletionChunk {
    debug!(request_id = %id, finish_reason = %finish_reason, "Creating final chunk");
    ChatCompletionChunk {
        id,
        object: "chat.completion.chunk".to_string(),
        created: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        model,
        choices: vec![Choice {
            index: 0,
            delta: Delta::default(),
            finish_reason: Some(finish_reason),
        }],
    }
}

// Helper to create a chunk representing an error received from the Dev stream
fn create_error_chunk(id: String, model: String, error_message: String) -> ChatCompletionChunk {
    warn!(request_id = %id, error = %error_message, "Creating error chunk");
    ChatCompletionChunk {
        id,
        object: "chat.completion.chunk".to_string(),
        created: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        model,
        choices: vec![Choice {
            index: 0,
            delta: Delta {
                role: Some("assistant".to_string()),
                content: Some(format!("[STREAM_ERROR]: {}", error_message)),
            },
            finish_reason: Some("stop".to_string()),
        }],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sse_line_empty() {
        assert_eq!(parse_sse_line(""), SseLine::Empty);
    }
    #[test]
    fn test_parse_sse_line_comment() {
        assert_eq!(parse_sse_line(": this is a comment"), SseLine::Comment);
    }

    #[test]
    fn test_parse_sse_line_event() {
        assert_eq!(
            parse_sse_line("event: message"),
            SseLine::Event("message".to_string())
        );
    }

    #[test]
    fn test_parse_sse_line_data() {
        assert_eq!(
            parse_sse_line("data: {\"key\": \"value\"}"),
            SseLine::Data("{\"key\": \"value\"}".to_string())
        );
    }
    
    #[test]
    fn test_parse_sse_line_data_strips_leading_space() {
         assert_eq!(
            parse_sse_line("data: {\"key\": \"value\"}"),
            SseLine::Data("{\"key\": \"value\"}".to_string())
        );
    }

    #[test]
    fn test_parse_sse_line_id() {
        assert_eq!(parse_sse_line("id: 123"), SseLine::Id("123".to_string()));
    }
    #[test]
    fn test_parse_sse_line_retry() {
        assert_eq!(parse_sse_line("retry: 5000"), SseLine::Retry("5000".to_string()));
    }
    #[test]
    fn test_parse_sse_line_unknown_field() {
        assert_eq!(parse_sse_line("unknown: value"), SseLine::Comment);
    }

    // Helper for tests
    fn default_accumulator() -> SseAccumulator {
        SseAccumulator::default()
    }

    #[test]
    fn test_process_event_content() {
        let mut acc = default_accumulator();
        let data = r#"{"text": "Hello"}"#.to_string();
        let chunk = process_single_dev_event(&mut acc, "content".to_string(), data, "req-1", "model-1");

        assert_eq!(acc.text, "Hello");
        assert!(chunk.is_some());
        let choice = &chunk.unwrap().choices[0];
        assert_eq!(choice.delta.content.as_deref(), Some("Hello"));
    }

    #[test]
    fn test_process_event_message() {
        let mut acc = default_accumulator();
        let data = r#"{"text": " World"}"#.to_string();
        process_single_dev_event(&mut acc, "content".to_string(), r#"{"text": "Hello"}"#.to_string(), "req-1", "model-1");
        let chunk = process_single_dev_event(&mut acc, "message".to_string(), data, "req-1", "model-1");

        assert_eq!(acc.text, "Hello World");
        let choice = &chunk.unwrap().choices[0];
        assert_eq!(choice.delta.content.as_deref(), Some(" World"));
    }

    #[test]
    fn test_process_event_c_alias() {
        let mut acc = default_accumulator();
        let data = r#"{"text": "Aliased"}"#.to_string();
        let chunk = process_single_dev_event(&mut acc, "c".to_string(), data, "req-1", "model-1");
        
        assert_eq!(acc.text, "Aliased");
        let choice = &chunk.unwrap().choices[0];
        assert_eq!(choice.delta.content.as_deref(), Some("Aliased"));
    }

    #[test]
    fn test_process_event_action() {
        let mut acc = default_accumulator();
        let data = r#"{"type": 1, "some_data": "value"}"#.to_string();
        let chunk = process_single_dev_event(&mut acc, "action".to_string(), data, "req-1", "model-1");

        assert_eq!(acc.actions.len(), 1);
        assert_eq!(acc.actions[0].action_type, 1);
        assert!(chunk.is_none());
    }

    #[test]
    fn test_process_event_action_invalid_json() {
        let mut acc = default_accumulator();
        let data = r#"{"type: 1}"#.to_string(); // Invalid JSON
        let chunk = process_single_dev_event(&mut acc, "action".to_string(), data, "req-1", "model-1");

        assert!(acc.actions.is_empty());
        assert!(chunk.is_none());
    }

    #[test]
    fn test_process_event_sources() {
        let mut acc = default_accumulator();
        let data = r#"[{"title": "Doc 1", "url": "http://a.com"}]"#.to_string();
        let chunk = process_single_dev_event(&mut acc, "sources".to_string(), data, "req-1", "model-1");

        assert_eq!(acc.sources.len(), 1);
        assert_eq!(acc.sources[0].title.as_deref(), Some("Doc 1"));
        assert!(chunk.is_none());
    }

    #[test]
    fn test_process_event_repo_sources() {
        let mut acc = default_accumulator();
        let data = r#"[{"repo": "org/repo", "filePath": "src/main.rs"}]"#.to_string();
        let chunk = process_single_dev_event(&mut acc, "repoSources".to_string(), data, "req-1", "model-1");

        assert_eq!(acc.github_sources.len(), 1);
        assert_eq!(acc.github_sources[0].repo.as_deref(), Some("org/repo"));
        assert!(chunk.is_none());
    }

    #[test]
    fn test_process_event_rlq_and_q() {
        let mut acc = default_accumulator();
        process_single_dev_event(&mut acc, "rlq".to_string(), "Question 1".to_string(), "req-1", "model-1");
        process_single_dev_event(&mut acc, "q".to_string(), "Question 2".to_string(), "req-1", "model-1");
        
        assert_eq!(acc.related_questions.len(), 2);
        assert_eq!(acc.related_questions[0], "Question 1");
        assert_eq!(acc.related_questions[1], "Question 2");
    }

    #[test]
    fn test_process_event_reasoning() {
        let mut acc = default_accumulator();
        process_single_dev_event(&mut acc, "r".to_string(), r#"{"text":"First part. "}"#.to_string(), "req-1", "model-1");
        process_single_dev_event(&mut acc, "r".to_string(), r#"{"text":"Second part."}"#.to_string(), "req-1", "model-1");
        assert_eq!(acc.reasoning, Some("First part. Second part.".to_string()));
    }

    #[test]
    fn test_process_event_metadata() {
        let mut acc = default_accumulator();
        let data = r#"{
            "threadId": "th-123",
            "queryMessageId": "q-456",
            "answerMessageId": "a-789",
            "threadTitle": "Test Thread"
        }"#.to_string();
        let chunk = process_single_dev_event(&mut acc, "threadTitle".to_string(), data, "req-1", "model-1");

        assert_eq!(acc.thread_title.as_deref(), Some("Test Thread"));
        assert!(chunk.is_none());
    }

    #[test]
    fn test_process_event_error() {
        let mut acc = default_accumulator();
        let data = "Something went wrong".to_string();
        let chunk = process_single_dev_event(&mut acc, "error".to_string(), data, "req-1", "model-1");

        assert_eq!(acc.error.as_deref(), Some("Something went wrong"));
        assert!(acc.is_finished);
        assert!(chunk.is_none()); // Error event itself doesn't produce a chunk
    }
    
    #[test]
    fn test_process_event_unknown() {
        let mut acc = default_accumulator();
        let original_acc = acc.clone();
        let chunk = process_single_dev_event(&mut acc, "unknown_event".to_string(), "data".to_string(), "req-1", "model-1");

        // Ensure accumulator is unchanged
        assert_eq!(acc.text, original_acc.text);
        assert_eq!(acc.actions.len(), original_acc.actions.len());
        assert!(chunk.is_none());
    }
}