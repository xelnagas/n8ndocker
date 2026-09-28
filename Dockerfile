# ==============================================================================
# Multi-stage Dockerfile pour bionic-agent-tools (Rust)
# ==============================================================================

# --- Étape de Build ---
FROM rust:1.90-bookworm AS builder

WORKDIR /usr/src/bionic-agent-tools

# Optimisation du cache Docker : copie des manifestes de dépendances
COPY Cargo.toml Cargo.lock ./

# Pré-compilation des dépendances pour bénéficier du cache de layer Docker
RUN mkdir -p src && \
    echo "pub mod bionic; pub mod config; pub mod server; pub mod tools;" > src/lib.rs && \
    mkdir -p src/bionic src/config src/server src/tools && \
    touch src/bionic/mod.rs src/server/mod.rs src/tools/mod.rs && \
    echo "pub struct AppConfig;" > src/config.rs && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release || true && \
    rm -rf src

# Copie des sources et tests réels
COPY src ./src
COPY tests ./tests

# Compilation finale en mode Release
RUN touch src/main.rs src/lib.rs && cargo build --release

# --- Étape Finale Runtime Légère ---
FROM debian:bookworm-slim AS runtime

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates curl && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copie du binaire compilé depuis l'étape de build
COPY --from=builder /usr/src/bionic-agent-tools/target/release/bionic-agent-tools /app/bionic-agent-tools

# Variables d'environnement par défaut
ENV RUST_TOOLS_HOST=0.0.0.0 \
    RUST_TOOLS_PORT=3000 \
    BIONIC_BASE_URL=http://host.docker.internal:1234 \
    RUST_LOG=bionic_agent_tools=info,tower_http=info

EXPOSE 3000

HEALTHCHECK --interval=15s --timeout=5s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:3000/health || exit 1

ENTRYPOINT ["/app/bionic-agent-tools"]
