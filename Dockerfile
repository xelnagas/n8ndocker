# ==============================================================================
# Multi-stage Dockerfile pour bionic-agent-tools (Rust)
# ==============================================================================

# --- Étape de Build ---
FROM rust:1.90-bookworm AS builder

WORKDIR /usr/src/bionic-agent-tools

# Copie des manifestes, configuration cargo et sources vendored (offline)
COPY Cargo.toml Cargo.lock ./
COPY .cargo/config.offline.toml ./.cargo/config.toml
COPY vendor ./vendor
COPY src ./src
COPY tests ./tests

# Compilation en mode Release 100% Offline
RUN cargo build --release --offline

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
