use super::BaseModel;
use serde::{Deserialize, Serialize};
use sqlx::{Row, postgres::PgRow, types::chrono::NaiveDateTime};
use std::collections::BTreeMap;
use utoipa::ToSchema;

#[derive(ToSchema, Serialize, Deserialize, Clone)]
pub struct Author {
    pub id: i32,

    pub name: String,
    pub website: Option<String>,
    pub support: Option<String>,

    pub created: NaiveDateTime,
}

impl BaseModel for Author {
    #[inline]
    fn columns(prefix: Option<&str>, table: Option<&str>) -> BTreeMap<String, String> {
        let table = table.unwrap_or("authors");

        BTreeMap::from([
            (
                format!("{}.id", table),
                format!("{}id", prefix.unwrap_or_default()),
            ),
            (
                format!("{}.name", table),
                format!("{}name", prefix.unwrap_or_default()),
            ),
            (
                format!("{}.website", table),
                format!("{}website", prefix.unwrap_or_default()),
            ),
            (
                format!("{}.support", table),
                format!("{}support", prefix.unwrap_or_default()),
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
            name: row.get(format!("{}name", prefix).as_str()),
            website: row.get(format!("{}website", prefix).as_str()),
            support: row.get(format!("{}support", prefix).as_str()),
            created: row.get(format!("{}created", prefix).as_str()),
        }
    }
}

impl Author {
    #[inline]
    pub async fn by_key(database: &crate::database::Database, key: &str) -> Option<Self> {
        let row = sqlx::query(&format!(
            r#"
            SELECT {}
            FROM authors
            WHERE authors.key = $1
            "#,
            Self::columns_sql(None, None),
        ))
        .bind(key)
        .fetch_optional(database.read())
        .await
        .unwrap();

        row.map(|row| Self::map(None, &row))
    }
}
