use crate::{env::RedisMode, response::ApiResponse};
use axum::http::StatusCode;
use colored::Colorize;
use rustis::{
    client::Client,
    commands::{GenericCommands, SetExpiration, StringCommands},
    resp::{BulkString, cmd},
};
use serde::{Serialize, de::DeserializeOwned};
use std::future::Future;

pub struct Cache {
    pub client: Client,
}

impl Cache {
    pub async fn new(env: &crate::env::Env) -> Self {
        let start = std::time::Instant::now();

        let instance = Self {
            client: match env.redis_mode {
                RedisMode::Redis => Client::connect(env.redis_url.as_ref().unwrap().clone())
                    .await
                    .unwrap(),
                RedisMode::Sentinel => Client::connect(
                    format!(
                        "redis-sentinel://{}/mymaster/0",
                        env.redis_sentinels.as_ref().unwrap().clone().join(",")
                    )
                    .as_str(),
                )
                .await
                .unwrap(),
            },
        };

        let version = String::from_utf8(
            instance
                .client
                .send(cmd("INFO"), None)
                .await
                .unwrap()
                .as_bytes()
                .into(),
        )
        .unwrap()
        .lines()
        .find(|line| line.starts_with("redis_version:"))
        .unwrap()
        .split(':')
        .collect::<Vec<&str>>()[1]
            .to_string();

        tracing::info!(
            "{} connected {}",
            "cache".bright_yellow(),
            format!("(redis@{}, {}ms)", version, start.elapsed().as_millis()).bright_black()
        );

        instance
    }

    pub async fn ratelimit(
        &self,
        limit_identifier: impl AsRef<str>,
        limit: u64,
        limit_window: u64,
        client: impl AsRef<str>,
    ) -> Result<(), ApiResponse> {
        let key = format!(
            "ratelimit::{}::{}",
            limit_identifier.as_ref(),
            client.as_ref()
        );

        let now = chrono::Utc::now().timestamp();
        let expiry = self.client.expiretime(&key).await.unwrap_or_default();
        let expire_unix: u64 = if expiry > now + 2 {
            expiry as u64
        } else {
            now as u64 + limit_window
        };

        let limit_used = self.client.get::<u64>(&key).await.unwrap_or_default() + 1;
        self.client
            .set_with_options(key, limit_used, None, SetExpiration::Exat(expire_unix))
            .await?;

        if limit_used >= limit {
            return Err(ApiResponse::error(&format!(
                "you are ratelimited, retry in {}s",
                expiry - now
            ))
            .with_status(StatusCode::TOO_MANY_REQUESTS));
        }

        Ok(())
    }

    #[tracing::instrument(skip(self, fn_compute))]
    pub async fn cached<T, F, Fut>(
        &self,
        key: &str,
        ttl: u64,
        fn_compute: F,
    ) -> Result<T, anyhow::Error>
    where
        T: Serialize + DeserializeOwned,
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<T, anyhow::Error>>,
    {
        let cached_value: Option<BulkString> = self.client.get(key).await?;

        match cached_value.and_then(|v| rmp_serde::from_slice::<T>(&v).ok()) {
            Some(value) => Ok(value),
            None => {
                let result = match fn_compute().await {
                    Ok(result) => result,
                    Err(err) => return Err(err),
                };

                let serialized = rmp_serde::to_vec(&result)?;
                self.client
                    .set_with_options(key, serialized, None, SetExpiration::Ex(ttl))
                    .await?;

                Ok(result)
            }
        }
    }

    #[inline]
    pub async fn clear_extension(
        &self,
        extension: &crate::models::extension::Extension,
    ) -> Result<(), rustis::Error> {
        let (mut id_keys, mut identifier_keys): (Vec<String>, Vec<String>) = tokio::try_join!(
            self.client.keys(format!("extensions::{}*", extension.id)),
            self.client
                .keys(format!("extensions::{}*", extension.identifier))
        )?;

        if !id_keys.is_empty() || !identifier_keys.is_empty() {
            id_keys.append(&mut identifier_keys);

            self.client.del(id_keys).await?;
        }

        Ok(())
    }
}
