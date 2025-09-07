use chrono::NaiveDateTime;
use colored::Colorize;
use rustis::commands::{ExpireOption, GenericCommands, StringCommands};
use serde::{Deserialize, Serialize};
use sha2::Digest;
use sqlx::types::Uuid;
use std::{
    collections::{HashMap, HashSet},
    net::IpAddr,
    sync::Arc,
};
use tokio::sync::Mutex;

nestify::nest! {
    #[derive(Deserialize, Serialize)] pub struct TelemetryData {
        #[serde(skip_serializing)]
        id: Uuid,

        #[serde(skip_serializing)]
        telemetry_version: u16,

        blueprint: #[derive(Deserialize, Serialize)] struct Blueprint {
            version: String,
            docker: bool,

            flags: #[derive(Deserialize, Serialize)] struct Flags {
                disable_attribution: bool,
                is_developer: bool,
                show_in_sidebar: bool
            },

            extensions: Vec<#[derive(Deserialize, Serialize)] struct Extension {
                identifier: String,
                version: String,
                author: Option<String>,
                target: String
            }>
        },
        panel: #[derive(Deserialize, Serialize)] struct Panel {
            version: String,
            #[serde(rename = "phpVersion")]
            php_version: String,

            drivers: #[derive(Deserialize, Serialize)] struct Drivers {
                backup: #[derive(Deserialize, Serialize)] struct Backup {
                    r#type: String
                },

                cache: #[derive(Deserialize, Serialize)] struct Cache {
                    r#type: String
                },

                database: #[derive(Deserialize, Serialize)] struct Database {
                    r#type: String,
                    version: String
                }
            }
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Telemetry {
    panel_id: Uuid,
    telemetry_version: i16,
    ip: String,
    continent: Option<String>,
    country: Option<String>,
    data: TelemetryData,
    created: NaiveDateTime,
}

pub struct TelemetryLogger {
    processing: Mutex<Vec<Telemetry>>,
    database: Arc<crate::database::Database>,
    cache: Arc<crate::cache::Cache>,
    env: Arc<crate::env::Env>,

    client: reqwest::Client,
}

impl TelemetryLogger {
    pub fn new(
        database: Arc<crate::database::Database>,
        cache: Arc<crate::cache::Cache>,
        env: Arc<crate::env::Env>,
    ) -> Self {
        Self {
            processing: Mutex::new(Vec::new()),
            database,
            cache,
            env,

            client: reqwest::Client::builder()
                .user_agent("Blueprint API https://blueprint.zip")
                .build()
                .unwrap(),
        }
    }

    #[inline]
    pub async fn log(&self, ip: IpAddr, telemetry: TelemetryData) -> Option<()> {
        let mut processing = self.processing.lock().await;

        let ratelimit_key = format!("blueprint_api::ratelimit::{ip}");

        let count = self.cache.client.incr(&ratelimit_key).await.ok()?;
        if count == 1 {
            self.cache
                .client
                .expire(&ratelimit_key, 86400, ExpireOption::None)
                .await
                .ok()?;
        }

        if count > self.env.telemetry_ratelimit_per_day {
            return None;
        }

        let data = Telemetry {
            panel_id: telemetry.id,
            telemetry_version: telemetry.telemetry_version as i16,
            ip: ip.to_string(),
            continent: None,
            country: None,
            data: telemetry,
            created: chrono::Utc::now().naive_utc(),
        };

        processing.push(data);

        Some(())
    }

    #[inline]
    async fn lookup_ips(
        &self,
        ips: &[&str],
    ) -> Result<HashMap<String, [String; 2]>, Box<dyn std::error::Error>> {
        let mut result = HashMap::new();

        let data = self
            .client
            .post("http://ip-api.com/batch")
            .header("Content-Type", "application/json")
            .json(
                &ips.iter()
                    .map(|ip| {
                        serde_json::json!({
                            "query": ip,
                            "fields": "continentCode,countryCode,query"
                        })
                    })
                    .collect::<HashSet<_>>(),
            )
            .send()
            .await?
            .json::<Vec<IpApiResponse>>()
            .await?;

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct IpApiResponse {
            continent_code: String,
            country_code: String,
            query: String,
        }

        for entry in data {
            result.insert(entry.query, [entry.continent_code, entry.country_code]);
        }

        Ok(result)
    }

    pub async fn process(&self) -> Result<(), anyhow::Error> {
        let mut processing = self.processing.lock().await;
        let length = processing.len();

        let mut telemetry = processing
            .splice(0..std::cmp::min(30, length), Vec::new())
            .collect::<Vec<_>>();

        if telemetry.is_empty() {
            return Ok(());
        }

        let ips = self
            .lookup_ips(
                telemetry
                    .iter()
                    .map(|t| t.ip.as_str())
                    .collect::<Vec<_>>()
                    .as_slice(),
            )
            .await
            .unwrap_or_default();

        for t in telemetry.iter_mut() {
            if let Some([continent, country]) = ips.get(&t.ip) {
                t.continent = Some(continent.clone());
                t.country = Some(country.clone());
            }

            t.ip = format!("{:x}", sha2::Sha256::digest(t.ip.as_bytes()));
        }

        let panels = telemetry
            .iter()
            .map(|t| t.panel_id)
            .collect::<std::collections::HashSet<_>>();

        for id in panels {
            match sqlx::query!(
                "INSERT INTO telemetry_panels (id)
                VALUES ($1)
                ON CONFLICT (id) DO UPDATE SET last_update = GREATEST(
                    telemetry_panels.last_update,
                    (SELECT created FROM telemetry_data WHERE panel_id = $1 ORDER BY created DESC LIMIT 1)
                )",
                id
            )
            .execute(self.database.write())
            .await {
                Ok(_) => {},
                Err(err) => {
                    tracing::error!("failed to insert telemetry panel data: {:#?}", err);

                    self.processing
                        .lock()
                        .await
                        .append(&mut telemetry);

                    return Err(err.into());
                }
            }
        }

        for t in telemetry.iter() {
            match sqlx::query!(
                "INSERT INTO telemetry_data (panel_id, telemetry_version, ip, continent, country, data, created)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                ON CONFLICT DO NOTHING",
                t.panel_id,
                t.telemetry_version,
                t.ip,
                t.continent,
                t.country,
                serde_json::to_value(&t.data).unwrap(),
                t.created
            )
            .execute(self.database.write())
            .await {
                Ok(_) => {},
                Err(err) => {
                    tracing::error!("failed to insert telemetry data: {:#?}", err);

                    self.processing
                        .lock()
                        .await
                        .append(&mut telemetry);

                    return Err(err.into());
                }
            }
        }

        tracing::info!(
            "processed {} telemetry entries",
            telemetry.len().to_string().cyan()
        );

        Ok(())
    }
}
