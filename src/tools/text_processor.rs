use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextStats {
    pub characters: usize,
    pub words: usize,
    pub sentences: usize,
    pub lines: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeywordScore {
    pub word: String,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentResult {
    pub label: String,
    pub score: f64,
}

#[derive(Debug, Clone, Default)]
pub struct TextProcessor {
    stop_words: HashSet<&'static str>,
    positive_words: HashSet<&'static str>,
    negative_words: HashSet<&'static str>,
}

impl TextProcessor {
    pub fn new() -> Self {
        let stop_words: HashSet<&'static str> = [
            "le", "la", "les", "un", "une", "des", "du", "de", "d", "l", "et", "ou", "en", "dans",
            "sur", "pour", "par", "avec", "sans", "est", "sont", "a", "ce", "cette", "ces", "qui",
            "que", "the", "a", "an", "and", "or", "in", "on", "for", "with", "is", "are", "of",
            "to",
        ]
        .into_iter()
        .collect();

        let positive_words: HashSet<&'static str> = [
            "bon",
            "bien",
            "excellent",
            "excellente",
            "parfait",
            "parfaite",
            "sécurisé",
            "sécurisée",
            "rapide",
            "performant",
            "robuste",
            "réussi",
            "succès",
            "fiable",
            "optimal",
            "génial",
            "good",
            "great",
            "excellent",
            "perfect",
            "fast",
            "reliable",
            "robust",
            "optimal",
            "safe",
        ]
        .into_iter()
        .collect();

        let negative_words: HashSet<&'static str> = [
            "mauvais",
            "erreur",
            "critique",
            "grave",
            "échec",
            "inadmissible",
            "lent",
            "bloqué",
            "bug",
            "faille",
            "problème",
            "dangereux",
            "cassé",
            "invalide",
            "bad",
            "error",
            "critical",
            "severe",
            "failure",
            "slow",
            "broken",
            "vulnerability",
        ]
        .into_iter()
        .collect();

        Self {
            stop_words,
            positive_words,
            negative_words,
        }
    }

    pub fn analyze_stats(&self, text: &str) -> TextStats {
        let characters = text.chars().count();
        let words = text
            .split_whitespace()
            .filter(|w| w.chars().any(|c| c.is_alphanumeric()))
            .count();

        let sentences = text
            .split(&['.', '!', '?'][..])
            .filter(|s| !s.trim().is_empty())
            .count();

        let lines = text.lines().count();

        TextStats {
            characters,
            words,
            sentences: sentences.max(if words > 0 { 1 } else { 0 }),
            lines: lines.max(1),
        }
    }

    pub fn extract_keywords(&self, text: &str, top_n: usize) -> Vec<KeywordScore> {
        let mut counts: HashMap<String, usize> = HashMap::new();

        for word in text.split(|c: char| !c.is_alphanumeric()) {
            let lower = word.to_lowercase();
            if lower.len() >= 3 && !self.stop_words.contains(lower.as_str()) {
                *counts.entry(lower).or_insert(0) += 1;
            }
        }

        let mut sorted: Vec<KeywordScore> = counts
            .into_iter()
            .map(|(word, count)| KeywordScore { word, count })
            .collect();

        sorted.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.word.cmp(&b.word)));
        sorted.truncate(top_n);
        sorted
    }

    pub fn analyze_sentiment(&self, text: &str) -> SentimentResult {
        let mut pos_count = 0;
        let mut neg_count = 0;

        for raw_word in text.split(|c: char| !c.is_alphanumeric()) {
            let lower = raw_word.to_lowercase();
            if self.positive_words.contains(lower.as_str()) {
                pos_count += 1;
            } else if self.negative_words.contains(lower.as_str()) {
                neg_count += 1;
            }
        }

        let total = pos_count + neg_count;
        if total == 0 {
            SentimentResult {
                label: "neutral".to_string(),
                score: 0.0,
            }
        } else {
            let diff = pos_count as f64 - neg_count as f64;
            let score = diff / (total as f64);
            let label = if score > 0.15 {
                "positive".to_string()
            } else if score < -0.15 {
                "negative".to_string()
            } else {
                "neutral".to_string()
            };
            SentimentResult { label, score }
        }
    }

    pub fn execute_tool(&self, payload: &serde_json::Value) -> Result<serde_json::Value, String> {
        let text = payload
            .get("text")
            .and_then(|t| t.as_str())
            .ok_or_else(|| "Champ 'text' obligatoire".to_string())?;

        let stats = self.analyze_stats(text);
        let keywords = self.extract_keywords(text, 5);
        let sentiment = self.analyze_sentiment(text);

        Ok(serde_json::json!({
            "status": "success",
            "stats": stats,
            "keywords": keywords,
            "sentiment": sentiment
        }))
    }
}
