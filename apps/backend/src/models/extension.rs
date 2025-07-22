use super::{BaseModel, user::User};
use serde::{Deserialize, Serialize};
use sqlx::{Row, postgres::PgRow, prelude::Type, types::chrono::NaiveDateTime};
use std::collections::BTreeMap;
use utoipa::ToSchema;

#[derive(ToSchema, Serialize, Deserialize, Type)]
#[serde(rename_all = "lowercase")]
#[schema(rename_all = "lowercase")]
#[sqlx(type_name = "extension_type", rename_all = "UPPERCASE")]
pub enum ExtensionType {
    Theme,
    Extension,
}

#[derive(ToSchema, Serialize, Deserialize, Type, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[schema(rename_all = "lowercase")]
#[sqlx(type_name = "extension_status", rename_all = "UPPERCASE")]
pub enum ExtensionStatus {
    Approved,
    Ready,
    Pending,
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

#[derive(Serialize, Deserialize)]
pub struct Extension {
    pub id: i32,
    pub author: User,

    pub r#type: ExtensionType,
    pub status: ExtensionStatus,
    pub deny_reason: Option<String>,
    pub unlisted: bool,

    pub name: String,
    pub identifier: String,
    pub summary: String,
    pub description: Option<String>,

    pub platforms: BTreeMap<String, ExtensionPlatform>,
    pub versions: Vec<ExtensionVersion>,

    pub keywords: Vec<String>,
    pub banner: String,

    pub created: NaiveDateTime,

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
                format!("{}.status", table),
                format!("{}status", prefix.unwrap_or_default()),
            ),
            (
                format!("{}.deny_reason", table),
                format!("{}deny_reason", prefix.unwrap_or_default()),
            ),
            (
                format!("{}.unlisted", table),
                format!("{}unlisted", prefix.unwrap_or_default()),
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
                format!("{}.description", table),
                format!("{}description", prefix.unwrap_or_default()),
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

        columns.extend(User::columns(Some("author_"), None));

        columns
    }

    #[inline]
    fn map(prefix: Option<&str>, row: &PgRow) -> Self {
        let prefix = prefix.unwrap_or_default();

        Self {
            id: row.get(format!("{}id", prefix).as_str()),
            author: User::map(Some("author_"), row),
            r#type: row.get(format!("{}type", prefix).as_str()),
            status: row.get(format!("{}status", prefix).as_str()),
            deny_reason: row.get(format!("{}deny_reason", prefix).as_str()),
            unlisted: row.get(format!("{}unlisted", prefix).as_str()),
            name: row.get(format!("{}name", prefix).as_str()),
            identifier: row.get(format!("{}identifier", prefix).as_str()),
            summary: row.get(format!("{}summary", prefix).as_str()),
            description: row.get(format!("{}description", prefix).as_str()),
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

    pub async fn all(database: &crate::database::Database) -> Vec<Self> {
        sqlx::query(&format!(
            r#"
            SELECT {}
            FROM extensions
            JOIN users ON extensions.author_id = users.id
            LEFT JOIN mv_extension_stats ON extensions.id = mv_extension_stats.id
            WHERE
                NOT unlisted
                AND status = 'approved'
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

    pub async fn by_identifier(
        database: &crate::database::Database,
        identifier: &str,
    ) -> Option<Self> {
        let data = sqlx::query(&format!(
            r#"
            SELECT {}
            FROM extensions
            JOIN users ON extensions.author_id = users.id
            LEFT JOIN mv_extension_stats ON extensions.id = mv_extension_stats.id
            WHERE extensions.identifier = $1
            "#,
            Self::columns_sql(None, None)
        ))
        .bind(identifier)
        .fetch_optional(database.read())
        .await
        .unwrap();

        data.map(|data| Self::map(None, &data))
    }

    pub async fn by_id(database: &crate::database::Database, id: i32) -> Option<Self> {
        let data = sqlx::query(&format!(
            r#"
            SELECT {}
            FROM extensions
            JOIN users ON extensions.author_id = users.id
            LEFT JOIN mv_extension_stats ON extensions.id = mv_extension_stats.id
            WHERE extensions.id = $1
            "#,
            Self::columns_sql(None, None)
        ))
        .bind(id)
        .fetch_optional(database.read())
        .await
        .unwrap();

        data.map(|data| Self::map(None, &data))
    }

    #[inline]
    pub fn into_api_object(self) -> ApiExtension {
        ApiExtension {
            id: self.id,
            author: self.author.into_api_object(),
            r#type: self.r#type,
            status: self.status,
            unlisted: self.unlisted,
            name: self.name,
            identifier: self.identifier,
            summary: self.summary,
            description: self.description,
            platforms: self.platforms,
            versions: self.versions,
            keywords: self.keywords,
            banner: self.banner,
            stats: self.stats,
            created: self.created.and_utc(),
        }
    }
}

#[derive(ToSchema, Serialize)]
#[schema(title = "Extension")]
pub struct ApiExtension {
    pub id: i32,
    pub author: super::user::ApiUser,

    pub r#type: ExtensionType,
    pub status: ExtensionStatus,
    pub unlisted: bool,

    pub name: String,
    pub identifier: String,
    pub summary: String,
    pub description: Option<String>,

    #[schema(inline)]
    pub platforms: BTreeMap<String, ExtensionPlatform>,
    #[schema(inline)]
    pub versions: Vec<ExtensionVersion>,

    pub keywords: Vec<String>,
    pub banner: String,

    #[schema(inline)]
    pub stats: ExtensionStats,

    pub created: chrono::DateTime<chrono::Utc>,
}
