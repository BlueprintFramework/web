use crate::routes::State;
use colored::Colorize;
use serde::Deserialize;

#[derive(Deserialize)]
struct GithubRelease {
    pub tag_name: String,
}

async fn run_inner(state: &State) -> Result<(), anyhow::Error> {
    let start = std::time::Instant::now();

    let releases = state
        .client()
        .get("https://api.github.com/repos/BlueprintFramework/framework/releases")
        .send()
        .await?
        .json::<Vec<GithubRelease>>()
        .await?;

    *state.github_releases.write().await = releases
        .into_iter()
        .map(|release| release.tag_name)
        .collect();

    tracing::info!(
        "github releases refreshed {}",
        format!(
            "({} releases, {}ms)",
            state.github_releases.read().await.len(),
            start.elapsed().as_millis()
        )
        .bright_black()
    );

    Ok(())
}

pub async fn run(state: State) {
    loop {
        if let Err(err) = run_inner(&state).await {
            tracing::error!("failed to update github releases: {:#?}", err);
            sentry_anyhow::capture_anyhow(&err);
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60)).await;
    }
}
