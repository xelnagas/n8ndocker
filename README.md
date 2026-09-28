# n8n Docker - Plateforme Multi-Agents & Modèles Locaux (Bionic)

Plateforme d'orchestration de flux et de **systèmes multi-agents autonomes** basée sur **n8n**, connectée aux modèles d'intelligence artificielle locaux hébergés par **Bionic** (modèle `qwen3.8` / `qwen/qwen3.8-27b`) via la passerelle `host.docker.internal`, enrichie par un microservice d'outils haute performance en **Rust** ([bionic-agent-tools](file:///Users/julien.simand/n8n/src)) et par un métamoteur de **recherche web privée autonome** ([SearXNG](file:///Users/julien.simand/n8n/searxng)).

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

    subgraph Recherche Web Privée (Port 8088 / 8080)
        Supervisor -->|Tool Calling JSON / HTTP| SearXNG[SearXNG Metasearch Engine\nhttp://searxng:8080]
    end

    subgraph Sandbox d'Exécution Sécurisée (Port 5680 / 8080)
        N8N -->|Code Execution & AI Sandbox| SandboxAPI[Sandbox Service API\nhttp://sandbox-api:8080]
        SandboxAPI --> SandboxRunner[Sandbox Runner DinD\nIsolation Conteneurisée]
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
Cette commande démarre le conteneur n8n, la sandbox d'exécution ainsi que le microservice Rust compilé :
```bash
docker compose up -d --build
```

### 4. Vérifier l'état des services
```bash
docker compose ps
```

* **n8n** : [http://localhost:5678](http://localhost:5678)
* **Sandbox d'Exécution n8n (Host)** : [http://localhost:5680](http://localhost:5680) (Santé : [http://localhost:5680/healthz](http://localhost:5680/healthz))
* **Sandbox d'Exécution n8n (Réseau Docker interne)** : `http://sandbox-api:8080` (Santé : `http://sandbox-api:8080/healthz`)
* **SearXNG Web Search (Interface Web Host)** : [http://localhost:8088](http://localhost:8088)
* **SearXNG Web Search (Réseau Docker interne / API JSON)** : `http://searxng:8080` (Requête API : `http://searxng:8080/search?q=test&format=json`)
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
# Exécution de tous les tests unitaires et d'intégration (dont conformité Docker)
cargo test

# Validation bout-en-bout du build Docker et healthcheck
./tests/test_docker_build.sh

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

## 🛡️ Sandbox d'Exécution Sécurisée (n8n AI Sandbox Service)

La stack Docker intègre la **sandbox d'exécution officielle n8n** (`ghcr.io/n8n-io/n8n-sandbox-service-api` et runner `n8n-sandbox-service-runner-dind`), offrant aux agents autonomes et aux flux d'automatisation un environnement d'exécution de code totalement étanche et sécurisé (*air-gapped*).

### URLs d'Accès de la Sandbox :
* **URL externe (Machine Host)** : [http://localhost:5680](http://localhost:5680)  
  *Endpoint de vérification de santé* : [http://localhost:5680/healthz](http://localhost:5680/healthz)
* **URL interne (Réseau Docker `agent-network`)** : `http://sandbox-api:8080`  
  *Endpoint de vérification interne* : `http://sandbox-api:8080/healthz`

### Composants & Architecture de la Sandbox :
1. **`sandbox-certs`** (`n8n-sandbox-service-api`) : Génère les certificats mTLS éphémères nécessaires pour sécuriser les canaux de contrôle gRPC entre l'API et le runner d'exécution.
2. **`sandbox-api`** (`n8n-sandbox-service-api`) : API de contrôle HTTP & gRPC exposée sur le port `5680` (hôte) / `8080` (interne), servant de point d'entrée pour la délégation d'exécution de code depuis n8n.
3. **`sandbox-runner-1`** (`n8n-sandbox-service-runner-dind`) : Moteur d'exécution Docker-in-Docker (*privileged*) instanciant à la demande des conteneurs sandbox éphémères (`n8n-sandbox-service-sandbox`) pour exécuter le code sans risque pour le système hôte ou le conteneur principal.

### Configuration n8n associée (`docker-compose.yml`) :
Le conteneur n8n est préconfiguré pour router nativement toute exécution de code IA vers le service sandbox :
```env
N8N_INSTANCE_AI_SANDBOX_ENABLED=true
N8N_INSTANCE_AI_SANDBOX_PROVIDER=n8n-sandbox
N8N_INSTANCE_AI_SANDBOX_API_URL=http://sandbox-api:8080
N8N_SANDBOX_SERVICE_URL=http://sandbox-api:8080
N8N_SANDBOX_SERVICE_API_KEY=sandbox-secret-token
```

---

## 🔍 Recherche Web Autonome & Privée : SearXNG

La stack Docker intègre le métamoteur open source et respectueux de la vie privée **SearXNG** (`searxng/searxng:latest`), permettant aux agents autonomes n8n d'effectuer des recherches sur le Web en direct sans dépendre d'APIs tierces payantes ou traçantes.

### URLs d'Accès de SearXNG :
* **Interface Web utilisateur (Machine Host)** : [http://localhost:8088](http://localhost:8088)  
  *Permet de tester manuellement des requêtes depuis votre navigateur.*
* **URL interne (Réseau Docker `agent-network`)** : `http://searxng:8080`
* **Endpoint API JSON pour n8n & Agents** : `http://searxng:8080/search?q={query}&format=json`  
  *(Sur la machine hôte : `http://localhost:8088/search?q={query}&format=json`)*

### Configuration Dédiée (`searxng/settings.yml`) :
Le métamoteur est préconfiguré pour une utilisation agentique locale fluide :
* **Formats activés** : `html` et `json` (permet le *Tool Calling* n8n sans blocage 403).
* **Limiteur désactivé** (`limiter: false`) : exécute les requêtes concurrentes des agents sans rejet de requêtes ni besoin d'un conteneur Redis/Valkey annexe.
* **Volume de configuration monté** : `./searxng:/etc/searxng:rw`.

### Intégration dans un Tool n8n AI Agent :
Pour donner la capacité de recherche Web à votre agent superviseur ou à un sous-agent de recherche :
1. Associez un outil **Custom Tool** (ou **HTTP Request**) au port `tools` du nœud **AI Agent**.
2. Méthode : `GET`
3. URL : `http://searxng:8080/search`
4. Paramètres de requête (*Query Parameters*) :
   - `q` : `{{ $fromAI('query', 'La requête de recherche web') }}`
   - `format` : `json`
5. L'agent reçoit une réponse JSON native avec la liste des résultats (`title`, `url`, `content`).

---

## 📁 Structure du Projet

```text
.
├── Cargo.toml                       # Manifeste Cargo Rust du microservice
├── Dockerfile                       # Image multi-stage optimisée pour bionic-agent-tools
├── docker-compose.yml               # Orchestration Docker (n8n + sandbox + searxng + bionic-agent-tools)
├── .env.example                     # Gabarit de configuration des variables d'environnement
├── .env                             # Variables d'environnement locales
├── norme.md                         # Norme de développement TDD & bonnes pratiques
├── projet.md                        # Fiche de cadrage projet & architecture multi-agents
├── README.md                        # Documentation opérationnelle et guide d'exploitation
├── searxng/
│   └── settings.yml                 # Configuration SearXNG (API JSON & limiter désactivé)
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

# Logs SearXNG (Recherche Web)
docker compose logs -f searxng

# Logs de la sandbox d'exécution
docker compose logs -f sandbox-api
docker compose logs -f sandbox-runner-1

# Logs du service Rust
docker compose logs -f rust-tools
```

### Tester l'état de la recherche web SearXNG
```bash
# Test direct depuis la machine hôte (format JSON)
curl -s "http://localhost:8088/search?q=bionic+ai&format=json"

# Test de connectivité interne n8n -> SearXNG
docker compose exec n8n wget -qO- "http://searxng:8080/search?q=bionic+ai&format=json"
```

### Tester l'état de la sandbox d'exécution
```bash
# Test direct depuis la machine hôte
curl -i http://localhost:5680/healthz

# Test de connectivité interne n8n -> Sandbox API
docker compose exec n8n wget -qO- http://sandbox-api:8080/healthz
```

### Tester la connectivité hôte Bionic depuis n8n
```bash
docker compose exec n8n curl -s http://host.docker.internal:1234/v1/models
```

### Arrêt propre
```bash
docker compose down
```