# n8n Docker - Plateforme Multi-Agents & Modèles Locaux (Bionic)

Plateforme d'orchestration de flux et de **systèmes multi-agents autonomes** basée sur **n8n**, connectée aux modèles d'intelligence artificielle locaux hébergés par **Bionic** (modèle `qwen3.8` / `qwen/qwen3.8-27b`) via la passerelle `host.docker.internal`, et enrichie par un microservice d'outils haute performance développé en **Rust** ([bionic-agent-tools](file:///Users/julien.simand/n8n/src)).

---

## 🏛️ Architecture du Projet

```mermaid
graph TD
    User([Utilisateur / Client Webhook]) -->|HTTP Chat / Webhook 5678| N8N[Conteneur Docker n8n]
    
    subgraph n8n Workflow Multi-Agents
        N8N --> Supervisor[Agent Superviseur LangChain]
        Supervisor --> Memory[Window Buffer Memory]
        Supervisor --> Model[OpenAI Chat Model]
    end
    
    Model -->|Inférence OpenAI-compatible| HostBionic[Bionic LLM Engine\nHost:1234 / qwen3.8]
    
    subgraph Outils Métier Rust Natifs (Port 3000)
        Supervisor -->|Tool Calling HTTP| RustCalc[Outil Calculateur\nArithmétique, Logique, Sqrt, Abs]
        Supervisor -->|Tool Calling HTTP| RustText[Outil Traitement Texte\nStats, Sentiments, Keywords]
        Supervisor -->|Tool Calling HTTP| RustRAG[Moteur RAG Rust\nIndexation & Recherche Documentaire]
    end

    RustCalc --- RustService[Microservice bionic-agent-tools]
    RustText --- RustService
    RustRAG --- RustService
    RustService -.->|Health Check / Models| HostBionic
```

---

## 🚀 Démarrage Rapide

### 1. Prérequis
* Docker & Docker Compose installés et démarrés.
* Bionic / LM Studio actif sur la machine hôte exposant l'API OpenAI locale sur le port `1234` (modèle `qwen/qwen3.8-27b` ou `qwen/qwen3.5-9b`).
* (Optionnel pour développement local) Chaîne de compilation Rust 1.80+ (`cargo`, `rustc`).

### 2. Configuration d'environnement
Copiez le fichier d'exemple et ajustez les variables si nécessaire :
```bash
cp .env.example .env
```

### 3. Lancer l'infrastructure complète via Docker Compose
Cette commande démarre le conteneur n8n ainsi que le microservice Rust compilé :
```bash
docker compose up -d --build
```

### 4. Vérifier l'état des services
```bash
docker compose ps
```

* **n8n** : [http://localhost:5678](http://localhost:5678)
* **Microservice Rust Tools (Health)** : [http://localhost:3000/health](http://localhost:3000/health)
* **Catalogue d'outils OpenAPI/JSON** : [http://localhost:3000/api/v1/tools](http://localhost:3000/api/v1/tools)

---

## 🦀 Microservice Rust : `bionic-agent-tools`

Le backend d'outils d'agents est écrit en **Rust 2021** avec le runtime asynchrone **Tokio** et le framework web **Axum**. Il fournit aux agents n8n une exécution native sans latence, avec une garantie de sécurité mémoire absolue (*Memory Safety*).

### Outils Disponibles pour n8n AI Agent

1. **`calculator`** :
   - Évaluation d'expressions arithmétiques et mathématiques sans injection de code arbitraire (`+`, `-`, `*`, `/`, `%`, `^`, parenthèses, fonctions `sqrt`, `abs`, `round`, `floor`, `ceil`).
   - Endpoint direct : `POST /api/v1/tools/calculator` avec `{"expression": "sqrt(144) + 25 * 4"}`.
2. **`text_processor`** :
   - Analyse lexicale, métriques (mots, caractères, phrases), extraction de mots-clés pondérés, détection de polarité et de sentiment.
   - Endpoint direct : `POST /api/v1/tools/text` avec `{"text": "Ce projet est excellent et très performant !"}`.
3. **`rag_engine`** :
   - Moteur d'indexation et de recherche documentaire interne avec scoring de pertinence TF/BM25.
   - Endpoint direct : `POST /api/v1/tools/rag` avec `{"action": "search", "query": "bionic llm", "top_k": 3}`.
4. **Endpoint Unifié** :
   - `POST /api/v1/tools/execute` avec `{"tool": "calculator", "parameters": {"expression": "100 / 4"}}`.

### Exécution locale directe (hors Docker)
```bash
cargo run --release
```

---

## 🧪 Conformité aux Normes de Développement (TDD & Qualité)

Ce projet respecte intégralement la norme d'ingénierie logicielle [norme.md](file:///Users/julien.simand/n8n/norme.md) :

* **Cycle TDD Red-Green-Refactor** : Tests unitaires rédigés avant le code de production.
* **Pattern AAA** : Chaque test est strictement découpé en *Arrange*, *Act*, *Assert*.
* **Critères FIRST** : Tests rapides (< 5 ms), isolés, reproductibles et auto-validants.
* **Doublures de test (Mocks)** : Simulation de l'API Bionic via `wiremock` pour des tests unitaires déterministes sans dépendance réseau externe.
* **Analyses Statiques Stricte** : `cargo clippy -- -D warnings` et `cargo fmt --check` validés à 100% sans aucun avertissement.

### Lancer la suite de tests et les vérifications qualité
```bash
# Exécution de tous les tests unitaires et d'intégration
cargo test

# Vérification du formatage de code
cargo fmt --check

# Analyse statique et linters stricts
cargo clippy -- -D warnings
```

---

## 🤖 Importation du Workflow Multi-Agents dans n8n

Un workflow de référence prêt pour la production est disponible dans [workflows/multi_agent_bionic_workflow.json](file:///Users/julien.simand/n8n/workflows/multi_agent_bionic_workflow.json).

### Étapes d'installation dans n8n :
1. Connectez-vous sur [http://localhost:5678](http://localhost:5678).
2. Dans le menu de gauche, cliquez sur **Workflows** > **Import from File...** et sélectionnez `workflows/multi_agent_bionic_workflow.json`.
3. Configurez le Credential **OpenAI API** :
   - **Credential Name** : `Bionic Host LLM`
   - **API Key** : `dummy-token-local` (ou votre token Bionic)
   - **Base URL** : `http://host.docker.internal:1234/v1`
4. Activez le workflow et ouvrez la fenêtre de Chat intégrée pour dialoguer avec l'**Agent Superviseur**. L'agent interrogera automatiquement le modèle local `qwen3.8` et exécutera les outils Rust en fonction de vos demandes.

---

## 📁 Structure du Projet

```text
.
├── Cargo.toml                       # Manifeste Cargo Rust du microservice
├── Dockerfile                       # Image multi-stage optimisée pour bionic-agent-tools
├── docker-compose.yml               # Orchestration Docker (n8n + bionic-agent-tools)
├── .env.example                     # Gabarit de configuration des variables d'environnement
├── .env                             # Variables d'environnement locales
├── norme.md                         # Norme de développement TDD & bonnes pratiques
├── projet.md                        # Fiche de cadrage projet & architecture multi-agents
├── README.md                        # Documentation opérationnelle et guide d'exploitation
├── src/
│   ├── lib.rs                       # Bibliothèque racine
│   ├── main.rs                      # Point d'entrée binaire du serveur Axum
│   ├── config.rs                    # Gestion de la configuration applicative
│   ├── bionic/                      # Client Bionic LLM (OpenAI v1 compatible)
│   ├── tools/                       # Modules d'outils d'agents (Calculator, Text, RAG)
│   └── server/                      # Routes et handlers HTTP Axum
├── tests/                           # Suite de tests TDD (AAA, Mocks wiremock, FIRST)
└── workflows/
    └── multi_agent_bionic_workflow.json # Workflow démo multi-agents n8n exporté
```

---

## 🛠️ Dépannage & Commandes Utiles

### Consulter les logs
```bash
# Logs n8n
docker compose logs -f n8n

# Logs du service Rust
docker compose logs -f rust-tools
```

### Tester la connectivité hôte Bionic depuis n8n
```bash
docker compose exec n8n curl -s http://host.docker.internal:1234/v1/models
```

### Arrêt propre
```bash
docker compose down
```