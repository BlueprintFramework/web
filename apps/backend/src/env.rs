use dotenvy::dotenv;

#[derive(Clone)]
pub enum RedisMode {
    Redis,
    Sentinel,
}

#[derive(Clone)]
pub struct Env {
    pub redis_url: Option<String>,
    pub redis_sentinels: Option<Vec<String>>,
    pub redis_mode: RedisMode,

    pub sentry_url: Option<String>,
    pub database_migrate: bool,
    pub database_refresh: bool,
    pub database_url: String,
    pub database_url_primary: Option<String>,

    pub bind: String,
    pub port: u16,

    pub internal_key: String,
    pub telemetry_ratelimit_per_day: i64,

    pub update_prices: bool,
    pub sxc_token: Option<String>,
    pub bbb_token: Option<String>,

    pub app_url: String,
    pub server_name: Option<String>,
}

impl Env {
    pub fn parse() -> Env {
        dotenv().ok();

        let redis_mode = match std::env::var("REDIS_MODE")
            .unwrap_or("redis".to_string())
            .trim_matches('"')
        {
            "redis" => RedisMode::Redis,
            "sentinel" => RedisMode::Sentinel,
            _ => panic!("Invalid REDIS_MODE"),
        };

        Self {
            redis_url: match redis_mode {
                RedisMode::Redis => Some(
                    std::env::var("REDIS_URL")
                        .expect("REDIS_URL is required")
                        .trim_matches('"')
                        .to_string(),
                ),
                RedisMode::Sentinel => None,
            },
            redis_sentinels: match redis_mode {
                RedisMode::Redis => None,
                RedisMode::Sentinel => Some(
                    std::env::var("REDIS_SENTINELS")
                        .expect("REDIS_SENTINELS is required")
                        .trim_matches('"')
                        .split(',')
                        .map(|s| s.to_string())
                        .collect(),
                ),
            },
            redis_mode,

            sentry_url: std::env::var("SENTRY_URL")
                .ok()
                .map(|s| s.trim_matches('"').to_string()),
            database_migrate: std::env::var("DATABASE_MIGRATE")
                .unwrap_or("true".to_string())
                .trim_matches('"')
                .parse()
                .unwrap(),
            database_refresh: std::env::var("DATABASE_REFRESH")
                .unwrap_or("false".to_string())
                .trim_matches('"')
                .parse()
                .unwrap(),
            database_url: std::env::var("DATABASE_URL")
                .expect("DATABASE_URL is required")
                .trim_matches('"')
                .to_string(),
            database_url_primary: std::env::var("DATABASE_URL_PRIMARY")
                .ok()
                .map(|s| s.trim_matches('"').to_string()),

            bind: std::env::var("BIND")
                .unwrap_or("0.0.0.0".to_string())
                .trim_matches('"')
                .to_string(),
            port: std::env::var("PORT")
                .unwrap_or("6969".to_string())
                .parse()
                .unwrap(),

            internal_key: std::env::var("INTERNAL_KEY")
                .expect("INTERNAL_KEY is required")
                .trim_matches('"')
                .to_string(),
            telemetry_ratelimit_per_day: std::env::var("RATELIMIT_PER_DAY")
                .unwrap_or("2".to_string())
                .parse()
                .unwrap(),

            update_prices: std::env::var("UPDATE_PRICES")
                .unwrap_or("false".to_string())
                .trim_matches('"')
                .parse()
                .unwrap(),
            sxc_token: std::env::var("SXC_TOKEN")
                .ok()
                .map(|s| s.trim_matches('"').to_string()),
            bbb_token: std::env::var("BBB_TOKEN")
                .ok()
                .map(|s| s.trim_matches('"').to_string()),
            app_url: std::env::var("APP_URL")
                .expect("APP_URL is required")
                .trim_matches('"')
                .to_string(),
            server_name: std::env::var("SERVER_NAME")
                .ok()
                .map(|s| s.trim_matches('"').to_string()),
        }
    }
}
