use serde::Deserialize;

// Structure to deserialize the incoming request body for /v1/chat/completions
#[derive(Debug, Deserialize)]
pub struct OpenAiChatRequest {
    // We primarily need messages and model
    pub messages: Vec<OpenAiMessage>,
    pub model: Option<String>, // Model name might be used for  options
    // #[serde(default)] // Default to false if not present
    // pub stream: bool,
    // We don't necessarily need to deserialize other fields like temperature, top_p etc.
    // unless we plan to map them to  options, which seems unlikely based on JS analysis.
    // #[serde(flatten)]
    // pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct OpenAiMessage {
    // pub role: String, // e.g., "user", "system", "assistant"
    pub content: String,
    // name, tool_calls, tool_call_id can be ignored for now
} 