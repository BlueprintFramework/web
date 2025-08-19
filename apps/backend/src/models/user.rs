use super::BaseModel;
use rand::distr::SampleString;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::{Row, postgres::PgRow, types::chrono::NaiveDateTime};
use std::{collections::BTreeMap, sync::LazyLock};
use utoipa::ToSchema;

pub static NAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9_]+$").expect("Failed to compile username regex"));

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub github_id: Option<i64>,

    pub name: String,
    pub email: String,
    pub email_pending: Option<String>,
    pub email_verification: Option<String>,
    pub support: Option<String>,

    pub admin: bool,

    pub created: NaiveDateTime,
}

impl BaseModel for User {
    #[inline]
    fn columns(prefix: Option<&str>, table: Option<&str>) -> BTreeMap<String, String> {
        let table = table.unwrap_or("users");

        BTreeMap::from([
            (
                format!("{table}.id"),
                format!("{}id", prefix.unwrap_or_default()),
            ),
            (
                format!("{table}.github_id"),
                format!("{}github_id", prefix.unwrap_or_default()),
            ),
            (
                format!("{table}.name"),
                format!("{}name", prefix.unwrap_or_default()),
            ),
            (
                format!("{table}.email"),
                format!("{}email", prefix.unwrap_or_default()),
            ),
            (
                format!("{table}.email_pending"),
                format!("{}email_pending", prefix.unwrap_or_default()),
            ),
            (
                format!("{table}.email_verification"),
                format!("{}email_verification", prefix.unwrap_or_default()),
            ),
            (
                format!("{table}.support"),
                format!("{}support", prefix.unwrap_or_default()),
            ),
            (
                format!("{table}.admin"),
                format!("{}admin", prefix.unwrap_or_default()),
            ),
            (
                format!("{table}.created"),
                format!("{}created", prefix.unwrap_or_default()),
            ),
        ])
    }

    #[inline]
    fn map(prefix: Option<&str>, row: &PgRow) -> Self {
        let prefix = prefix.unwrap_or_default();

        Self {
            id: row.get(format!("{prefix}id").as_str()),
            github_id: row.get(format!("{prefix}github_id").as_str()),
            name: row.get(format!("{prefix}name").as_str()),
            email: row.get(format!("{prefix}email").as_str()),
            email_pending: row.get(format!("{prefix}email_pending").as_str()),
            email_verification: row.get(format!("{prefix}email_verification").as_str()),
            support: row.get(format!("{prefix}support").as_str()),
            admin: row.get(format!("{prefix}admin").as_str()),
            created: row.get(format!("{prefix}created").as_str()),
        }
    }
}

impl User {
    pub async fn create(
        database: &crate::database::Database,
        name: &str,
        email: &str,
        password: &str,
    ) -> Result<(Self, String), sqlx::Error> {
        let email_verification = rand::distr::Alphanumeric.sample_string(&mut rand::rng(), 16);

        let row = sqlx::query(&format!(
            r#"
            INSERT INTO users (name, email, email_verification, password)
            VALUES ($1, $2, $3, crypt($4, gen_salt('bf', 8)))
            RETURNING {}
            "#,
            Self::columns_sql(None, None)
        ))
        .bind(name)
        .bind(email)
        .bind(&email_verification)
        .bind(password)
        .fetch_one(database.write())
        .await?;

        Ok((Self::map(None, &row), email_verification))
    }

    pub async fn by_email(
        database: &crate::database::Database,
        email: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        let row = sqlx::query(&format!(
            r#"
            SELECT {}
            FROM users
            WHERE email = $1
            "#,
            Self::columns_sql(None, None)
        ))
        .bind(email)
        .fetch_optional(database.read())
        .await?;

        Ok(row.map(|row| Self::map(None, &row)))
    }

    pub async fn by_session(
        database: &crate::database::Database,
        session: &str,
    ) -> Result<Option<(Self, super::user_session::UserSession)>, sqlx::Error> {
        let (key_id, key) = match session.split_once(':') {
            Some((id, key)) => (id, key),
            None => return Ok(None),
        };

        let row = sqlx::query(&format!(
            r#"
            SELECT {}, {}
            FROM users
            JOIN user_sessions ON user_sessions.user_id = users.id
            WHERE user_sessions.key_id = $1 AND user_sessions.key = crypt($2, user_sessions.key)
            "#,
            Self::columns_sql(None, None),
            super::user_session::UserSession::columns_sql(Some("session_"), None)
        ))
        .bind(key_id)
        .bind(key)
        .fetch_optional(database.read())
        .await?;

        Ok(row.map(|row| {
            (
                Self::map(None, &row),
                super::user_session::UserSession::map(Some("session_"), &row),
            )
        }))
    }

    pub async fn by_email_password(
        database: &crate::database::Database,
        email: &str,
        password: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        let row = sqlx::query(&format!(
            r#"
            SELECT {}
            FROM users
            WHERE users.email = $1 AND users.password = crypt($2, users.password)
            "#,
            Self::columns_sql(None, None)
        ))
        .bind(email)
        .bind(password)
        .fetch_optional(database.read())
        .await?;

        Ok(row.map(|row| Self::map(None, &row)))
    }

    pub async fn update_password(
        &self,
        database: &crate::database::Database,
        password: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE users
            SET password = crypt($2, gen_salt('bf'))
            WHERE users.id = $1
            "#,
        )
        .bind(self.id)
        .bind(password)
        .execute(database.write())
        .await?;

        Ok(())
    }

    #[inline]
    pub fn into_api_object(self) -> ApiUser {
        ApiUser {
            id: self.id,
            name: self.name,
            support: self.support,
            admin: self.admin,
            created: self.created.and_utc(),
        }
    }

    #[inline]
    pub fn into_api_full_object(self) -> ApiFullUser {
        ApiFullUser {
            id: self.id,
            name: self.name,
            email: self.email,
            email_pending: self.email_pending,
            support: self.support,
            admin: self.admin,
            created: self.created.and_utc(),
        }
    }
}

#[derive(ToSchema, Serialize)]
#[schema(title = "FullUser")]
pub struct ApiFullUser {
    pub id: i32,

    pub name: String,
    pub email: String,
    pub email_pending: Option<String>,
    pub support: Option<String>,

    pub admin: bool,

    pub created: chrono::DateTime<chrono::Utc>,
}

#[derive(ToSchema, Serialize)]
#[schema(title = "User")]
pub struct ApiUser {
    pub id: i32,

    pub name: String,
    pub support: Option<String>,

    pub admin: bool,

    pub created: chrono::DateTime<chrono::Utc>,
}
