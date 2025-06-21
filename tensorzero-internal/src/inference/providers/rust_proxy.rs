//! # RustProxyProvider
//! 
//! This module implements a TensorZero provider that integrates with the DevV AI service
//! via the rust-proxy-core crate. It provides comprehensive model management capabilities
//! including dynamic model discovery, validation, configuration mapping, and state management.
//! 
//! ## Key Features
//! 
//! - **Dynamic Model Discovery**: Automatically fetches available models from DevV API
//! - **Model Validation**: Validates model availability and usage limits before inference
//! - **Configuration Mapping**: Maps TensorZero model names to DevV API model identifiers
//! - **State Management**: Tracks model types (base/freeTrial/premium) and usage limits
//! - **Caching**: Implements intelligent caching to reduce API calls and improve performance
//! 
//! ## Architecture
//! 
//! The provider is built around the `RustProxyProvider` struct which wraps the `DevApiClient`
//! from rust-proxy-core and adds TensorZero-specific functionality. It implements the
//! `InferenceProvider` trait to integrate seamlessly with TensorZero's inference pipeline.
//! 
//! ## Usage Example
//! 
//! ```rust,ignore
//! let provider = RustProxyProvider::new(
//!     "claude-3.5-sonnet".to_string(),
//!     None, // api_key_location
//!     None, // device_id  
//!     None, // session_id
//! )?;
//! 
//! // Discover available models
//! let models = provider.discover_models().await?;
//! 
//! // Validate a model
//! let is_valid = provider.validate_model("claude-3.5-sonnet").await?;
//! ```

use std::time::Instant;
use std::sync::Arc;
use std::collections::HashMap;

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
use rust_proxy_core::dev_client::{DevApiClient, DevRequestOptions, ModelInfo};
use rust_proxy_core::sse_processor::ChatCompletionChunk;
use tokio::sync::RwLock;
use axum::http::StatusCode;

const PROVIDER_TYPE: &str = "RustProxy";

/// The main provider struct that integrates DevV AI service with TensorZero.
/// 
/// This provider handles all aspects of model management and inference requests
/// for the DevV AI service, including dynamic model discovery, validation,
/// and intelligent caching.
/// 
/// ## Thread Safety
/// 
/// The provider is designed to be thread-safe and can be safely shared across
/// multiple async tasks. Model caches use `Arc<RwLock<>>` for concurrent access.
/// 
/// ## Caching Strategy
/// 
/// - **Model Discovery Cache**: Caches the list of available models to reduce API calls
/// - **Validation Cache**: Caches model validation results for better performance
/// - **Cache Invalidation**: Provides methods to clear caches when needed
pub struct RustProxyProvider {
    /// The underlying DevV API client for making requests
    client: DevApiClient,
    /// The model name configured for this provider instance
    model_name: String,
    /// Cache for available models from the DevV API
    /// 
    /// This cache stores the result of the models API endpoint to avoid
    /// repeated requests. It's wrapped in Arc<RwLock<>> for thread-safe access.
    available_models: Arc<RwLock<Option<Vec<ModelInfo>>>>,
    /// Cache for model validation results
    /// 
    /// Maps model names to their validation status (true if available and usable).
    /// This cache is used to avoid repeated validation checks for the same models.
    validated_models: Arc<RwLock<HashMap<String, bool>>>,
    /// Optional model filtering configuration
    /// 
    /// When provided, this filter is applied to the discovered models to restrict
    /// which models are considered available based on type, usage limits, etc.
    model_filter: Option<crate::model::RustProxyModelFilter>,
}

impl std::fmt::Debug for RustProxyProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RustProxyProvider")
            .field("client", &"<DevApiClient>")
            .field("model_name", &self.model_name)
            .finish()
    }
}

impl RustProxyProvider {
    /// Creates a new RustProxyProvider instance.
    /// 
    /// # Arguments
    /// 
    /// * `model_name` - The name of the model to use for inference requests
    /// * `_api_key_location` - Optional API key location (currently unused)
    /// * `device_id` - Optional device identifier for requests
    /// * `session_id` - Optional session identifier for requests  
    /// * `api_endpoint` - Optional API endpoint URL (defaults to hardcoded value)
    /// * `os_type` - Optional OS type identifier (defaults to "3")
    /// * `accept_language` - Optional language preference (defaults to "en")
    /// * `model_filter` - Optional model filtering configuration
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<Self, Error>` with the configured provider or an error
    /// if the DevApiClient cannot be created.
    /// 
    /// # Example
    /// 
    /// ```rust,ignore
    /// let provider = RustProxyProvider::new(
    ///     "claude-3.5-sonnet".to_string(),
    ///     None,
    ///     Some("device123".to_string()),
    ///     Some("session456".to_string()),
    ///     Some("https://api.devv.ai/api/v1/stream/chat".to_string()),
    ///     Some("3".to_string()),
    ///     Some("en".to_string()),
    ///     None,
    /// )?;
    /// ```
    pub fn new(
        model_name: String,
        _api_key_location: Option<crate::model::CredentialLocation>,
        device_id: Option<String>,
        session_id: Option<String>,
        api_endpoint: Option<String>,
        os_type: Option<String>,
        accept_language: Option<String>,
        model_filter: Option<crate::model::RustProxyModelFilter>,
    ) -> Result<Self, Error> {
        // Use provided configuration or defaults
        let api_endpoint = api_endpoint.unwrap_or_else(|| 
            "https://api.devv.ai/api/v1/stream/chat".to_string()
        );
        let device_id = device_id.unwrap_or_else(|| 
            "default-device-id".to_string()
        );
        let os_type = os_type.unwrap_or_else(|| "3".to_string());
        let session_id = session_id.unwrap_or_else(|| 
            "default-session-id".to_string()
        );

        let client = DevApiClient::with_config(api_endpoint, device_id, os_type, session_id)
            .map_err(|e| Error::new(ErrorDetails::InferenceClient { 
                message: format!("Failed to create DevApiClient: {}", e),
                status_code: None,
                provider_type: PROVIDER_TYPE.to_string(),
                raw_request: None,
                raw_response: None,
            }))?;
        
        Ok(Self { 
            client, 
            model_name,
            available_models: Arc::new(RwLock::new(None)),
            validated_models: Arc::new(RwLock::new(HashMap::new())),
            model_filter: model_filter,
        })
    }

    /// Returns the configured model name for this provider instance.
    /// 
    /// This is the original model name as configured by the user, before any
    /// mapping to DevV API model identifiers.
    pub fn model_name(&self) -> &str {
        &self.model_name
    }

    /// Discovers and retrieves available models from the DevV API.
    /// 
    /// This method first checks the local cache for available models. If the cache
    /// is empty, it makes a request to the DevV API to fetch the current list of
    /// available models, applies any configured filters, and caches the result.
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<Vec<ModelInfo>, Error>` containing the list of available
    /// models (after filtering) or an error if the API request fails.
    /// 
    /// # Caching Behavior
    /// 
    /// - First call: Fetches from API, applies filters, and caches the result
    /// - Subsequent calls: Returns cached data
    /// - Cache can be cleared using `clear_model_cache()`
    /// 
    /// # Filtering
    /// 
    /// If a `model_filter` is configured, only models matching the filter criteria
    /// will be returned and cached.
    /// 
    /// # Example
    /// 
    /// ```rust,ignore
    /// let models = provider.discover_models().await?;
    /// for model in models {
    ///     println!("Model: {} (Type: {}, Usage: {})", 
    ///              model.model_name, model.model_type, model.usage_left);
    /// }
    /// ```
    pub async fn discover_models(&self) -> Result<Vec<ModelInfo>, Error> {
        // Check if we already have cached models
        {
            let cached_models = self.available_models.read().await;
            if let Some(ref models) = *cached_models {
                return Ok(models.clone());
            }
        }

        // Fetch models from the API
        let raw_models = self.client
            .get_models()
            .await
            .map_err(|e| Error::new(ErrorDetails::InferenceClient { 
                message: format!("Failed to discover models: {}", e),
                status_code: None,
                provider_type: PROVIDER_TYPE.to_string(),
                raw_request: None,
                raw_response: None,
            }))?;

        // Apply filters if configured
        let filtered_models = self.apply_model_filter(&raw_models);

        // Cache the filtered models
        {
            let mut cached_models = self.available_models.write().await;
            *cached_models = Some(filtered_models.clone());
        }

        Ok(filtered_models)
    }

    /// Validates if a specific model is available for use.
    /// 
    /// This method checks if the specified model exists in the list of available
    /// models and has sufficient usage quota. The validation logic differs based
    /// on model type:
    /// 
    /// - **Base models**: Always considered available
    /// - **FreeTrial/Premium models**: Must have `usage_left > 0`
    /// 
    /// # Arguments
    /// 
    /// * `model_name` - The name of the model to validate
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<bool, Error>` where `true` indicates the model is
    /// available and usable, `false` indicates it's not available or has no
    /// usage quota remaining.
    /// 
    /// # Caching Behavior
    /// 
    /// Validation results are cached to improve performance. The cache is
    /// automatically cleared when `clear_model_cache()` is called.
    /// 
    /// # Example
    /// 
    /// ```rust,ignore
    /// if provider.validate_model("claude-3.5-sonnet").await? {
    ///     println!("Model is available for use");
    /// } else {
    ///     println!("Model is not available or has no usage left");
    /// }
    /// ```
    pub async fn validate_model(&self, model_name: &str) -> Result<bool, Error> {
        // Check validation cache first
        {
            let validated_cache = self.validated_models.read().await;
            if let Some(&is_valid) = validated_cache.get(model_name) {
                return Ok(is_valid);
            }
        }

        // Discover models to validate against
        let available_models = self.discover_models().await?;
        
        // Check if the model exists and has usage left
        let is_valid = available_models.iter().any(|model| {
            model.model_name == model_name && 
            (model.model_type == "base" || model.usage_left > 0)
        });

        // Cache the validation result
        {
            let mut validated_cache = self.validated_models.write().await;
            validated_cache.insert(model_name.to_string(), is_valid);
        }

        Ok(is_valid)
    }

    /// Retrieves detailed information for a specific model.
    /// 
    /// # Arguments
    /// 
    /// * `model_name` - The name of the model to get information for
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<Option<ModelInfo>, Error>` where `Some(ModelInfo)`
    /// contains the model details if found, or `None` if the model doesn't exist.
    /// 
    /// # Example
    /// 
    /// ```rust,ignore
    /// if let Some(model_info) = provider.get_model_info("claude-3.5-sonnet").await? {
    ///     println!("Model: {} has {} usage left", 
    ///              model_info.display_name, model_info.usage_left);
    /// }
    /// ```
    pub async fn get_model_info(&self, model_name: &str) -> Result<Option<ModelInfo>, Error> {
        let available_models = self.discover_models().await?;
        Ok(available_models.into_iter().find(|m| m.model_name == model_name))
    }

    /// Clears all cached model data.
    /// 
    /// This method clears both the available models cache and the model validation
    /// cache. It's useful when you want to force a refresh of model information
    /// from the API, for example after model usage or when models may have been
    /// added/removed.
    /// 
    /// # Example
    /// 
    /// ```rust,ignore
    /// // Clear cache to get fresh model data
    /// provider.clear_model_cache().await;
    /// let fresh_models = provider.discover_models().await?;
    /// ```
    pub async fn clear_model_cache(&self) {
        let mut cached_models = self.available_models.write().await;
        *cached_models = None;
        
        let mut validated_cache = self.validated_models.write().await;
        validated_cache.clear();
    }

    /// Applies the configured model filter to a list of models.
    /// 
    /// This method filters the provided models based on the configured
    /// `model_filter` settings. If no filter is configured, returns all models.
    /// 
    /// # Arguments
    /// 
    /// * `models` - The list of models to filter
    /// 
    /// # Returns
    /// 
    /// Returns a filtered list of models that match the filter criteria.
    /// 
    /// # Filter Criteria
    /// 
    /// - **include_types**: Only models with types in this list are included
    /// - **exclude_types**: Models with types in this list are excluded
    /// - **min_usage_left**: Only models with usage_left >= this value
    /// - **only_new**: If true, only models with is_new = true are included
    /// 
    /// # Example
    /// 
    /// ```rust,ignore
    /// let all_models = provider.discover_models().await?;
    /// let filtered_models = provider.apply_model_filter(&all_models);
    /// ```
    fn apply_model_filter(&self, models: &[ModelInfo]) -> Vec<ModelInfo> {
        let Some(ref filter) = self.model_filter else {
            return models.to_vec();
        };

        models.iter()
            .filter(|model| {
                // Check include_types filter
                if let Some(ref include_types) = filter.include_types {
                    if !include_types.contains(&model.model_type) {
                        return false;
                    }
                }

                // Check exclude_types filter
                if let Some(ref exclude_types) = filter.exclude_types {
                    if exclude_types.contains(&model.model_type) {
                        return false;
                    }
                }

                // Check min_usage_left filter
                if let Some(min_usage) = filter.min_usage_left {
                    if model.usage_left < min_usage {
                        return false;
                    }
                }

                // Check only_new filter
                if let Some(only_new) = filter.only_new {
                    if only_new && !model.is_new {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect()
    }

    /// Maps TensorZero model names to DevV API model identifiers.
    /// 
    /// This method provides a flexible mapping system that allows users to configure
    /// their TensorZero models using friendly names while automatically translating
    /// them to the specific model identifiers required by the DevV API.
    /// 
    /// # Arguments
    /// 
    /// * `tensorzero_model_name` - The model name as configured in TensorZero
    /// 
    /// # Returns
    /// 
    /// Returns the corresponding DevV API model identifier. If no mapping is found,
    /// returns the original name unchanged, allowing for direct specification of
    /// DevV API model names.
    /// 
    /// # Supported Mappings
    /// 
    /// - **Claude models**: `claude-3.5-sonnet` → `us.anthropic.claude-3-7-sonnet-20250219-v1:0`
    /// - **GPT models**: `gpt-4.1` → `gpt-4.1`
    /// - **Gemini models**: `gemini-2.0-flash` → `gemini-2.0-flash-001`
    /// - **OpenAI o3**: `o3` → `o3`
    /// 
    /// # Example
    /// 
    /// ```rust,ignore
    /// let mapped_name = provider.map_model_name("claude-3.5-sonnet");
    /// // Returns: "us.anthropic.claude-3-7-sonnet-20250219-v1:0"
    /// ```
    pub fn map_model_name(&self, tensorzero_model_name: &str) -> String {
        // Common model name mappings from TensorZero conventions to DevV API
        match tensorzero_model_name.to_lowercase().as_str() {
            // Claude models
            "claude-3.5-sonnet" | "claude-3-5-sonnet" => "us.anthropic.claude-3-7-sonnet-20250219-v1:0".to_string(),
            "claude-3.5-sonnet-thinking" | "claude-3-5-sonnet-thinking" => "us.anthropic.claude-3-7-sonnet-20250219-v1:0-thinking".to_string(),
            "claude-sonnet-4" | "claude-4-sonnet" => "us.anthropic.claude-sonnet-4-20250514-v1:0".to_string(),
            "claude-opus-4" | "claude-4-opus" => "us.anthropic.claude-opus-4-20250514-v1:0".to_string(),
            
            // GPT models  
            "gpt-4.1" | "gpt-4-1" => "gpt-4.1".to_string(),
            "gpt-4.1-mini" | "gpt-4-1-mini" => "gpt-4.1-mini".to_string(),
            
            // Gemini models
            "gemini-2.0-flash" | "gemini-2-flash" => "gemini-2.0-flash-001".to_string(),
            "gemini-1.5-pro" | "gemini-1-5-pro" => "gemini-1.5-pro-002".to_string(),
            
            // OpenAI o3 model
            "o3" => "o3".to_string(),
            
            // If no mapping is found, return the original name
            // This allows for direct specification of DevV API model names
            _ => tensorzero_model_name.to_string(),
        }
    }

    /// Gets the effective model name for DevV API requests.
    /// 
    /// This method applies the model name mapping configured in `map_model_name()`
    /// and returns the actual model identifier that should be used for API requests.
    /// 
    /// # Returns
    /// 
    /// Returns the mapped model name that corresponds to the configured model.
    /// 
    /// # Example
    /// 
    /// ```rust,ignore
    /// let provider = RustProxyProvider::new("claude-3.5-sonnet".to_string(), None, None, None)?;
    /// let effective_name = provider.get_effective_model_name();
    /// // Returns: "us.anthropic.claude-3-7-sonnet-20250219-v1:0"
    /// ```
    pub fn get_effective_model_name(&self) -> String {
        self.map_model_name(&self.model_name)
    }

    /// Validates that a mapped model name is available.
    /// 
    /// This method first applies model name mapping, then validates the resulting
    /// model identifier against the available models from the DevV API.
    /// 
    /// # Arguments
    /// 
    /// * `tensorzero_model_name` - The TensorZero model name to map and validate
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<bool, Error>` indicating whether the mapped model is
    /// available and usable.
    /// 
    /// # Example
    /// 
    /// ```rust,ignore
    /// let is_available = provider.validate_mapped_model("claude-3.5-sonnet").await?;
    /// ```
    pub async fn validate_mapped_model(&self, tensorzero_model_name: &str) -> Result<bool, Error> {
        let effective_model_name = self.map_model_name(tensorzero_model_name);
        self.validate_model(&effective_model_name).await
    }

    /// Gets model information for a mapped model.
    /// 
    /// This method first applies model name mapping, then retrieves detailed
    /// information for the resulting model identifier.
    /// 
    /// # Arguments
    /// 
    /// * `tensorzero_model_name` - The TensorZero model name to map and query
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<Option<ModelInfo>, Error>` with the model information
    /// if found, or `None` if the mapped model doesn't exist.
    /// 
    /// # Example
    /// 
    /// ```rust,ignore
    /// if let Some(info) = provider.get_mapped_model_info("claude-3.5-sonnet").await? {
    ///     println!("Mapped model has {} usage left", info.usage_left);
    /// }
    /// ```
    pub async fn get_mapped_model_info(&self, tensorzero_model_name: &str) -> Result<Option<ModelInfo>, Error> {
        let effective_model_name = self.map_model_name(tensorzero_model_name);
        self.get_model_info(&effective_model_name).await
    }

    /// Checks detailed availability status for a model.
    /// 
    /// This method provides comprehensive availability information including
    /// model type, usage limits, and specific reasons for unavailability.
    /// It's more detailed than the basic `validate_model()` method.
    /// 
    /// # Arguments
    /// 
    /// * `model_name` - The model name to check availability for
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<ModelAvailability, Error>` with detailed availability
    /// information including:
    /// 
    /// - `Available`: Model is usable, with type and usage information
    /// - `Unavailable`: Model cannot be used, with specific reason
    /// 
    /// # Model Type Behavior
    /// 
    /// - **Base models**: Always available regardless of usage_left
    /// - **FreeTrial models**: Available only if usage_left > 0
    /// - **Premium models**: Available only if usage_left > 0
    /// - **Unknown types**: Always considered unavailable
    /// 
    /// # Example
    /// 
    /// ```rust,ignore
    /// match provider.check_model_availability("claude-3.5-sonnet").await? {
    ///     ModelAvailability::Available { model_type, usage_left, .. } => {
    ///         println!("Model available: type {:?}, usage {:?}", model_type, usage_left);
    ///     },
    ///     ModelAvailability::Unavailable { reason, .. } => {
    ///         println!("Model unavailable: {:?}", reason);
    ///     }
    /// }
    /// ```
    pub async fn check_model_availability(&self, model_name: &str) -> Result<ModelAvailability, Error> {
        let models = self.discover_models().await?;
        
        match models.iter().find(|m| m.model_name == model_name) {
            Some(model_info) => {
                let availability = match model_info.model_type.as_str() {
                    "base" => {
                        // Base models are always available
                        ModelAvailability::Available { 
                            model_type: ModelType::Base,
                            usage_left: None,
                            is_premium: false,
                        }
                    },
                    "freeTrial" => {
                        if model_info.usage_left > 0 {
                            ModelAvailability::Available { 
                                model_type: ModelType::FreeTrial,
                                usage_left: Some(model_info.usage_left),
                                is_premium: false,
                            }
                        } else {
                            ModelAvailability::Unavailable { 
                                reason: UnavailabilityReason::UsageLimitExceeded,
                                model_type: ModelType::FreeTrial,
                            }
                        }
                    },
                    "premium" => {
                        if model_info.usage_left > 0 {
                            ModelAvailability::Available { 
                                model_type: ModelType::Premium,
                                usage_left: Some(model_info.usage_left),
                                is_premium: true,
                            }
                        } else {
                            ModelAvailability::Unavailable { 
                                reason: UnavailabilityReason::UsageLimitExceeded,
                                model_type: ModelType::Premium,
                            }
                        }
                    },
                    _ => {
                        ModelAvailability::Unavailable { 
                            reason: UnavailabilityReason::UnknownModelType,
                            model_type: ModelType::Unknown,
                        }
                    }
                };
                Ok(availability)
            },
            None => Ok(ModelAvailability::Unavailable { 
                reason: UnavailabilityReason::ModelNotFound,
                model_type: ModelType::Unknown,
            }),
        }
    }

    /// Updates model usage information after a successful inference request.
    /// 
    /// This method is called automatically after successful inference requests
    /// to update the local cache with current usage information. Currently,
    /// it clears the model cache to ensure fresh data on the next validation.
    /// 
    /// # Arguments
    /// 
    /// * `_model_name` - The name of the model that was used (currently unused)
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<(), Error>` indicating success or failure of the update.
    /// 
    /// # Implementation Notes
    /// 
    /// The current implementation simply clears the cache to force a refresh.
    /// Future enhancements could include:
    /// 
    /// - Making a real-time usage check API call
    /// - Decrementing cached usage_left values locally
    /// - Triggering alerts when usage is low
    /// - Batching usage updates for efficiency
    /// 
    /// # Example
    /// 
    /// ```rust,ignore
    /// // Called automatically after successful inference
    /// provider.update_model_usage("claude-3.5-sonnet").await?;
    /// ```
    pub async fn update_model_usage(&self, _model_name: &str) -> Result<(), Error> {
        // For now, we'll clear the cache to force a refresh on next validation
        // In a more sophisticated implementation, we could decrement the cached usage_left
        self.clear_model_cache().await;
        
        // In the future, this could also:
        // 1. Make a request to check current usage
        // 2. Update local cache with new usage information
        // 3. Trigger alerts if usage is low
        
        Ok(())
    }

    /// Gets a comprehensive summary of all available models grouped by type.
    /// 
    /// This method provides an organized view of all available models,
    /// categorized by their type (base, freeTrial, premium, unknown).
    /// It's useful for understanding the overall model landscape and
    /// making informed decisions about model usage.
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<ModelsSummary, Error>` containing:
    /// 
    /// - `base_models`: Models that are always available
    /// - `free_trial_models`: Models with limited free usage
    /// - `premium_models`: Models requiring payment/subscription
    /// - `unknown_models`: Models with unrecognized types
    /// - `total_count`: Total number of available models
    /// 
    /// # Example
    /// 
    /// ```rust,ignore
    /// let summary = provider.get_models_summary().await?;
    /// println!("Available models: {} total", summary.total_count);
    /// println!("  Base models: {}", summary.base_models.len());
    /// println!("  Free trial models: {}", summary.free_trial_models.len());
    /// println!("  Premium models: {}", summary.premium_models.len());
    /// ```
    pub async fn get_models_summary(&self) -> Result<ModelsSummary, Error> {
        let models = self.discover_models().await?;
        
        let mut base_models = Vec::new();
        let mut free_trial_models = Vec::new();
        let mut premium_models = Vec::new();
        let mut unknown_models = Vec::new();
        
        for model in models {
            match model.model_type.as_str() {
                "base" => base_models.push(model),
                "freeTrial" => free_trial_models.push(model),
                "premium" => premium_models.push(model),
                _ => unknown_models.push(model),
            }
        }
        
        let total_count = base_models.len() + free_trial_models.len() + premium_models.len() + unknown_models.len();
        
        Ok(ModelsSummary {
            base_models,
            free_trial_models,
            premium_models,
            unknown_models,
            total_count,
        })
    }

    /// Converts TensorZero messages to a simple string format for DevV API.
    /// 
    /// This helper method transforms the structured TensorZero message format
    /// into a simple string representation that can be sent to the DevV API.
    /// Each message is formatted as "Role: Content" and multiple messages
    /// are joined with newlines.
    /// 
    /// # Arguments
    /// 
    /// * `messages` - Array of TensorZero RequestMessage structs
    /// 
    /// # Returns
    /// 
    /// Returns a formatted string representing the conversation, with each
    /// message prefixed by its role (User/Assistant).
    /// 
    /// # Message Processing
    /// 
    /// - **Text content**: Extracted and included in full
    /// - **Non-text content**: Replaced with "[Non-text content]" placeholder
    /// - **Multiple content blocks**: Joined with spaces
    /// 
    /// # Example Output
    /// 
    /// ```text
    /// User: Hello, how are you?
    /// Assistant: I'm doing well, thank you!
    /// User: Can you help me with Rust?
    /// ```
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

    /// Builds DevV API request options from a TensorZero inference request.
    /// 
    /// This helper method creates the `DevRequestOptions` struct required by
    /// the DevV API, using the configured model name and default settings
    /// optimized for general-purpose inference.
    /// 
    /// # Arguments
    /// 
    /// * `_request` - The TensorZero ModelInferenceRequest (currently unused)
    /// 
    /// # Returns
    /// 
    /// Returns a `DevRequestOptions` struct configured with:
    /// 
    /// - **model**: The effective (mapped) model name
    /// - **search_mode**: Set to "web" for enhanced responses
    /// - **is_expert**: Set to false for general use
    /// - **language**: Set to "en" for English responses
    /// 
    /// # Future Enhancements
    /// 
    /// This method could be enhanced to use request parameters such as:
    /// - Temperature and other generation parameters
    /// - Custom search modes based on request type
    /// - Language detection from request content
    /// - Expert mode based on model capabilities
    fn build_request_options(&self, _request: &ModelInferenceRequest) -> DevRequestOptions {
        // Use the mapped model name for the actual API request
        let effective_model_name = self.get_effective_model_name();
        
        DevRequestOptions {
            model: Some(effective_model_name),
            search_mode: Some("web".to_string()),
            is_expert: Some(false),
            language: Some("en".to_string()),
            ..Default::default()
        }
    }

    /// Converts streaming response chunks to TensorZero format.
    /// 
    /// This helper method transforms SSE `ChatCompletionChunk` data from the
    /// DevV API into TensorZero's `ProviderInferenceResponseChunk` format
    /// for streaming responses.
    /// 
    /// # Arguments
    /// 
    /// * `chunk` - The ChatCompletionChunk from the DevV API
    /// * `chunk_id` - Unique identifier for this chunk
    /// * `latency` - Time elapsed since the request started
    /// 
    /// # Returns
    /// 
    /// Returns a `Result<ProviderInferenceResponseChunk, Error>` with the
    /// formatted chunk data or an error if the chunk is invalid.
    /// 
    /// # Chunk Processing
    /// 
    /// - Extracts text content from the first choice in the chunk
    /// - Maps finish reasons to TensorZero's FinishReason enum
    /// - Creates appropriate ContentBlockChunk entries
    /// - Preserves raw response data for debugging
    /// 
    /// # Error Conditions
    /// 
    /// Returns an error if the chunk contains no choices, which indicates
    /// an invalid or malformed response from the API.
    /// 
    /// # Note
    /// 
    /// This method is currently used by the streaming infrastructure but
    /// streaming support is temporarily disabled due to lifetime complexity.
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
            
            // Validate the model before processing the request
            // Use the mapped model name for validation
            let effective_model_name = self.get_effective_model_name();
            if !self.validate_model(&effective_model_name).await? {
                return Err(Error::new(ErrorDetails::InferenceClient { 
                    message: format!("Model '{}' (mapped to '{}') is not available or has no usage left", 
                                   self.model_name, effective_model_name),
                    status_code: Some(StatusCode::BAD_REQUEST),
                    provider_type: PROVIDER_TYPE.to_string(),
                    raw_request: None,
                    raw_response: None,
                }));
            }
            
            let content = Self::messages_to_content(&request.request.messages);
            let options = self.build_request_options(&request.request);
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

            // Update model usage after successful inference
            if let Err(e) = self.update_model_usage(&effective_model_name).await {
                // Log the error but don't fail the inference
                tracing::warn!("Failed to update model usage for '{}': {}", effective_model_name, e);
            }

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

// Model state management types and enums

/// Represents the different types of models available in the DevV API.
/// 
/// Each model type has different availability rules and usage restrictions:
/// 
/// - **Base**: Always available, no usage limits
/// - **FreeTrial**: Limited usage, requires checking usage_left
/// - **Premium**: Paid models, requires checking usage_left  
/// - **Unknown**: Unrecognized model types, treated as unavailable
#[derive(Debug, Clone, PartialEq)]
pub enum ModelType {
    /// Base models that are always available without usage restrictions
    Base,
    /// Free trial models with limited usage quotas
    FreeTrial,
    /// Premium models requiring payment or subscription
    Premium,
    /// Models with unrecognized or unsupported types
    Unknown,
}

/// Specific reasons why a model might be unavailable for use.
/// 
/// This enum provides detailed information about why a model cannot
/// be used, enabling better error messages and user guidance.
#[derive(Debug, Clone, PartialEq)]
pub enum UnavailabilityReason {
    /// The requested model was not found in the available models list
    ModelNotFound,
    /// The model exists but has no remaining usage quota
    UsageLimitExceeded,
    /// The model has an unknown or unsupported type
    UnknownModelType,
}

/// Detailed availability status for a specific model.
/// 
/// This enum provides comprehensive information about whether a model
/// can be used and includes relevant metadata for decision making.
/// 
/// # Usage Example
/// 
/// ```rust,ignore
/// match availability {
///     ModelAvailability::Available { model_type, usage_left, is_premium } => {
///         if let Some(usage) = usage_left {
///             println!("Model available with {} uses remaining", usage);
///         } else {
///             println!("Model available with unlimited usage");
///         }
///     },
///     ModelAvailability::Unavailable { reason, .. } => {
///         match reason {
///             UnavailabilityReason::UsageLimitExceeded => {
///                 println!("Model quota exhausted, please upgrade or wait");
///             },
///             UnavailabilityReason::ModelNotFound => {
///                 println!("Model not found, check model name");
///             },
///             UnavailabilityReason::UnknownModelType => {
///                 println!("Unsupported model type");
///             }
///         }
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum ModelAvailability {
    /// Model is available and can be used for inference
    Available {
        /// The type classification of this model
        model_type: ModelType,
        /// Number of uses remaining (None for unlimited)
        usage_left: Option<u32>,
        /// Whether this is a premium/paid model
        is_premium: bool,
    },
    /// Model is not available for use
    Unavailable {
        /// Specific reason for unavailability
        reason: UnavailabilityReason,
        /// The type classification of this model
        model_type: ModelType,
    },
}

/// Comprehensive summary of all available models organized by type.
/// 
/// This struct provides an organized view of the model landscape,
/// making it easy to understand what models are available and
/// make informed decisions about model usage.
/// 
/// # Usage Example
/// 
/// ```rust,ignore
/// let summary = provider.get_models_summary().await?;
/// 
/// println!("Model Summary:");
/// println!("  Total models: {}", summary.total_count);
/// println!("  Base models: {}", summary.base_models.len());
/// println!("  Free trial models: {}", summary.free_trial_models.len());
/// println!("  Premium models: {}", summary.premium_models.len());
/// 
/// if !summary.unknown_models.is_empty() {
///     println!("  Unknown types: {}", summary.unknown_models.len());
/// }
/// ```
#[derive(Debug, Clone)]
pub struct ModelsSummary {
    /// Models that are always available without restrictions
    pub base_models: Vec<ModelInfo>,
    /// Models with limited free trial usage
    pub free_trial_models: Vec<ModelInfo>,
    /// Models requiring payment or subscription
    pub premium_models: Vec<ModelInfo>,
    /// Models with unrecognized or unsupported types
    pub unknown_models: Vec<ModelInfo>,
    /// Total count of all available models
    pub total_count: usize,
} 