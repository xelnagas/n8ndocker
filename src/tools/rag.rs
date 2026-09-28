use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub title: String,
    pub content: String,
    #[serde(default)]
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub document: Document,
    pub score: f64,
}

#[derive(Debug, Clone)]
pub struct RagEngine {
    documents: Arc<RwLock<Vec<Document>>>,
}

impl Default for RagEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl RagEngine {
    pub fn new() -> Self {
        Self {
            documents: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn add_document(&self, doc: Document) {
        let mut docs = self.documents.write().expect("lock poisoned");
        // Remplacer si l'id existe déjà
        if let Some(pos) = docs.iter().position(|d| d.id == doc.id) {
            docs[pos] = doc;
        } else {
            docs.push(doc);
        }
    }

    pub fn list_documents(&self) -> Vec<Document> {
        let docs = self.documents.read().expect("lock poisoned");
        docs.clone()
    }

    pub fn search(&self, query: &str, top_k: usize) -> Vec<SearchResult> {
        let query_terms: Vec<String> = query
            .split_whitespace()
            .map(|t| t.to_lowercase())
            .filter(|t| t.len() >= 2)
            .collect();

        if query_terms.is_empty() {
            return Vec::new();
        }

        let docs = self.documents.read().expect("lock poisoned");
        let mut results: Vec<SearchResult> = Vec::new();

        for doc in docs.iter() {
            let title_lower = doc.title.to_lowercase();
            let content_lower = doc.content.to_lowercase();

            let mut score = 0.0;
            let mut matched_any = false;

            for term in &query_terms {
                let term_in_title = title_lower.matches(term).count();
                let term_in_content = content_lower.matches(term).count();

                if term_in_title > 0 || term_in_content > 0 {
                    matched_any = true;
                    // Le titre a un poids supérieur (x3)
                    score += (term_in_title as f64 * 3.0) + (term_in_content as f64 * 1.0);
                }
            }

            if matched_any && score > 0.0 {
                // Normalisation par longueur pour éviter de sur-pondérer les documents géants
                let length_penalty = (doc.content.len() as f64).sqrt().max(1.0);
                let normalized_score = (score / length_penalty) * 10.0;

                results.push(SearchResult {
                    document: doc.clone(),
                    score: normalized_score,
                });
            }
        }

        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        results.truncate(top_k);
        results
    }

    pub fn execute_tool(&self, payload: &serde_json::Value) -> Result<serde_json::Value, String> {
        let action = payload
            .get("action")
            .and_then(|a| a.as_str())
            .unwrap_or("search");

        match action {
            "search" => {
                let query = payload
                    .get("query")
                    .and_then(|q| q.as_str())
                    .ok_or_else(|| "Paramètre 'query' obligatoire pour la recherche".to_string())?;

                let top_k = payload.get("top_k").and_then(|k| k.as_u64()).unwrap_or(3) as usize;

                let hits = self.search(query, top_k);
                Ok(serde_json::json!({
                    "status": "success",
                    "action": "search",
                    "count": hits.len(),
                    "results": hits
                }))
            }
            "index" => {
                let doc_val = payload.get("document").ok_or_else(|| {
                    "Paramètre 'document' obligatoire pour l'indexation".to_string()
                })?;

                let doc: Document = serde_json::from_value(doc_val.clone())
                    .map_err(|e| format!("Format de document invalide: {}", e))?;

                self.add_document(doc);
                Ok(serde_json::json!({
                    "status": "success",
                    "action": "index",
                    "message": "Document indexé avec succès"
                }))
            }
            "list" => {
                let docs = self.list_documents();
                Ok(serde_json::json!({
                    "status": "success",
                    "action": "list",
                    "count": docs.len(),
                    "documents": docs
                }))
            }
            other => Err(format!(
                "Action inconnue: '{}'. Actions supportées: search, index, list",
                other
            )),
        }
    }
}
