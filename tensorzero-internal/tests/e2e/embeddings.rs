#![expect(clippy::print_stdout)]

use axum::{extract::State, Json};
use serde_json::{json, Value};
use tracing_test::traced_test;

use crate::providers::common::make_embedded_gateway;
use tensorzero_internal::endpoints::embeddings::EmbeddingsRequest;

#[tokio::test(flavor = "multi_thread")]
#[traced_test]
async fn test_embeddings_single_string_input() {
    let client = make_embedded_gateway().await;
    let state = client.get_app_state_data().unwrap().clone();

    let request: EmbeddingsRequest = serde_json::from_value(json!({
        "model": "text-embedding-3-small",
        "input": "Hello, world!"
    }))
    .unwrap();

    let response = tensorzero_internal::endpoints::embeddings::create_embeddings_handler(
        State(state),
        Json(request),
    )
    .await
    .unwrap();

    // Convert Json response to Value for testing
    let response_json: Value = serde_json::to_value(&response.0).unwrap();
    println!("Single string response: {response_json:?}");

    // Validate response structure
    assert_eq!(response_json.get("object").unwrap().as_str().unwrap(), "list");
    assert_eq!(response_json.get("model").unwrap().as_str().unwrap(), "text-embedding-3-small");
    
    let data = response_json.get("data").unwrap().as_array().unwrap();
    assert_eq!(data.len(), 1);
    
    let embedding = data.first().unwrap();
    assert_eq!(embedding.get("object").unwrap().as_str().unwrap(), "embedding");
    assert_eq!(embedding.get("index").unwrap().as_u64().unwrap(), 0);
    
    let embedding_vector = embedding.get("embedding").unwrap().as_array().unwrap();
    assert!(!embedding_vector.is_empty());
    // Check that all elements are numbers
    for value in embedding_vector {
        assert!(value.is_number());
    }
}

#[tokio::test(flavor = "multi_thread")]
#[traced_test]
async fn test_embeddings_array_input() {
    let client = make_embedded_gateway().await;
    let state = client.get_app_state_data().unwrap().clone();

    let test_inputs = vec!["Hello, world!", "This is a test", "Another example"];

    let request: EmbeddingsRequest = serde_json::from_value(json!({
        "model": "text-embedding-3-small",
        "input": test_inputs
    }))
    .unwrap();

    let response = tensorzero_internal::endpoints::embeddings::create_embeddings_handler(
        State(state),
        Json(request),
    )
    .await
    .unwrap();

    // Convert Json response to Value for testing
    let response_json: Value = serde_json::to_value(&response.0).unwrap();
    println!("Array input response: {response_json:?}");

    // Validate response structure
    assert_eq!(response_json.get("object").unwrap().as_str().unwrap(), "list");
    assert_eq!(response_json.get("model").unwrap().as_str().unwrap(), "text-embedding-3-small");
    
    let data = response_json.get("data").unwrap().as_array().unwrap();
    assert_eq!(data.len(), test_inputs.len());
    
    // Check each embedding
    for (i, embedding) in data.iter().enumerate() {
        assert_eq!(embedding.get("object").unwrap().as_str().unwrap(), "embedding");
        assert_eq!(embedding.get("index").unwrap().as_u64().unwrap(), i as u64);
        
        let embedding_vector = embedding.get("embedding").unwrap().as_array().unwrap();
        assert!(!embedding_vector.is_empty());
        // Check that all elements are numbers
        for value in embedding_vector {
            assert!(value.is_number());
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
#[traced_test]
async fn test_embeddings_unknown_model() {
    let client = make_embedded_gateway().await;
    let state = client.get_app_state_data().unwrap().clone();

    let request: EmbeddingsRequest = serde_json::from_value(json!({
        "model": "nonexistent-model",
        "input": "Hello, world!"
    }))
    .unwrap();

    let response = tensorzero_internal::endpoints::embeddings::create_embeddings_handler(
        State(state),
        Json(request),
    )
    .await;

    // Should return an error for unknown model
    assert!(response.is_err());
    let error = response.unwrap_err();
    let error_details = error.get_details();
    println!("Unknown model error: {error_details:?}");
    
    // Should be an UnknownModel error
    assert!(matches!(
        error_details,
        tensorzero_internal::error::ErrorDetails::UnknownModel { .. }
    ));
}

#[tokio::test(flavor = "multi_thread")]
#[traced_test]
async fn test_embeddings_with_user_parameter() {
    let client = make_embedded_gateway().await;
    let state = client.get_app_state_data().unwrap().clone();

    let request: EmbeddingsRequest = serde_json::from_value(json!({
        "model": "text-embedding-3-small",
        "input": "Hello, world!",
        "user": "test-user-123"
    }))
    .unwrap();

    let response = tensorzero_internal::endpoints::embeddings::create_embeddings_handler(
        State(state),
        Json(request),
    )
    .await
    .unwrap();

    // Convert Json response to Value for testing
    let response_json: Value = serde_json::to_value(&response.0).unwrap();
    println!("With user parameter response: {response_json:?}");

    // Validate response structure (user parameter should not affect response format)
    assert_eq!(response_json.get("object").unwrap().as_str().unwrap(), "list");
    assert_eq!(response_json.get("model").unwrap().as_str().unwrap(), "text-embedding-3-small");
    
    let data = response_json.get("data").unwrap().as_array().unwrap();
    assert_eq!(data.len(), 1);
}

#[tokio::test(flavor = "multi_thread")]
#[traced_test]
async fn test_embeddings_empty_input_array() {
    let client = make_embedded_gateway().await;
    let state = client.get_app_state_data().unwrap().clone();

    let request: EmbeddingsRequest = serde_json::from_value(json!({
        "model": "text-embedding-3-small",
        "input": []
    }))
    .unwrap();

    let response = tensorzero_internal::endpoints::embeddings::create_embeddings_handler(
        State(state),
        Json(request),
    )
    .await
    .unwrap();

    // Convert Json response to Value for testing
    let response_json: Value = serde_json::to_value(&response.0).unwrap();
    println!("Empty array response: {response_json:?}");

    // Validate response structure for empty input
    assert_eq!(response_json.get("object").unwrap().as_str().unwrap(), "list");
    assert_eq!(response_json.get("model").unwrap().as_str().unwrap(), "text-embedding-3-small");
    
    let data = response_json.get("data").unwrap().as_array().unwrap();
    assert_eq!(data.len(), 0);
}

#[tokio::test(flavor = "multi_thread")]
#[traced_test]
async fn test_embeddings_openai_compatibility() {
    let client = make_embedded_gateway().await;
    let state = client.get_app_state_data().unwrap().clone();

    let request: EmbeddingsRequest = serde_json::from_value(json!({
        "model": "text-embedding-3-small",
        "input": "The food was delicious and the waiter..."
    }))
    .unwrap();

    let response = tensorzero_internal::endpoints::embeddings::create_embeddings_handler(
        State(state),
        Json(request),
    )
    .await
    .unwrap();

    // Convert Json response to Value for testing
    let response_json: Value = serde_json::to_value(&response.0).unwrap();
    
    // Validate OpenAI compatibility - response should match OpenAI's embedding API structure
    assert!(response_json.get("object").is_some());
    assert!(response_json.get("data").is_some());
    assert!(response_json.get("model").is_some());
    
    let data = response_json.get("data").unwrap().as_array().unwrap();
    let embedding = data.first().unwrap();
    
    // Check required fields according to OpenAI API spec
    assert!(embedding.get("object").is_some());
    assert!(embedding.get("embedding").is_some());
    assert!(embedding.get("index").is_some());
    
    assert_eq!(embedding.get("object").unwrap().as_str().unwrap(), "embedding");
    assert_eq!(embedding.get("index").unwrap().as_u64().unwrap(), 0);
    
    let embedding_vector = embedding.get("embedding").unwrap().as_array().unwrap();
    assert!(!embedding_vector.is_empty());
    
    // Verify all embedding values are float numbers in expected range
    for value in embedding_vector {
        let float_val = value.as_f64().unwrap();
        assert!(float_val >= -1.0 && float_val <= 1.0, "Embedding value {} out of expected range", float_val);
    }
} 