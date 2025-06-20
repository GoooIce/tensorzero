use std::time::Instant;

use crate::cache::ModelProviderRequest;
use crate::endpoints::inference::InferenceCredentials;
use crate::error::{Error, ErrorDetails};
use crate::inference::providers::provider_trait::InferenceProvider;
use crate::inference::types::{
    ModelInferenceRequest, ProviderInferenceResponse, RequestMessage, ContentBlock, Usage, 
    Latency, FinishReason, ContentBlockOutput, Text, Role, current_timestamp, 
    ProviderInferenceResponseChunk, ContentBlockChunk, TextChunk
};
use crate::inference::types::batch::{BatchRequestRow, PollBatchInferenceResponse, StartBatchProviderInferenceResponse};
use crate::model::ModelProvider;
use reqwest::Client;
use rust_proxy_core::dev_client::{DevApiClient, DevRequestOptions};
use rust_proxy_core::sse_processor::ChatCompletionChunk;

const PROVIDER_TYPE: &str = "RustProxy";

pub struct RustProxyProvider {
    client: DevApiClient,
}

impl std::fmt::Debug for RustProxyProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RustProxyProvider")
            .field("client", &"<DevApiClient>")
            .finish()
    }
}

impl RustProxyProvider {
    pub fn new() -> Result<Self, Error> {
        let client = DevApiClient::new()
            .map_err(|e| Error::new(ErrorDetails::InferenceClient { 
                message: format!("Failed to create DevApiClient: {}", e),
                status_code: None,
                provider_type: PROVIDER_TYPE.to_string(),
                raw_request: None,
                raw_response: None,
            }))?;
        
        Ok(Self { client })
    }

    /// Convert TensorZero messages to a simple string content for rust_proxy
    fn messages_to_content(messages: &[RequestMessage]) -> String {
        messages.iter()
            .map(|msg| {
                let role_str = match msg.role {
                    Role::User => "User",
                    Role::Assistant => "Assistant",
                };
                
                let content = msg.content.iter()
                    .map(|block| match block {
                        ContentBlock::Text(text) => text.text.clone(),
                        _ => "[Non-text content]".to_string(),
                    })
                    .collect::<Vec<_>>()
                    .join(" ");
                
                format!("{}: {}", role_str, content)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Build DevRequestOptions from ModelInferenceRequest
    fn build_request_options(_request: &ModelInferenceRequest) -> DevRequestOptions {
        DevRequestOptions {
            model: Some("gpt-4".to_string()),
            search_mode: Some("web".to_string()),
            is_expert: Some(false),
            language: Some("en".to_string()),
            ..Default::default()
        }
    }

    /// Convert SSE ChatCompletionChunk to TensorZero ProviderInferenceResponseChunk format
    fn convert_chunk_to_response_chunk(
        chunk: ChatCompletionChunk,
        chunk_id: String,
        latency: std::time::Duration,
    ) -> Result<ProviderInferenceResponseChunk, Error> {
        if chunk.choices.is_empty() {
            return Err(Error::new(ErrorDetails::InferenceClient { 
                message: "Empty choices in chunk".to_string(),
                status_code: None,
                provider_type: PROVIDER_TYPE.to_string(),
                raw_request: None,
                raw_response: Some(serde_json::to_string(&chunk).unwrap_or_default()),
            }));
        }

        let choice = &chunk.choices[0];
        let content_text = choice.delta.content.clone().unwrap_or_default();
        
        let finish_reason = match choice.finish_reason.as_deref() {
            Some("stop") => Some(FinishReason::Stop),
            Some("length") => Some(FinishReason::Length),
            _ => None,
        };

        let content = if !content_text.is_empty() {
            vec![ContentBlockChunk::Text(TextChunk {
                id: chunk_id,
                text: content_text,
            })]
        } else {
            vec![]
        };

        Ok(ProviderInferenceResponseChunk {
            content,
            created: current_timestamp(),
            usage: None, // Could be extracted from chunk if available
            raw_response: serde_json::to_string(&chunk).unwrap_or_default(),
            latency,
            finish_reason,
        })
    }
}

impl InferenceProvider for RustProxyProvider {
    fn infer<'a>(
        &'a self,
        request: ModelProviderRequest<'a>,
        _http_client: &'a reqwest::Client,
        _dynamic_api_keys: &'a InferenceCredentials,
        _model_provider: &'a ModelProvider,
    ) -> impl futures::Future<Output = Result<ProviderInferenceResponse, Error>> + Send + 'a {
        Box::pin(async move {
            let start_time = Instant::now();
            let content = Self::messages_to_content(&request.request.messages);
            let options = Self::build_request_options(&request.request);
            let raw_request = content.clone();

            let response = self.client
                .send_request(&content, options)
                .await
                .map_err(|e| Error::new(ErrorDetails::InferenceClient { 
                    message: format!("Request failed: {}", e),
                    status_code: None,
                    provider_type: PROVIDER_TYPE.to_string(),
                    raw_request: Some(raw_request.clone()),
                    raw_response: None,
                }))?;

            let response_text = response
                .text()
                .await
                .map_err(|e| Error::new(ErrorDetails::InferenceClient { 
                    message: format!("Failed to read response: {}", e),
                    status_code: None,
                    provider_type: PROVIDER_TYPE.to_string(),
                    raw_request: Some(raw_request.clone()),
                    raw_response: None,
                }))?;

            let output = vec![ContentBlockOutput::Text(Text { text: response_text.clone() })];

            Ok(ProviderInferenceResponse {
                id: request.request.inference_id,
                created: current_timestamp(),
                output,
                system: request.request.system.clone(),
                input_messages: request.request.messages.clone(),
                raw_request,
                raw_response: response_text,
                usage: Usage {
                    input_tokens: 0, // TODO: Calculate actual tokens
                    output_tokens: 0, // TODO: Calculate actual tokens
                },
                latency: Latency::NonStreaming {
                    response_time: start_time.elapsed(),
                },
                finish_reason: Some(FinishReason::Stop),
            })
        })
    }

    fn infer_stream<'a>(
        &'a self,
        request: ModelProviderRequest<'a>,
        _http_client: &'a reqwest::Client,
        _dynamic_api_keys: &'a InferenceCredentials,
        _model_provider: &'a ModelProvider,
    ) -> impl futures::Future<Output = Result<(crate::inference::types::PeekableProviderInferenceResponseStream, String), Error>> + Send + 'a {
        Box::pin(async move {
            let _inference_id = request.request.inference_id.to_string();
            
            // TODO: Implement full streaming support 
            // The infrastructure is ready (SSE processor, DevApiClient stream method),
            // but Rust lifetime requirements for streaming are complex.
            // For now, return an error but preserve the infrastructure.
            
            Err(Error::new(ErrorDetails::InferenceClient {
                message: "Streaming support temporarily disabled due to lifetime complexity. Infrastructure is ready for future implementation.".to_string(),
                status_code: None,
                provider_type: PROVIDER_TYPE.to_string(),
                raw_request: None,
                raw_response: None,
            }))
        })
    }

    fn start_batch_inference<'a>(
        &'a self,
        _requests: &'a [ModelInferenceRequest],
        _client: &'a Client,
        _dynamic_api_keys: &'a InferenceCredentials,
    ) -> impl futures::Future<Output = Result<StartBatchProviderInferenceResponse, Error>> + Send + 'a {
        Box::pin(async move {
            Err(Error::new(ErrorDetails::InferenceClient { 
                message: "Batch inference not supported for RustProxy provider".to_string(),
                status_code: None,
                provider_type: PROVIDER_TYPE.to_string(),
                raw_request: None,
                raw_response: None,
            }))
        })
    }

    fn poll_batch_inference<'a>(
        &'a self,
        _batch_request: &'a BatchRequestRow<'a>,
        _http_client: &'a reqwest::Client,
        _dynamic_api_keys: &'a InferenceCredentials,
    ) -> impl futures::Future<Output = Result<PollBatchInferenceResponse, Error>> + Send + 'a {
        Box::pin(async move {
            Err(Error::new(ErrorDetails::InferenceClient { 
                message: "Batch inference not supported for RustProxy provider".to_string(),
                status_code: None,
                provider_type: PROVIDER_TYPE.to_string(),
                raw_request: None,
                raw_response: None,
            }))
        })
    }
} 