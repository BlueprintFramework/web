use colored::Colorize;
use sqlx::postgres::PgPoolOptions;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

type EmptyFuture = Box<dyn Future<Output = ()> + Send>;
pub struct Database {
    write: sqlx::PgPool,
    read: Option<sqlx::PgPool>,

    batch_actions: Arc<Mutex<HashMap<(&'static str, i32), EmptyFuture>>>,
}

impl Database {
    pub async fn new(env: &crate::env::Env) -> Self {
        let start = std::time::Instant::now();

        let instance = Self {
            write: match &env.database_url_primary {
                Some(url) => PgPoolOptions::new()
                    .max_connections(25)
                    .test_before_acquire(false)
                    .connect(url)
                    .await
                    .unwrap(),

                None => PgPoolOptions::new()
                    .max_connections(25)
                    .test_before_acquire(false)
                    .connect(&env.database_url)
                    .await
                    .unwrap(),
            },
            read: if env.database_url_primary.is_some() {
                Some(
                    PgPoolOptions::new()
                        .max_connections(25)
                        .test_before_acquire(false)
                        .connect(&env.database_url)
                        .await
                        .unwrap(),
                )
            } else {
                None
            },
            batch_actions: Arc::new(Mutex::new(HashMap::new())),
        };

        let version: (String,) = sqlx::query_as("SELECT split_part(version(), ' ', 4)")
            .fetch_one(instance.read())
            .await
            .unwrap();

        tracing::info!(
            "{} connected {}",
            "database".bright_cyan(),
            format!(
                "(postgres@{}, {}ms)",
                version.0[..version.0.len() - 1].bright_black(),
                start.elapsed().as_millis()
            )
            .bright_black()
        );

        tokio::spawn({
            let batch_actions = instance.batch_actions.clone();

            async move {
                loop {
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

                    let mut actions = batch_actions.lock().await;
                    for (key, action) in actions.drain() {
                        tracing::debug!(
                            "executing batch action for {}:{}",
                            key.0.bright_cyan(),
                            key.1
                        );
                        Box::into_pin(action).await;
                    }
                }
            }
        });

        if env.database_migrate {
            let writer = instance.write.clone();
            tokio::spawn(async move {
                let start = std::time::Instant::now();

                sqlx::migrate!("../../migrations")
                    .run(&writer)
                    .await
                    .unwrap();

                tracing::info!(
                    "database migrations completed successfully ({}ms)",
                    start.elapsed().as_millis()
                );
            });
        }

        if env.database_refresh {
            let writer = instance.write.clone();
            tokio::spawn(async move {
                loop {
                    tokio::time::sleep(std::time::Duration::from_secs(60 * 30)).await;

                    let start = std::time::Instant::now();

                    match sqlx::query("REFRESH MATERIALIZED VIEW mv_extension_stats")
                        .execute(&writer)
                        .await
                    {
                        Ok(_) => {
                            tracing::info!(
                                "database views refreshed successfully ({}ms)",
                                start.elapsed().as_millis()
                            );
                        }
                        Err(err) => {
                            tracing::error!("failed to refresh database views: {:#?}", err);
                            sentry::capture_error(&err);
                        }
                    }
                }
            });
        }

        instance
    }

    #[inline]
    pub fn write(&self) -> &sqlx::PgPool {
        &self.write
    }

    #[inline]
    pub fn read(&self) -> &sqlx::PgPool {
        self.read.as_ref().unwrap_or(&self.write)
    }

    #[inline]
    pub async fn batch_action<F>(&self, key: &'static str, id: i32, action: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let mut actions = self.batch_actions.lock().await;
        actions.insert((key, id), Box::new(action));
    }
}
