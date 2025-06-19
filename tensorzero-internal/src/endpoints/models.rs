use axum::extract::{Path, State};
use axum::response::Json;
use serde::{Deserialize, Serialize};

use crate::gateway_util::AppState;

#[derive(Debug, Deserialize, Serialize)]
pub struct Model {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub owned_by: String,
    #[serde(default)]
    pub permission: Vec<Permission>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Permission {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub allow_create_engine: bool,
    pub allow_sampling: bool,
    pub allow_logprobs: bool,
    pub allow_search_indices: bool,
    pub allow_view: bool,
    pub allow_fine_tuning: bool,
    pub organization: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default)]
    pub is_blocking: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListModelsResponse {
    pub object: String,
    pub data: Vec<Model>,
}

/// Handler for listing available models
pub async fn list_models_handler(
    State(app_state): AppState,
) -> Json<ListModelsResponse> {
    let mut models = Vec::new();
    
    // Add all configured models from the model table
    for (model_name, _model_config) in app_state.config.models.iter_static_models() {
        models.push(Model {
            id: model_name.to_string(),
            object: "model".to_string(),
            created: 1677610602, // Static timestamp for consistency
            owned_by: "tensorzero".to_string(),
            permission: vec![],
        });
    }
    
    Json(ListModelsResponse {
        object: "list".to_string(),
        data: models,
    })
}

/// Handler for retrieving a specific model
pub async fn get_model_handler(
    State(app_state): AppState,
    Path(model_id): Path<String>,
) -> Result<Json<Model>, axum::http::StatusCode> {
    // Check if the model exists in the configuration
    match app_state.config.models.validate(&model_id) {
        Ok(_) => {
            // Model exists, return it
            Ok(Json(Model {
                id: model_id,
                object: "model".to_string(),
                created: 1677610602, // Static timestamp for consistency
                owned_by: "tensorzero".to_string(),
                permission: vec![],
            }))
        }
        Err(_) => {
            // Model not found
            Err(axum::http::StatusCode::NOT_FOUND)
        }
    }
} 