use axum::extract::State;
use axum::response::Json;
use serde::{Deserialize, Serialize};

use crate::embeddings::EmbeddingRequest as InternalEmbeddingRequest;
use crate::endpoints::inference::InferenceClients;
use crate::error::{Error, ErrorDetails};
use crate::gateway_util::AppState;
use crate::endpoints::inference::InferenceCredentials;
use crate::cache::{CacheOptions, CacheEnabledMode};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum EmbeddingsInput {
    String(String),
    StringArray(Vec<String>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmbeddingsRequest {
    pub input: EmbeddingsInput,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Embedding {
    pub object: String,
    pub embedding: Vec<f32>,
    pub index: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateEmbeddingsResponse {
    pub object: String,
    pub model: String,
    pub data: Vec<Embedding>,
}

/// Handler for creating embeddings
pub async fn create_embeddings_handler(
    State(app_state): AppState,
    Json(req): Json<EmbeddingsRequest>,
) -> Result<Json<CreateEmbeddingsResponse>, Error> {
    // Convert the request inputs to a vector of strings
    let inputs = match req.input {
        EmbeddingsInput::String(s) => vec![s],
        EmbeddingsInput::StringArray(arr) => arr,
    };

    // Check if the embedding model exists
    let embedding_model = app_state.config.embedding_models.get(&req.model).await?
        .ok_or_else(|| Error::new(ErrorDetails::UnknownModel {
            name: req.model.clone(),
        }))?;

    // Set up inference clients
    let inference_clients = InferenceClients {
        http_client: &app_state.http_client,
        clickhouse_connection_info: &app_state.clickhouse_connection_info,
        credentials: &InferenceCredentials::default(), // Use default credentials for now
        cache_options: &CacheOptions {
            max_age_s: None,
            enabled: CacheEnabledMode::WriteOnly,
        },
    };

    // Process each input and generate embeddings
    let mut embeddings = Vec::new();
    for (index, input_text) in inputs.into_iter().enumerate() {
        let internal_request = InternalEmbeddingRequest {
            input: input_text,
        };

        let embedding_response = embedding_model
            .embed(&internal_request, &req.model, &inference_clients)
            .await?;

        embeddings.push(Embedding {
            object: "embedding".to_string(),
            embedding: embedding_response.embedding,
            index: index as u32,
        });
    }

    Ok(Json(CreateEmbeddingsResponse {
        object: "list".to_string(),
        model: req.model,
        data: embeddings,
    }))
} 