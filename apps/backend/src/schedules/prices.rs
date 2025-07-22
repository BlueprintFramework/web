use crate::{
    models::extension::{Extension, ExtensionPlatform, ExtensionVersion},
    routes::State,
};
use colored::Colorize;
use serde::Deserialize;

#[derive(Deserialize)]
struct SxcProduct {
    id: u32,
    price: f64,
    currency: String,
    url: String,
    rating_avg: Option<f64>,
    review_count: Option<u32>,
}

#[derive(Deserialize)]
struct SxcProductVersion {
    name: String,
    created_at: String,
    downloads_count: u32,
}

#[derive(Deserialize)]
struct BbbProduct {
    price: f64,
    currency: String,
    review_average: Option<f64>,
    review_count: u32,
}

#[derive(Deserialize)]
struct BbbProductVersion {
    name: String,
    release_date: i64,
    download_count: u32,
}

#[derive(Deserialize)]
struct GithubRelease {
    name: String,
    published_at: String,
    assets: Vec<GithubAsset>,
}

#[derive(Deserialize)]
struct GithubAsset {
    name: String,
    download_count: u32,
}

#[inline]
fn clean_version_name(name: &str) -> String {
    let name = name.trim().to_lowercase();

    name.trim_start_matches("v.")
        .trim_start_matches("v")
        .to_string()
}

async fn run_inner(state: State) -> Result<(), Box<dyn std::error::Error>> {
    let start = std::time::Instant::now();

    let mut count = 0;
    let mut extensions = Extension::all(&state.database).await;
    let mut sxc_products: Vec<SxcProduct> = vec![];

    if let Some(sxc_token) = &state.env.sxc_token {
        sxc_products = serde_json::from_value(
            state
                .client()
                .get("https://www.sourcexchange.net/api/products/blueprint")
                .header("Authorization", format!("Bearer {}", sxc_token))
                .send()
                .await?
                .json::<serde_json::Value>()
                .await
                .unwrap_or_default()
                .get("data")
                .cloned()
                .unwrap_or_default(),
        )
        .unwrap_or_default()
    }

    for extension in extensions.iter_mut() {
        count += 1;

        crate::logger::log(
            crate::logger::LoggerLevel::Info,
            format!(
                "updating extension prices of {}",
                extension.name.bright_cyan()
            ),
        );

        for version in extension.versions.iter_mut() {
            version.downloads = 0;
        }

        if let Some(key) = extension.platforms.get_mut("SOURCEXCHANGE") {
            if let Some(sxc_product) = sxc_products.iter().find(|product| product.url == key.url) {
                match state
                    .client()
                    .get(format!(
                        "https://www.sourcexchange.net/api/products/{}/releases",
                        sxc_product.id
                    ))
                    .header(
                        "Authorization",
                        format!("Bearer {}", state.env.sxc_token.as_ref().unwrap()),
                    )
                    .send()
                    .await?
                    .json::<Vec<SxcProductVersion>>()
                    .await
                {
                    Ok(versions) => {
                        let mut versions: Vec<ExtensionVersion> = versions
                            .into_iter()
                            .map(|version| ExtensionVersion {
                                name: clean_version_name(&version.name),
                                downloads: version.downloads_count,
                                created: chrono::NaiveDateTime::parse_from_str(
                                    &version.created_at,
                                    "%Y-%m-%dT%H:%M:%S%.fZ",
                                )
                                .unwrap_or_default(),
                            })
                            .collect();

                        if versions.len() > extension.versions.len() {
                            versions.sort_unstable_by(|a, b| a.created.cmp(&b.created).reverse());
                            versions.dedup_by(|a, b| a.name == b.name);

                            extension.versions = versions;
                        } else {
                            for version in extension.versions.iter_mut() {
                                if let Some(sxc_version) = versions
                                    .iter()
                                    .find(|sxc_version| sxc_version.name == version.name)
                                {
                                    version.downloads += sxc_version.downloads;
                                }
                            }
                        }

                        *key = ExtensionPlatform {
                            url: key.url.clone(),
                            price: sxc_product.price,
                            currency: sxc_product.currency.clone(),
                            reviews: sxc_product.review_count,
                            rating: sxc_product.rating_avg,
                        };
                    }
                    Err(err) => {
                        crate::logger::log(
                            crate::logger::LoggerLevel::Error,
                            format!(
                                "failed to get sourcexchange versions for {}:\n{:#?}",
                                extension.name.bright_cyan(),
                                err
                            ),
                        );
                    }
                };
            }
        }

        if let Some(bbb_token) = &state.env.bbb_token {
            if let Some(key) = extension.platforms.get_mut("BUILTBYBIT") {
                let product_id: u32 = key
                    .url
                    .split('.')
                    .next_back()
                    .ok_or_else(|| {
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            format!("Invalid BBB URL: {}", key.url.bright_cyan()),
                        )
                    })?
                    .trim_end_matches(|c: char| !c.is_ascii_digit())
                    .parse()
                    .unwrap_or_default();

                #[derive(Deserialize)]
                struct BbbProductResponse {
                    #[serde(rename = "data")]
                    product: BbbProduct,
                }

                #[derive(Deserialize)]
                struct BbbProductVersionResponse {
                    #[serde(rename = "data")]
                    versions: Vec<BbbProductVersion>,
                }

                match state
                    .client()
                    .get(format!(
                        "https://api.builtbybit.com/v1/resources/{}",
                        product_id
                    ))
                    .header("Authorization", format!("Private {}", bbb_token))
                    .send()
                    .await?
                    .json::<BbbProductResponse>()
                    .await
                {
                    Ok(BbbProductResponse { product }) => {
                        match state
                            .client()
                            .get(format!(
                                "https://api.builtbybit.com/v1/resources/{}/versions",
                                product_id
                            ))
                            .header("Authorization", format!("Private {}", bbb_token))
                            .send()
                            .await?
                            .json::<BbbProductVersionResponse>()
                            .await
                        {
                            Ok(BbbProductVersionResponse { versions }) => {
                                let mut versions: Vec<ExtensionVersion> = versions
                                    .into_iter()
                                    .map(|version| ExtensionVersion {
                                        name: clean_version_name(&version.name),
                                        downloads: version.download_count,
                                        created: chrono::DateTime::from_timestamp(
                                            version.release_date,
                                            0,
                                        )
                                        .unwrap_or_default()
                                        .naive_utc(),
                                    })
                                    .collect();

                                if versions.len() > extension.versions.len() {
                                    versions.sort_unstable_by(|a, b| {
                                        a.created.cmp(&b.created).reverse()
                                    });
                                    versions.dedup_by(|a, b| a.name == b.name);

                                    extension.versions = versions;
                                } else {
                                    for version in extension.versions.iter_mut() {
                                        if let Some(bbb_version) = versions
                                            .iter()
                                            .find(|bbb_version| bbb_version.name == version.name)
                                        {
                                            version.downloads += bbb_version.downloads;
                                        }
                                    }
                                }

                                *key = ExtensionPlatform {
                                    url: key.url.clone(),
                                    price: product.price,
                                    currency: product.currency.clone(),
                                    reviews: Some(product.review_count),
                                    rating: product.review_average,
                                };
                            }
                            Err(err) => {
                                crate::logger::log(
                                    crate::logger::LoggerLevel::Error,
                                    format!(
                                        "failed to get builtbybit versions for {}:\n{:#?}",
                                        extension.name.bright_cyan(),
                                        err
                                    ),
                                );
                            }
                        }
                    }
                    Err(err) => {
                        crate::logger::log(
                            crate::logger::LoggerLevel::Error,
                            format!(
                                "failed to get builtbybit product for {} (#{}):\n{:#?}",
                                extension.name.bright_cyan(),
                                product_id,
                                err
                            ),
                        );
                    }
                };
            }
        }

        if let Some(key) = extension.platforms.get_mut("GITHUB") {
            let repo = key.url.split('/').collect::<Vec<_>>()[3..5].join("/");

            match state
                .client()
                .get(format!("https://api.github.com/repos/{}/releases", repo))
                .send()
                .await?
                .json::<Vec<GithubRelease>>()
                .await
            {
                Ok(releases) => {
                    let mut versions: Vec<ExtensionVersion> = releases
                        .into_iter()
                        .flat_map(|release| {
                            release
                                .assets
                                .into_iter()
                                .filter(|asset| asset.name.ends_with(".blueprint"))
                                .map(move |asset| ExtensionVersion {
                                    name: clean_version_name(&release.name),
                                    downloads: asset.download_count,
                                    created: chrono::NaiveDateTime::parse_from_str(
                                        &release.published_at,
                                        "%Y-%m-%dT%H:%M:%S%.fZ",
                                    )
                                    .unwrap_or_default(),
                                })
                        })
                        .collect();

                    if versions.len() > extension.versions.len() {
                        versions.sort_unstable_by(|a, b| a.created.cmp(&b.created).reverse());
                        versions.dedup_by(|a, b| a.name == b.name);

                        extension.versions = versions;
                    } else {
                        for version in extension.versions.iter_mut() {
                            if let Some(github_version) = versions
                                .iter()
                                .find(|github_version| github_version.name == version.name)
                            {
                                version.downloads += github_version.downloads;
                            }
                        }
                    }

                    *key = ExtensionPlatform {
                        url: key.url.clone(),
                        price: 0.0,
                        currency: "USD".to_string(),
                        reviews: Some(0),
                        rating: None,
                    };
                }
                Err(err) => {
                    crate::logger::log(
                        crate::logger::LoggerLevel::Error,
                        format!(
                            "failed to get github releases for {}:\n{:#?}",
                            extension.name.bright_cyan(),
                            err
                        ),
                    );
                }
            }
        }

        sqlx::query!(
            "UPDATE extensions SET platforms = $1, versions = $2 WHERE id = $3",
            serde_json::to_value(&extension.platforms)?,
            serde_json::to_value(&extension.versions)?,
            extension.id
        )
        .execute(state.database.write())
        .await?;
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    crate::logger::log(
        crate::logger::LoggerLevel::Info,
        format!(
            "product prices refreshed {}",
            format!("({} prices, {}ms)", count, start.elapsed().as_millis()).bright_black()
        ),
    );

    Ok(())
}

pub async fn run(state: State) {
    loop {
        if let Err(err) = run_inner(state.clone()).await {
            sentry::capture_error(err.as_ref());

            crate::logger::log(
                crate::logger::LoggerLevel::Error,
                format!(
                    "{} {}",
                    "failed to update extension prices".red(),
                    err.to_string().red()
                ),
            );
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60 * 2)).await;
    }
}
