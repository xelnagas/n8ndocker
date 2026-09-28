pub mod routes;

use axum::Router;
use std::sync::Arc;

use crate::{
    bionic::BionicClient,
    config::AppConfig,
    tools::{calculator::Calculator, rag::RagEngine, text_processor::TextProcessor},
};

pub fn app(
    config: AppConfig,
    calculator: Arc<Calculator>,
    text_processor: Arc<TextProcessor>,
    rag_engine: Arc<RagEngine>,
    bionic_client: Arc<BionicClient>,
) -> Router {
    let state = routes::AppState {
        config,
        calculator,
        text_processor,
        rag_engine,
        bionic_client,
    };
    routes::create_router(state)
}
