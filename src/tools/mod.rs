pub mod calculator;
pub mod rag;
pub mod text_processor;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

pub fn get_tool_catalog() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: "calculator".to_string(),
            description: "Évalue de manière sécurisée des expressions mathématiques et logiques (addition, soustraction, multiplication, division, modulo, puissance ^, parenthèses, sqrt, abs, round)."
                .to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "expression": {
                        "type": "string",
                        "description": "L'expression arithmétique à calculer (ex: 'sqrt(144) + 10 * 2')"
                    }
                },
                "required": ["expression"]
            }),
        },
        ToolDefinition {
            name: "text_processor".to_string(),
            description: "Analyse lexicale, comptage de mots/phrases, extraction de mots-clés et détection de polarité/sentiment."
                .to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "text": {
                        "type": "string",
                        "description": "Le texte à analyser"
                    }
                },
                "required": ["text"]
            }),
        },
        ToolDefinition {
            name: "rag_engine".to_string(),
            description: "Moteur de recherche documentaire et de connaissances pour les agents RAG (actions: search, index, list)."
                .to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "action": {
                        "type": "string",
                        "enum": ["search", "index", "list"],
                        "description": "Action à effectuer"
                    },
                    "query": {
                        "type": "string",
                        "description": "Terme ou phrase de recherche (pour action 'search')"
                    },
                    "top_k": {
                        "type": "integer",
                        "description": "Nombre maximal de résultats (défaut: 3)"
                    },
                    "document": {
                        "type": "object",
                        "description": "Document à indexer (id, title, content, metadata)"
                    }
                },
                "required": ["action"]
            }),
        },
    ]
}
