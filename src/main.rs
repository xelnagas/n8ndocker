use bionic_agent_tools::{
    bionic::BionicClient,
    config::AppConfig,
    server::app,
    tools::{calculator::Calculator, rag::RagEngine, text_processor::TextProcessor},
};
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "bionic_agent_tools=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = AppConfig::from_env();
    tracing::info!("Démarrage du microservice Rust Bionic Agent Tools");
    tracing::info!(
        "Configuration: Port={}, Bionic URL={}",
        config.port,
        config.bionic_base_url
    );

    let calculator = Arc::new(Calculator::new());
    let text_processor = Arc::new(TextProcessor::new());
    let rag_engine = Arc::new(RagEngine::new());
    let bionic_client = Arc::new(BionicClient::new(
        &config.bionic_base_url,
        config.bionic_api_key.clone(),
    ));

    let router = app(
        config.clone(),
        calculator,
        text_processor,
        rag_engine,
        bionic_client,
    );

    let bind_addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    tracing::info!("Serveur démarré avec succès sur http://{}", bind_addr);

    axum::serve(listener, router).await?;

    Ok(())
}
