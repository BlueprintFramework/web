use crate::routes::State;
use serde::Deserialize;

#[derive(Deserialize)]
struct GithubRelease {
    pub tag_name: String,
}

async fn run_blueprint_inner(state: &State) -> Result<(), anyhow::Error> {
    let start = std::time::Instant::now();

    let releases = state
        .client
        .get("https://api.github.com/repos/BlueprintFramework/framework/releases")
        .send()
        .await?
        .json::<Vec<GithubRelease>>()
        .await?;

    *state.blueprint_github_releases.write().await = releases
        .into_iter()
        .map(|release| release.tag_name)
        .collect();

    tracing::info!(
        "blueprint github releases refreshed ({} releases, {}ms)",
        state.blueprint_github_releases.read().await.len(),
        start.elapsed().as_millis()
    );

    Ok(())
}

async fn run_hydrodactyl_inner(state: &State) -> Result<(), anyhow::Error> {
    let start = std::time::Instant::now();

    let releases = state
        .client
        .get("https://api.github.com/repos/BlueprintFramework/Hydrodactyl/releases")
        .send()
        .await?
        .json::<Vec<GithubRelease>>()
        .await?;

    *state.hydrodactyl_github_releases.write().await = releases
        .into_iter()
        .map(|release| release.tag_name)
        .collect();

    tracing::info!(
        "hydrodactyl github releases refreshed ({} releases, {}ms)",
        state.hydrodactyl_github_releases.read().await.len(),
        start.elapsed().as_millis()
    );

    Ok(())
}

pub async fn run(state: State) {
    loop {
        if let Err(err) = run_blueprint_inner(&state).await {
            tracing::error!("failed to update blueprint github releases: {:#?}", err);
            sentry_anyhow::capture_anyhow(&err);
        }

        if let Err(err) = run_hydrodactyl_inner(&state).await {
            tracing::error!("failed to update hydrodactyl github releases: {:#?}", err);
            sentry_anyhow::capture_anyhow(&err);
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60)).await;
    }
}
