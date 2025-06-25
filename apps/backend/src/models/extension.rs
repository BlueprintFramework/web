use super::{BaseModel, author::Author};
use serde::{Deserialize, Serialize};
use sqlx::{Row, postgres::PgRow, prelude::Type, types::chrono::NaiveDateTime};
use std::collections::BTreeMap;
use utoipa::ToSchema;

#[derive(ToSchema, Serialize, Deserialize, Type)]
#[serde(rename_all = "UPPERCASE")]
#[schema(rename_all = "UPPERCASE")]
#[sqlx(type_name = "extension_type", rename_all = "UPPERCASE")]
pub enum ExtensionType {
    Theme,
    Extension,
}

#[derive(ToSchema, Serialize, Deserialize)]
pub struct ExtensionVersion {
    pub name: String,
    pub downloads: u32,

    pub created: NaiveDateTime,
}

#[derive(ToSchema, Serialize, Deserialize)]
pub struct ExtensionPlatform {
    pub url: String,
    pub price: f64,
    pub currency: String,

    pub reviews: Option<u32>,
    pub rating: Option<f64>,
}

#[derive(ToSchema, Serialize, Deserialize)]
pub struct ExtensionStats {
    pub panels: i64,
}

#[derive(ToSchema, Serialize, Deserialize)]
pub struct Extension {
    pub id: i32,
    pub author: Author,

    pub r#type: ExtensionType,

    pub name: String,
    pub identifier: String,
    pub summary: String,

    #[schema(inline)]
    pub platforms: BTreeMap<String, ExtensionPlatform>,
    #[schema(inline)]
    pub versions: Vec<ExtensionVersion>,

    pub keywords: Vec<String>,
    pub banner: String,

    pub created: NaiveDateTime,

    #[schema(inline)]
    pub stats: ExtensionStats,
}

impl BaseModel for Extension {
    #[inline]
    fn columns(prefix: Option<&str>, table: Option<&str>) -> BTreeMap<String, String> {
        let table = table.unwrap_or("extensions");

        let mut columns = BTreeMap::from([
            (
                format!("{}.id", table),
                format!("{}id", prefix.unwrap_or_default()),
            ),
            (
                format!("{}.type", table),
                format!("{}type", prefix.unwrap_or_default()),
            ),
            (
                format!("{}.name", table),
                format!("{}name", prefix.unwrap_or_default()),
            ),
            (
                format!("{}.identifier", table),
                format!("{}identifier", prefix.unwrap_or_default()),
            ),
            (
                format!("{}.summary", table),
                format!("{}summary", prefix.unwrap_or_default()),
            ),
            (
                format!("{}.platforms", table),
                format!("{}platforms", prefix.unwrap_or_default()),
            ),
            (
                format!("{}.versions", table),
                format!("{}versions", prefix.unwrap_or_default()),
            ),
            (
                format!("{}.keywords", table),
                format!("{}keywords", prefix.unwrap_or_default()),
            ),
            (
                format!("{}.banner", table),
                format!("{}banner", prefix.unwrap_or_default()),
            ),
            (
                "mv_extension_stats.stats".to_string(),
                format!("{}stats", prefix.unwrap_or_default()),
            ),
            (
                format!("{}.created", table),
                format!("{}created", prefix.unwrap_or_default()),
            ),
        ]);

        columns.extend(Author::columns(Some("author_"), None));

        columns
    }

    #[inline]
    fn map(prefix: Option<&str>, row: &PgRow) -> Self {
        let prefix = prefix.unwrap_or_default();

        Self {
            id: row.get(format!("{}id", prefix).as_str()),
            author: Author::map(Some("author_"), row),
            r#type: row.get(format!("{}type", prefix).as_str()),
            name: row.get(format!("{}name", prefix).as_str()),
            identifier: row.get(format!("{}identifier", prefix).as_str()),
            summary: row.get(format!("{}summary", prefix).as_str()),
            platforms: serde_json::from_value(row.get(format!("{}platforms", prefix).as_str()))
                .unwrap_or_default(),
            versions: serde_json::from_value(row.get(format!("{}versions", prefix).as_str()))
                .unwrap_or_default(),
            keywords: row.get(format!("{}keywords", prefix).as_str()),
            banner: row.get(format!("{}banner", prefix).as_str()),
            stats: serde_json::from_value(row.get(format!("{}stats", prefix).as_str())).unwrap(),
            created: row.get(format!("{}created", prefix).as_str()),
        }
    }
}

impl Extension {
    #[inline]
    pub fn versions(&self) -> Vec<&String> {
        let mut versions: Vec<&String> = Vec::new();

        for version in self.versions.iter() {
            versions.push(&version.name);
        }

        versions
    }

    #[inline]
    pub async fn all(database: &crate::database::Database) -> Vec<Self> {
        sqlx::query(&format!(
            r#"
            SELECT {}
            FROM extensions
            JOIN authors ON extensions.author_id = authors.id
            LEFT JOIN mv_extension_stats ON extensions.id = mv_extension_stats.id
            WHERE
                NOT pending
                AND NOT hidden
            ORDER BY id ASC
            "#,
            Self::columns_sql(None, None)
        ))
        .fetch_all(database.read())
        .await
        .unwrap()
        .into_iter()
        .map(|row| Self::map(None, &row))
        .collect()
    }

    #[inline]
    pub async fn by_identifier(
        database: &crate::database::Database,
        identifier: &str,
    ) -> Option<Self> {
        let data = sqlx::query(&format!(
            r#"
            SELECT {}
            FROM extensions
            JOIN authors ON extensions.author_id = authors.id
            LEFT JOIN mv_extension_stats ON extensions.id = mv_extension_stats.id
            WHERE
                extensions.identifier = $1
                AND NOT extensions.pending
                AND NOT extensions.hidden
            "#,
            Self::columns_sql(None, None)
        ))
        .bind(identifier)
        .fetch_one(database.read())
        .await;

        data.map(|data| Self::map(None, &data)).ok()
    }

    #[inline]
    pub async fn by_id(database: &crate::database::Database, id: i32) -> Option<Self> {
        let data = sqlx::query(&format!(
            r#"
            SELECT {}
            FROM extensions
            JOIN authors ON extensions.author_id = authors.id
            LEFT JOIN mv_extension_stats ON extensions.id = mv_extension_stats.id
            WHERE
                extensions.id = $1
                AND NOT extensions.pending
                AND NOT extensions.hidden
            "#,
            Self::columns_sql(None, None)
        ))
        .bind(id)
        .fetch_one(database.read())
        .await;

        data.map(|data| Self::map(None, &data)).ok()
    }
}
