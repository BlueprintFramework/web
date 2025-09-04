use dotenvy::dotenv;
use tracing_subscriber::fmt::writer::MakeWriterExt;

#[derive(Clone)]
pub enum RedisMode {
    Redis,
    Sentinel,
}

#[derive(Clone)]
pub enum MailMode {
    None,
    Smtp {
        host: String,
        port: u16,
        username: Option<String>,
        password: Option<String>,
        use_tls: bool,

        from_address: String,
        from_name: Option<String>,
    },
}

#[derive(Clone)]
pub enum CaptchaProvider {
    None,
    Turnstile {
        site_key: String,
        secret_key: String,
    },
    Recaptcha {
        v3: bool,
        site_key: String,
        secret_key: String,
    },
}

#[derive(Clone)]
pub struct Env {
    pub redis_url: Option<String>,
    pub redis_sentinels: Option<Vec<String>>,
    pub redis_mode: RedisMode,

    pub mail_mode: MailMode,
    pub captcha_provider: CaptchaProvider,

    pub sentry_url: Option<String>,
    pub database_migrate: bool,
    pub database_refresh: bool,
    pub database_url: String,
    pub database_url_primary: Option<String>,

    pub s3_url: String,
    pub s3_path_style: bool,
    pub s3_endpoint: String,
    pub s3_region: String,
    pub s3_bucket: String,
    pub s3_access_key: String,
    pub s3_secret_key: String,

    pub bind: String,
    pub port: u16,

    pub telemetry_ratelimit_per_day: i64,

    pub update_prices: bool,
    pub sxc_token: Option<String>,
    pub bbb_token: Option<String>,

    pub app_debug: bool,
    pub app_log_directory: String,
    pub app_url: String,
    pub server_name: Option<String>,
}

impl Env {
    pub fn parse() -> (tracing_appender::non_blocking::WorkerGuard, Env) {
        dotenv().ok();

        let redis_mode = match std::env::var("REDIS_MODE")
            .unwrap_or("redis".to_string())
            .trim_matches('"')
        {
            "redis" => RedisMode::Redis,
            "sentinel" => RedisMode::Sentinel,
            _ => panic!("Invalid REDIS_MODE"),
        };

        let env = Self {
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

            mail_mode: match std::env::var("MAIL_MODE")
                .unwrap_or("none".to_string())
                .trim_matches('"')
            {
                "none" => MailMode::None,
                "smtp" => MailMode::Smtp {
                    host: std::env::var("MAIL_SMTP_HOST")
                        .expect("MAIL_SMTP_HOST is required")
                        .trim_matches('"')
                        .to_string(),
                    port: std::env::var("MAIL_SMTP_PORT")
                        .expect("MAIL_SMTP_PORT is required")
                        .trim_matches('"')
                        .parse()
                        .unwrap(),
                    username: std::env::var("MAIL_SMTP_USERNAME")
                        .ok()
                        .map(|s| s.trim_matches('"').to_string()),
                    password: std::env::var("MAIL_SMTP_PASSWORD")
                        .ok()
                        .map(|s| s.trim_matches('"').to_string()),
                    use_tls: std::env::var("MAIL_SMTP_USE_TLS")
                        .unwrap_or("true".to_string())
                        .trim_matches('"')
                        .parse()
                        .unwrap(),
                    from_address: std::env::var("MAIL_FROM_ADDRESS")
                        .expect("MAIL_FROM_ADDRESS is required")
                        .trim_matches('"')
                        .to_string(),
                    from_name: std::env::var("MAIL_FROM_NAME")
                        .ok()
                        .map(|s| s.trim_matches('"').to_string()),
                },
                _ => panic!("Invalid MAIL_MODE"),
            },
            captcha_provider: match std::env::var("CAPTCHA_PROVIDER")
                .unwrap_or("none".to_string())
                .trim_matches('"')
            {
                "none" => CaptchaProvider::None,
                "turnstile" => CaptchaProvider::Turnstile {
                    site_key: std::env::var("CAPTCHA_TURNSTILE_SITE_KEY")
                        .expect("CAPTCHA_TURNSTILE_SITE_KEY is required")
                        .trim_matches('"')
                        .to_string(),
                    secret_key: std::env::var("CAPTCHA_TURNSTILE_SECRET_KEY")
                        .expect("CAPTCHA_TURNSTILE_SECRET_KEY is required")
                        .trim_matches('"')
                        .to_string(),
                },
                "recaptcha" => CaptchaProvider::Recaptcha {
                    v3: std::env::var("CAPTCHA_RECAPTCHA_V3")
                        .unwrap_or("false".to_string())
                        .trim_matches('"')
                        .parse()
                        .unwrap(),
                    site_key: std::env::var("CAPTCHA_RECAPTCHA_SITE_KEY")
                        .expect("CAPTCHA_RECAPTCHA_SITE_KEY is required")
                        .trim_matches('"')
                        .to_string(),
                    secret_key: std::env::var("CAPTCHA_RECAPTCHA_SECRET_KEY")
                        .expect("CAPTCHA_RECAPTCHA_SECRET_KEY is required")
                        .trim_matches('"')
                        .to_string(),
                },
                _ => panic!("Invalid CAPTCHA_PROVIDER"),
            },

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

            s3_url: std::env::var("S3_URL")
                .expect("S3_URL is required")
                .trim_matches('"')
                .to_string(),
            s3_path_style: std::env::var("S3_PATH_STYLE")
                .unwrap_or("true".to_string())
                .trim_matches('"')
                .parse()
                .unwrap(),
            s3_endpoint: std::env::var("S3_ENDPOINT")
                .expect("S3_ENDPOINT is required")
                .trim_matches('"')
                .to_string(),
            s3_region: std::env::var("S3_REGION")
                .expect("S3_REGION is required")
                .trim_matches('"')
                .to_string(),
            s3_bucket: std::env::var("S3_BUCKET")
                .expect("S3_BUCKET is required")
                .trim_matches('"')
                .to_string(),
            s3_access_key: std::env::var("S3_ACCESS_KEY")
                .expect("S3_ACCESS_KEY is required")
                .trim_matches('"')
                .to_string(),
            s3_secret_key: std::env::var("S3_SECRET_KEY")
                .expect("S3_SECRET_KEY is required")
                .trim_matches('"')
                .to_string(),

            bind: std::env::var("BIND")
                .unwrap_or("0.0.0.0".to_string())
                .trim_matches('"')
                .to_string(),
            port: std::env::var("PORT")
                .unwrap_or("6969".to_string())
                .parse()
                .unwrap(),

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

            app_debug: std::env::var("APP_DEBUG")
                .unwrap_or("false".to_string())
                .trim_matches('"')
                .parse()
                .unwrap(),
            app_log_directory: std::env::var("APP_LOG_DIRECTORY")
                .unwrap_or("logs".to_string())
                .trim_matches('"')
                .to_string(),
            app_url: std::env::var("APP_URL")
                .expect("APP_URL is required")
                .trim_matches('"')
                .to_string(),
            server_name: std::env::var("SERVER_NAME")
                .ok()
                .map(|s| s.trim_matches('"').to_string()),
        };

        if !std::path::Path::new(&env.app_log_directory).exists() {
            std::fs::create_dir_all(&env.app_log_directory)
                .expect("failed to create log directory");
        }

        let latest_log_path = std::path::Path::new(&env.app_log_directory).join("panel.log");
        let latest_file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&latest_log_path)
            .expect("failed to open latest log file");

        let rolling_appender = tracing_appender::rolling::Builder::new()
            .filename_prefix("panel")
            .filename_suffix("log")
            .max_log_files(30)
            .rotation(tracing_appender::rolling::Rotation::DAILY)
            .build(&env.app_log_directory)
            .expect("failed to create rolling log file appender");

        let (file_appender, _guard) = tracing_appender::non_blocking::NonBlockingBuilder::default()
            .buffered_lines_limit(50)
            .lossy(false)
            .finish(latest_file.and(rolling_appender));

        tracing::subscriber::set_global_default(
            tracing_subscriber::fmt()
                .with_timer(tracing_subscriber::fmt::time::ChronoLocal::rfc_3339())
                .with_writer(std::io::stdout.and(file_appender))
                .with_target(false)
                .with_level(true)
                .with_file(true)
                .with_line_number(true)
                .with_max_level(if env.app_debug {
                    tracing::Level::DEBUG
                } else {
                    tracing::Level::INFO
                })
                .finish(),
        )
        .unwrap();

        (_guard, env)
    }
}
