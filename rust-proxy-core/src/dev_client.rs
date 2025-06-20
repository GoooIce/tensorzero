use crate::{utils, wasm_signer::WasmSigner};
use anyhow::{anyhow, Context, Result};
// use bytes::Bytes;
use http::HeaderMap;
use reqwest::{Client, Response, RequestBuilder};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, instrument, info, error, warn};
// use crate::sse_processor::SseAccumulator;
// use futures_util::stream::{Stream, TryStreamExt};
// use http::StatusCode;
use std::env; // Import the env module
use std::collections::HashMap;
use std::str;
use std::io;
use futures_util::Stream;
use bytes::Bytes;

// Represents the parameters needed to make the final request
#[derive(Debug)]
pub struct BuiltRequestParams {
    pub url: String,
    pub headers: HeaderMap,
    pub body: String,
    // pub nonce: String, // Keep nonce if needed later for accumulator
    // pub extra_payload: ExtraPayload, // Keep if needed later
}

// Options to customize the Dev API request, mirroring JS options
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DevRequestOptions {
    pub sid: Option<String>, // Session ID?
    pub model: Option<String>,
    pub search_mode: Option<String>, // "web", "chat", etc.
    pub is_expert: Option<bool>,
    pub language: Option<String>,   // e.g., "en", "zh"
    pub thread_id: Option<String>,
    pub plugin_action: Option<String>,
    pub programming_language: Option<String>, 
}

// Structure for the "extra" field in the request body
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExtraPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    search_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_expert: Option<bool>,
    plugin_for: String, // Seems constant "vscode" in JS
    #[serde(skip_serializing_if = "Option::is_none")]
    plugin_action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    programming_language: Option<String>,
}

// Structure for the main request body sent to Dev API
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DevRequestBody {
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    thread_id: Option<String>,
    extra: ExtraPayload,
}

pub struct DevApiClient {
    client: Client,
    wasm_signer: &'static WasmSigner,
    // Add fields for configuration
    api_endpoint: String,
    device_id: String,
    os_type: String,
    sid: String,
}

// Manually implement Clone
impl Clone for DevApiClient {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            wasm_signer: self.wasm_signer, // &'static is Copy
            // Clone the new fields
            api_endpoint: self.api_endpoint.clone(),
            device_id: self.device_id.clone(),
            os_type: self.os_type.clone(),
            sid: self.sid.clone(),
        }
    }
}

impl DevApiClient {
    pub fn new() -> Result<Self> {
        // Read configuration from environment variables with defaults
        let api_endpoint = env::var("API_ENDPOINT")
            .unwrap_or_else(|_| "https://xxx".to_string());
        let device_id = env::var("DEVICE_ID")
            .unwrap_or_else(|_| "xxxx".to_string());
        let os_type = env::var("OS_TYPE")
            .unwrap_or_else(|_| "3".to_string());
        let sid = env::var("SID")
        .unwrap_or_else(|_|"sid".to_string());
            
        info!(api_endpoint, device_id, os_type, "DevApiClient configured");
        // debug!("api_endpoint: {}", api_endpoint);
        // debug!("device_id: {}", device_id);
        // debug!("os_type: {}", os_type);
        // debug!("sid: {}", sid);

        let client = Client::builder().build()
            .context("Failed to build reqwest client")?;
        let wasm_signer = WasmSigner::get_instance()
            .context("Failed to get WasmSigner instance")?; // Propagate error if init failed
            
        Ok(Self {
            client,
            wasm_signer,
            // Store the configuration
            api_endpoint,
            device_id,
            os_type,
            sid,
        })
    }

    #[instrument(skip(self, content, options), fields(content_len = content.len()))]
    pub fn build_request_params(
        &self,
        content: &str,
        options: &DevRequestOptions,
    ) -> Result<BuiltRequestParams> {
        debug!("Building request parameters using configured endpoint...");

        // 1. Timestamp and Nonce
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs()
            .to_string();
        let nonce = utils::generate_uuidv4();
        debug!(timestamp, nonce, "Generated timestamp and nonce");

        // 2. Device ID (Use configured value)
        debug!(device_id = %self.device_id, "Using configured device ID");

        // 3. WASM Signature
        debug!("Calling WASM signer...");
        let signature = self.wasm_signer.sign(
            &nonce,
            &timestamp,
            &self.device_id, // Pass configured device_id
            content,
        ).context("Failed to get signature from WASM")?;
        debug!(signature, "Signature received from WASM");

        // 4. Build Headers
        let mut headers = HeaderMap::new();
        headers.insert(http::header::CONTENT_TYPE, "application/json".parse()?);
        // Use self.device_id and self.os_type
        headers.insert("device-id", self.device_id.parse()?);
        headers.insert("os-type", self.os_type.parse()?);
        headers.insert("nonce", nonce.parse()?);
        headers.insert("timestamp", timestamp.parse()?);
        headers.insert("sign", signature.parse()?);
        headers.insert("sid", self.sid.parse()?);

        debug!(?headers, "Constructed headers");

        println!("headers: {:?}", headers);

        // 5. Build Body
        let extra_payload = ExtraPayload {
            search_mode: options.search_mode.clone(),
            model: options.model.clone(),
            is_expert: options.is_expert,
            plugin_for: "vscode".to_string(), // Still hardcoded
            plugin_action: options.plugin_action.clone(),
            language: options.language.clone(),
            programming_language: options.programming_language.clone(),
        };

        let request_body = DevRequestBody {
            content: content.to_string(),
            thread_id: options.thread_id.clone(),
            extra: extra_payload,
        };
        
        let body_json = serde_json::to_string(&request_body)
            .context("Failed to serialize request body to JSON")?;
        debug!(%body_json, "Constructed request body");

        Ok(BuiltRequestParams {
            // Use self.api_endpoint
            url: self.api_endpoint.clone(),
            headers,
            body: body_json,
        })
    }

    #[instrument(skip(self, content, options), fields(content_len = content.len()))]
    pub async fn send_request(
        &self,
        content: &str,
        options: DevRequestOptions, 
    ) -> Result<Response> {
        debug!("Preparing to send request to Dev API...");

        // 1. Build parameters
        let params = self.build_request_params(content, &options)
            .context("Failed to build request parameters")?;

        // 2. Build reqwest request
        let request = self.client
            .post(&params.url)
            .headers(params.headers)
            .body(params.body)
            .build()
            .context("Failed to build reqwest POST request")?;

        debug!(url = %params.url, "Sending request...");

        // 3. Send request and get response
        let response = self.client.execute(request).await
            .context("Failed to execute request to Dev API")?;

        debug!(status = %response.status(), "Received response status");

        // Check status: If not success, consume response to get error and return Err
        if !response.status().is_success() {
             let status = response.status();
             let error_body = response.text().await
                .unwrap_or_else(|e| format!("Failed to read error body: {}", e));
             error!(%status, error_body, "Dev API returned non-success status");
             return Err(anyhow!("Dev API Error ({}): {}", status, error_body)); // Return Err directly
        }
        
        // If success, return the response
        info!("Dev API request successful, returning response.");
        Ok(response)
    }

    /// Send a streaming request to the Dev API
    /// Returns a stream of bytes that can be processed by the SSE processor
    pub async fn send_stream_request(
        &self,
        content: &str,
        options: DevRequestOptions,
    ) -> anyhow::Result<impl Stream<Item = Result<Bytes, reqwest::Error>>> {
        // Build request parameters using the same logic as send_request
        let params = self.build_request_params(content, &options)?;

        // Send the request and expect a streaming response
        let response = self.client
            .post(&params.url)
            .headers(params.headers)
            .header("Accept", "text/event-stream")
            .body(params.body)
            .send()
            .await?;

        // Return the byte stream
        Ok(response.bytes_stream())
    }
} 