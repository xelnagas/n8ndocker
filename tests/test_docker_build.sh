#!/usr/bin/env bash
# ==============================================================================
# Script de validation du build Docker et de la résilience aux régressions vendor
# ==============================================================================
set -euo pipefail

echo "========================================================"
echo "🧪 [TEST DOCKER] Début de la validation de build Docker"
echo "========================================================"

WORKSPACE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$WORKSPACE_DIR"

# 1. Vérifier l'absence de dossier vendor obligatoire
if [ -d "vendor" ]; then
    echo "⚠️  Attention: un dossier 'vendor' local existe sur l'hôte."
    echo "   Vérification que le Dockerfile n'en dépend pas..."
fi

# 2. Vérification syntaxique et anti-régression via le test unitaire Rust
echo "➡️  1/4 - Exécution des tests de conformité Dockerfile & .dockerignore..."
cargo test --test docker_compliance_tests --quiet
echo "✅ Tests de conformité Docker réussis !"

# 3. Validation de la configuration Docker Compose
echo "➡️  2/4 - Validation de docker-compose config..."
docker compose config > /dev/null
echo "✅ Configuration docker-compose valide !"

# 4. Build de l'image Docker bionic-agent-tools sans dépendance à vendor
echo "➡️  3/4 - Construction de l'image Docker (rust-tools)..."
docker compose build rust-tools
echo "✅ Build Docker réussi sans dépendance à un dossier vendor hôte !"

# 5. Test d'exécution et de santé en conteneur éphémère
echo "➡️  4/4 - Démarrage d'un conteneur de test et vérification du /health..."
TEST_PORT=3099
TEST_CONTAINER="test-rust-tools-ci-$$"

cleanup() {
    docker rm -f "$TEST_CONTAINER" >/dev/null 2>&1 || true
}
trap cleanup EXIT

docker run -d --name "$TEST_CONTAINER" \
    -p "${TEST_PORT}:3000" \
    -e RUST_TOOLS_PORT=3000 \
    -e BIONIC_BASE_URL="http://localhost:1234" \
    n8n-rust-tools:latest

# Attente active du conteneur (max 10 secondes)
RETRIES=10
HEALTHY=false
while [ $RETRIES -gt 0 ]; do
    if curl -fs "http://127.0.0.1:${TEST_PORT}/health" > /dev/null 2>&1; then
        HEALTHY=true
        break
    fi
    sleep 1
    RETRIES=$((RETRIES - 1))
done

if [ "$HEALTHY" = true ]; then
    HEALTH_RESP=$(curl -s "http://127.0.0.1:${TEST_PORT}/health")
    echo "✅ Healthcheck OK : $HEALTH_RESP"
else
    echo "❌ Erreur: Le conteneur n'a pas répondu sur /health dans les délais impartis."
    docker logs "$TEST_CONTAINER"
    exit 1
fi

echo "========================================================"
echo "🎉 [SUCCÈS] Tous les tests Docker et anti-régression sont au vert !"
echo "========================================================"
