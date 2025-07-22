use super::BaseModel;
use serde::{Deserialize, Serialize};
use sqlx::{Row, postgres::PgRow, types::chrono::NaiveDateTime};
use std::collections::BTreeMap;
use utoipa::ToSchema;

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
                format!("{}.id", table),
                format!("{}id", prefix.unwrap_or_default()),
            ),
            (
                format!("{}.github_id", table),
                format!("{}github_id", prefix.unwrap_or_default()),
            ),
            (
                format!("{}.name", table),
                format!("{}name", prefix.unwrap_or_default()),
            ),
            (
                format!("{}.email", table),
                format!("{}email", prefix.unwrap_or_default()),
            ),
            (
                format!("{}.email_pending", table),
                format!("{}email_pending", prefix.unwrap_or_default()),
            ),
            (
                format!("{}.email_verification", table),
                format!("{}email_verification", prefix.unwrap_or_default()),
            ),
            (
                format!("{}.support", table),
                format!("{}support", prefix.unwrap_or_default()),
            ),
            (
                format!("{}.admin", table),
                format!("{}admin", prefix.unwrap_or_default()),
            ),
            (
                format!("{}.created", table),
                format!("{}created", prefix.unwrap_or_default()),
            ),
        ])
    }

    #[inline]
    fn map(prefix: Option<&str>, row: &PgRow) -> Self {
        let prefix = prefix.unwrap_or_default();

        Self {
            id: row.get(format!("{}id", prefix).as_str()),
            github_id: row.get(format!("{}github_id", prefix).as_str()),
            name: row.get(format!("{}name", prefix).as_str()),
            email: row.get(format!("{}email", prefix).as_str()),
            email_pending: row.get(format!("{}email_pending", prefix).as_str()),
            email_verification: row.get(format!("{}email_verification", prefix).as_str()),
            support: row.get(format!("{}support", prefix).as_str()),
            admin: row.get(format!("{}admin", prefix).as_str()),
            created: row.get(format!("{}created", prefix).as_str()),
        }
    }
}

impl User {
    pub async fn create(
        database: &crate::database::Database,
        name: &str,
        email: &str,
        password: &str,
    ) -> Result<Self, sqlx::Error> {
        let row = sqlx::query(&format!(
            r#"
            INSERT INTO users (name, email, password)
            VALUES ($1, $2, crypt($3, gen_salt('bf', 8)))
            RETURNING {}
            "#,
            Self::columns_sql(None, None)
        ))
        .bind(name)
        .bind(email)
        .bind(password)
        .fetch_one(database.write())
        .await?;

        Ok(Self::map(None, &row))
    }

    pub async fn by_session(
        database: &crate::database::Database,
        session: &str,
    ) -> Option<(Self, super::user_session::UserSession)> {
        let (key_id, key) = session.split_once(':')?;

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
        .await
        .unwrap();

        row.map(|row| {
            (
                Self::map(None, &row),
                super::user_session::UserSession::map(Some("session_"), &row),
            )
        })
    }

    pub async fn by_email_password(
        database: &crate::database::Database,
        email: &str,
        password: &str,
    ) -> Option<Self> {
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
        .await
        .unwrap();

        row.map(|row| Self::map(None, &row))
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
