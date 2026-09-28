use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

use crate::{
    bionic::BionicClient,
    config::AppConfig,
    tools::{
        calculator::Calculator, get_tool_catalog, rag::RagEngine, text_processor::TextProcessor,
    },
};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub calculator: Arc<Calculator>,
    pub text_processor: Arc<TextProcessor>,
    pub rag_engine: Arc<RagEngine>,
    pub bionic_client: Arc<BionicClient>,
}

#[derive(Serialize)]
pub struct HealthStatus {
    pub status: String,
    pub version: String,
    pub bionic_status: String,
    pub available_models: Vec<String>,
}

#[derive(Deserialize)]
pub struct ToolExecutionRequest {
    pub tool: String,
    pub parameters: Value,
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .route("/api/v1/tools", get(tools_catalog_handler))
        .route("/api/v1/tools/execute", post(execute_tool_handler))
        .route("/api/v1/tools/calculator", post(calculator_handler))
        .route("/api/v1/tools/text", post(text_handler))
        .route("/api/v1/tools/rag", post(rag_handler))
        .route("/api/v1/bionic/models", get(bionic_models_handler))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

async fn health_handler(State(state): State<AppState>) -> impl IntoResponse {
    let (bionic_status, available_models) = match state.bionic_client.check_health().await {
        Ok(models) => (
            "connected".to_string(),
            models.into_iter().map(|m| m.id).collect(),
        ),
        Err(e) => (format!("unavailable: {}", e), vec![]),
    };

    let response = HealthStatus {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        bionic_status,
        available_models,
    };

    (StatusCode::OK, Json(response))
}

async fn tools_catalog_handler() -> impl IntoResponse {
    let catalog = get_tool_catalog();
    (StatusCode::OK, Json(catalog))
}

async fn execute_tool_handler(
    State(state): State<AppState>,
    Json(payload): Json<ToolExecutionRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    match payload.tool.as_str() {
        "calculator" => match state.calculator.execute_tool(&payload.parameters) {
            Ok(res) => Ok((StatusCode::OK, Json(res))),
            Err(e) => Err((
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "error": e.to_string() })),
            )),
        },
        "text_processor" => match state.text_processor.execute_tool(&payload.parameters) {
            Ok(res) => Ok((StatusCode::OK, Json(res))),
            Err(e) => Err((
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "error": e })),
            )),
        },
        "rag_engine" => match state.rag_engine.execute_tool(&payload.parameters) {
            Ok(res) => Ok((StatusCode::OK, Json(res))),
            Err(e) => Err((
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "error": e })),
            )),
        },
        unknown => Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": format!("Outil inconnu: '{}'. Outils disponibles: calculator, text_processor, rag_engine", unknown)
            })),
        )),
    }
}

async fn calculator_handler(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    match state.calculator.execute_tool(&payload) {
        Ok(res) => Ok((StatusCode::OK, Json(res))),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": e.to_string() })),
        )),
    }
}

async fn text_handler(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    match state.text_processor.execute_tool(&payload) {
        Ok(res) => Ok((StatusCode::OK, Json(res))),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}

async fn rag_handler(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    match state.rag_engine.execute_tool(&payload) {
        Ok(res) => Ok((StatusCode::OK, Json(res))),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": e })),
        )),
    }
}

async fn bionic_models_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    match state.bionic_client.check_health().await {
        Ok(models) => Ok((
            StatusCode::OK,
            Json(serde_json::json!({ "models": models })),
        )),
        Err(e) => Err((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({ "error": format!("Impossible de joindre Bionic: {}", e) })),
        )),
    }
}
