use bionic_agent_tools::tools::rag::{Document, RagEngine};

#[test]
fn test_index_and_search_documents() {
    // 1. ARRANGE
    let engine = RagEngine::new();
    engine.add_document(Document {
        id: "doc-1".to_string(),
        title: "Architecture n8n".to_string(),
        content:
            "n8n est un orchestrateur de flux extensible supportant les agents IA et webhooks."
                .to_string(),
        metadata: serde_json::json!({ "category": "orchestration" }),
    });
    engine.add_document(Document {
        id: "doc-2".to_string(),
        title: "Modèles Bionic LLM".to_string(),
        content: "Bionic héberge localement le modèle qwen3.8 avec une compatibilité API OpenAI."
            .to_string(),
        metadata: serde_json::json!({ "category": "ai" }),
    });

    // 2. ACT
    let results = engine.search("qwen3.8 bionic", 5);

    // 3. ASSERT
    assert!(!results.is_empty());
    assert_eq!(results[0].document.id, "doc-2");
    assert!(results[0].score > 0.0);
}

#[test]
fn test_search_returns_empty_when_no_match() {
    // 1. ARRANGE
    let engine = RagEngine::new();
    engine.add_document(Document {
        id: "doc-1".to_string(),
        title: "Rust Lang".to_string(),
        content: "Rust garantit la sécurité de la mémoire sans ramasse-miettes.".to_string(),
        metadata: serde_json::json!({}),
    });

    // 2. ACT
    let results = engine.search("astronomie galaxie", 5);

    // 3. ASSERT
    assert!(results.is_empty());
}

#[test]
fn test_tool_execute_search_and_index() {
    // 1. ARRANGE
    let engine = RagEngine::new();

    // 2. ACT: Index a document via JSON
    let index_payload = serde_json::json!({
        "action": "index",
        "document": {
            "id": "doc-rust",
            "title": "Outils Rust",
            "content": "Les outils d'agents sont compilés en code natif pour une performance maximale.",
            "metadata": { "author": "julien" }
        }
    });
    let index_res = engine.execute_tool(&index_payload).expect("should index");

    // 3. ASSERT indexing succeeded
    assert_eq!(index_res["status"], "success");

    // ACT: Search document via JSON
    let search_payload = serde_json::json!({
        "action": "search",
        "query": "outils rust natif",
        "top_k": 3
    });
    let search_res = engine.execute_tool(&search_payload).expect("should search");

    // ASSERT search returned results
    assert_eq!(search_res["status"], "success");
    let hits = search_res["results"].as_array().unwrap();
    assert!(!hits.is_empty());
    assert_eq!(hits[0]["document"]["id"], "doc-rust");
}
