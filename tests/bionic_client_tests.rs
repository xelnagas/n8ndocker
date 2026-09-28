use bionic_agent_tools::bionic::BionicClient;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_bionic_health_check_success() {
    // 1. ARRANGE (Doublure de test via MockServer)
    let mock_server = MockServer::start().await;
    let mock_response = serde_json::json!({
        "object": "list",
        "data": [
            { "id": "qwen/qwen3.8-27b", "object": "model", "owned_by": "organization_owner" }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/v1/models"))
        .respond_with(ResponseTemplate::new(200).set_body_json(mock_response))
        .mount(&mock_server)
        .await;

    let client = BionicClient::new(&mock_server.uri(), None);

    // 2. ACT
    let health = client.check_health().await;

    // 3. ASSERT
    assert!(health.is_ok());
    let models = health.unwrap();
    assert_eq!(models.len(), 1);
    assert_eq!(models[0].id, "qwen/qwen3.8-27b");
}

#[tokio::test]
async fn test_bionic_health_check_failure() {
    // 1. ARRANGE
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/models"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;

    let client = BionicClient::new(&mock_server.uri(), None);

    // 2. ACT
    let health = client.check_health().await;

    // 3. ASSERT
    assert!(health.is_err());
}
