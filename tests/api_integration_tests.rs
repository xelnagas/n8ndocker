use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use bionic_agent_tools::{
    bionic::BionicClient,
    config::AppConfig,
    server::app,
    tools::{calculator::Calculator, rag::RagEngine, text_processor::TextProcessor},
};
use std::sync::Arc;
use tower::ServiceExt;

fn build_test_app() -> axum::Router {
    let config = AppConfig {
        host: "127.0.0.1".to_string(),
        port: 3000,
        bionic_base_url: "http://127.0.0.1:1234".to_string(),
        bionic_api_key: None,
    };
    let calculator = Arc::new(Calculator::new());
    let text_processor = Arc::new(TextProcessor::new());
    let rag_engine = Arc::new(RagEngine::new());
    let bionic_client = Arc::new(BionicClient::new(&config.bionic_base_url, None));

    app(
        config,
        calculator,
        text_processor,
        rag_engine,
        bionic_client,
    )
}

#[tokio::test]
async fn test_health_endpoint() {
    // 1. ARRANGE
    let app = build_test_app();
    let request = Request::builder()
        .uri("/health")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    // 2. ACT
    let response = app.oneshot(request).await.unwrap();

    // 3. ASSERT
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_tools_catalog_endpoint() {
    // 1. ARRANGE
    let app = build_test_app();
    let request = Request::builder()
        .uri("/api/v1/tools")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    // 2. ACT
    let response = app.oneshot(request).await.unwrap();

    // 3. ASSERT
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_execute_calculator_via_api() {
    // 1. ARRANGE
    let app = build_test_app();
    let payload = serde_json::json!({
        "tool": "calculator",
        "parameters": {
            "expression": "50 * 2 + 10"
        }
    });

    let request = Request::builder()
        .uri("/api/v1/tools/execute")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    // 2. ACT
    let response = app.oneshot(request).await.unwrap();

    // 3. ASSERT
    assert_eq!(response.status(), StatusCode::OK);
}
