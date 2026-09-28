use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub object: String,
    #[serde(default)]
    pub owned_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelsResponse {
    pub object: String,
    pub data: Vec<ModelInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct BionicClient {
    client: Client,
    base_url: String,
    api_key: Option<String>,
}

impl BionicClient {
    pub fn new(base_url: &str, api_key: Option<String>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .unwrap_or_default();

        let trimmed_url = base_url.trim_end_matches('/').to_string();

        Self {
            client,
            base_url: trimmed_url,
            api_key,
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub async fn check_health(&self) -> Result<Vec<ModelInfo>, reqwest::Error> {
        let url = format!("{}/v1/models", self.base_url);
        let mut req = self.client.get(&url);

        if let Some(ref key) = self.api_key {
            req = req.bearer_auth(key);
        }

        let res = req.send().await?;
        let res = res.error_for_status()?;
        let models: ModelsResponse = res.json().await?;
        Ok(models.data)
    }

    pub async fn create_chat_completion(
        &self,
        request: &ChatCompletionRequest,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/v1/chat/completions", self.base_url);
        let mut req = self.client.post(&url).json(request);

        if let Some(ref key) = self.api_key {
            req = req.bearer_auth(key);
        }

        let res = req.send().await?;
        let res = res.error_for_status()?;
        let json_body: serde_json::Value = res.json().await?;
        Ok(json_body)
    }
}
