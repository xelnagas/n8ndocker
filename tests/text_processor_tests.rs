use bionic_agent_tools::tools::text_processor::{TextProcessor, TextStats};

#[test]
fn test_text_statistics_calculation() {
    // 1. ARRANGE
    let processor = TextProcessor::new();
    let text = "Bonjour n8n. Bienvenue sur la plateforme multi-agents Bionic !";

    // 2. ACT
    let stats: TextStats = processor.analyze_stats(text);

    // 3. ASSERT
    assert_eq!(stats.words, 8);
    assert_eq!(stats.sentences, 2);
    assert!(stats.characters > 50);
}

#[test]
fn test_extract_keywords() {
    // 1. ARRANGE
    let processor = TextProcessor::new();
    let text = "Rust offre une sécurité mémoire totale. La mémoire en Rust est gérée sans ramasse-miettes.";

    // 2. ACT
    let keywords = processor.extract_keywords(text, 3);

    // 3. ASSERT
    assert!(!keywords.is_empty());
    assert!(keywords
        .iter()
        .any(|k| k.word.to_lowercase() == "rust" || k.word.to_lowercase() == "mémoire"));
}

#[test]
fn test_sentiment_analysis_positive() {
    // 1. ARRANGE
    let processor = TextProcessor::new();
    let text = "Cette solution est excellente, très rapide et parfaitement sécurisée !";

    // 2. ACT
    let sentiment = processor.analyze_sentiment(text);

    // 3. ASSERT
    assert_eq!(sentiment.label, "positive");
    assert!(sentiment.score > 0.0);
}

#[test]
fn test_sentiment_analysis_negative() {
    // 1. ARRANGE
    let processor = TextProcessor::new();
    let text = "Le service rencontre une erreur critique, un échec grave et inadmissible.";

    // 2. ACT
    let sentiment = processor.analyze_sentiment(text);

    // 3. ASSERT
    assert_eq!(sentiment.label, "negative");
    assert!(sentiment.score < 0.0);
}

#[test]
fn test_tool_execute_json_output() {
    // 1. ARRANGE
    let processor = TextProcessor::new();
    let payload = serde_json::json!({
        "action": "analyze",
        "text": "Plateforme n8n et Rust. Système très robuste et rapide."
    });

    // 2. ACT
    let response = processor
        .execute_tool(&payload)
        .expect("should execute text tool");

    // 3. ASSERT
    assert_eq!(response["status"], "success");
    assert!(response["stats"]["words"].as_u64().unwrap() >= 8);
    assert_eq!(response["sentiment"]["label"], "positive");
}
