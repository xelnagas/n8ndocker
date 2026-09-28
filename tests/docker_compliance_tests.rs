use std::fs;
use std::path::Path;

#[test]
fn test_dockerfile_does_not_copy_gitignored_or_vendor_dirs() {
    // ARRANGE: Lire le Dockerfile
    let dockerfile_path = Path::new("Dockerfile");
    assert!(
        dockerfile_path.exists(),
        "Le fichier Dockerfile doit exister à la racine du projet"
    );

    let content = fs::read_to_string(dockerfile_path).expect("Impossible de lire le Dockerfile");

    // ACT & ASSERT: Vérifier que chaque instruction COPY ne cible pas vendor ou target
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("COPY") && !trimmed.starts_with("COPY --from=") {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 3 {
                let source = parts[1];
                assert!(
                    !source.contains("vendor"),
                    "RÉGRESSION DÉTECTÉE: Le Dockerfile tente de copier '{}'. Le dossier vendor ne doit pas être requis !",
                    source
                );
                assert!(
                    !source.contains("target"),
                    "RÉGRESSION DÉTECTÉE: Le Dockerfile tente de copier '{}' qui est un dossier de compilation hôte !",
                    source
                );
                assert!(
                    !source.contains(".cargo/config.offline.toml"),
                    "Le Dockerfile ne doit pas dépendre de la configuration offline obsolète !"
                );
            }
        }
    }
}

#[test]
fn test_dockerignore_includes_critical_exclusions() {
    // ARRANGE: Lire le .dockerignore
    let dockerignore_path = Path::new(".dockerignore");
    assert!(
        dockerignore_path.exists(),
        "Le fichier .dockerignore doit exister pour garantir des builds hermétiques et légers"
    );

    let content =
        fs::read_to_string(dockerignore_path).expect("Impossible de lire le .dockerignore");

    // ACT: Collecter les règles
    let lines: Vec<&str> = content.lines().map(|l| l.trim()).collect();

    // ASSERT: S'assurer que target, vendor et .git sont ignorés
    let has_target = lines.iter().any(|&l| l == "target" || l == "target/");
    let has_vendor = lines.iter().any(|&l| l == "vendor" || l == "vendor/");
    let has_git = lines.iter().any(|&l| l == ".git" || l == ".git/");

    assert!(has_target, ".dockerignore doit exclure 'target/'");
    assert!(has_vendor, ".dockerignore doit exclure 'vendor/'");
    assert!(has_git, ".dockerignore doit exclure '.git/'");
}

#[test]
fn test_dockerfile_structure_and_healthcheck() {
    // ARRANGE
    let content = fs::read_to_string("Dockerfile").expect("Impossible de lire le Dockerfile");

    // ACT & ASSERT: Multi-stage, Healthcheck, Expose
    assert!(
        content.contains("FROM rust:1.90-bookworm AS builder"),
        "Le Dockerfile doit utiliser rust:1.90-bookworm comme étape builder"
    );
    assert!(
        content.contains("FROM debian:bookworm-slim AS runtime"),
        "Le Dockerfile doit utiliser debian:bookworm-slim comme étape runtime légère"
    );
    assert!(
        content.contains("HEALTHCHECK"),
        "Le Dockerfile doit déclarer un HEALTHCHECK pour la supervision Docker"
    );
    assert!(
        content.contains("EXPOSE 3000"),
        "Le Dockerfile doit exposer le port 3000 du microservice"
    );
}

#[test]
fn test_docker_compose_config_matches_rust_tools() {
    // ARRANGE
    let content =
        fs::read_to_string("docker-compose.yml").expect("Impossible de lire docker-compose.yml");

    // ACT & ASSERT: Vérification des liaisons de service
    assert!(
        content.contains("rust-tools:"),
        "docker-compose.yml doit définir le service rust-tools"
    );
    assert!(
        content.contains("bionic-agent-tools"),
        "docker-compose.yml doit définir le nom de conteneur bionic-agent-tools"
    );
    assert!(
        content.contains("\"3000:3000\""),
        "docker-compose.yml doit mapper le port 3000"
    );
}
