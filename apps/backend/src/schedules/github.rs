use crate::routes::State;
use colored::Colorize;
use serde::Deserialize;

#[derive(Deserialize)]
struct GithubRelease {
    pub tag_name: String,
}

async fn run_inner(state: State) -> Result<(), Box<dyn std::error::Error>> {
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

    crate::logger::log(
        crate::logger::LoggerLevel::Info,
        format!(
            "{} releases refreshed {}",
            "github".black(),
            format!(
                "({} releases, {}ms)",
                state.github_releases.read().await.len(),
                start.elapsed().as_millis()
            )
            .bright_black()
        ),
    );

    Ok(())
}

pub async fn run(state: State) {
    loop {
        if let Err(err) = run_inner(state.clone()).await {
            crate::logger::log(
                crate::logger::LoggerLevel::Error,
                format!(
                    "{} {}",
                    "github".black(),
                    format!("failed to fetch releases: {}", err).bright_red()
                ),
            );
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60)).await;
    }
}
