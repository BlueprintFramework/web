use super::{BaseModel, user::User};
use rand::distr::SampleString;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::{Row, postgres::PgRow, prelude::Type, types::chrono::NaiveDateTime};
use std::{collections::BTreeMap, sync::LazyLock};
use utoipa::ToSchema;

pub static IDENTIFIER_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-z]+$").expect("Failed to compile identifier regex"));

#[derive(ToSchema, Serialize, Deserialize, Type, Clone, Copy)]
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

#[derive(ToSchema, Serialize, Deserialize, Clone)]
pub struct ExtensionVersion {
    pub name: String,
    pub downloads: u32,

    pub created: NaiveDateTime,
}

#[derive(ToSchema, Serialize, Deserialize, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
#[schema(rename_all = "UPPERCASE")]
pub enum ExtensionPlatform {
    Builtbybit,
    Sourcexchange,
    Github,
}

#[derive(ToSchema, Serialize, Deserialize, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
#[schema(rename_all = "UPPERCASE")]
pub enum ExtensionPlatformCurrency {
    Usd,
    Eur,
    Gbp,
}

#[derive(ToSchema, Serialize, Deserialize, Clone)]
pub struct ExtensionPlatformData {
    pub url: String,
    pub price: f64,
    pub currency: ExtensionPlatformCurrency,

    pub reviews: Option<u32>,
    pub rating: Option<f64>,
}

#[derive(ToSchema, Serialize, Deserialize)]
pub struct MinimalExtensionPlatformData {
    pub url: String,
    pub price: f64,
    pub currency: ExtensionPlatformCurrency,
}

#[derive(Default, ToSchema, Serialize, Deserialize, Clone)]
pub struct ExtensionStats {
    pub panels: i64,
}

#[derive(Serialize, Deserialize, Clone)]
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

    pub platforms: BTreeMap<ExtensionPlatform, ExtensionPlatformData>,
    pub versions: Vec<ExtensionVersion>,

    pub keywords: Vec<String>,
    pub banner: String,

    pub created: NaiveDateTime,

    pub stats: ExtensionStats,
}

impl BaseModel for Extension {
    #[inline]
    fn columns(prefix: Option<&str>, table: Option<&str>) -> BTreeMap<String, String> {
        let prefix = prefix.unwrap_or_default();
        let table = table.unwrap_or("extensions");

        let mut columns = BTreeMap::from([
            (format!("{table}.id"), format!("{}id", prefix)),
            (format!("{table}.type"), format!("{}type", prefix)),
            (format!("{table}.status"), format!("{}status", prefix)),
            (
                format!("{table}.deny_reason"),
                format!("{}deny_reason", prefix),
            ),
            (format!("{table}.unlisted"), format!("{}unlisted", prefix)),
            (format!("{table}.name"), format!("{}name", prefix)),
            (
                format!("{table}.identifier"),
                format!("{}identifier", prefix),
            ),
            (format!("{table}.summary"), format!("{}summary", prefix)),
            (
                format!("{table}.description"),
                format!("{}description", prefix),
            ),
            (format!("{table}.platforms"), format!("{}platforms", prefix)),
            (format!("{table}.versions"), format!("{}versions", prefix)),
            (format!("{table}.keywords"), format!("{}keywords", prefix)),
            (format!("{table}.banner"), format!("{}banner", prefix)),
            (
                "mv_extension_stats.stats".to_string(),
                format!("{}stats", prefix),
            ),
            (format!("{table}.created"), format!("{}created", prefix)),
        ]);

        columns.extend(User::columns(Some("author_"), None));

        columns
    }

    #[inline]
    fn map(prefix: Option<&str>, row: &PgRow) -> Self {
        let prefix = prefix.unwrap_or_default();

        Self {
            id: row.get(format!("{prefix}id").as_str()),
            author: User::map(Some("author_"), row),
            r#type: row.get(format!("{prefix}type").as_str()),
            status: row.get(format!("{prefix}status").as_str()),
            deny_reason: row.get(format!("{prefix}deny_reason").as_str()),
            unlisted: row.get(format!("{prefix}unlisted").as_str()),
            name: row.get(format!("{prefix}name").as_str()),
            identifier: row.get(format!("{prefix}identifier").as_str()),
            summary: row.get(format!("{prefix}summary").as_str()),
            description: row.get(format!("{prefix}description").as_str()),
            platforms: serde_json::from_value(row.get(format!("{prefix}platforms").as_str()))
                .unwrap_or_default(),
            versions: serde_json::from_value(row.get(format!("{prefix}versions").as_str()))
                .unwrap_or_default(),
            keywords: row.get(format!("{prefix}keywords").as_str()),
            banner: row.get(format!("{prefix}banner").as_str()),
            stats: if let Ok(data) = row.try_get(format!("{prefix}stats").as_str()) {
                serde_json::from_value(data).unwrap_or_default()
            } else {
                Default::default()
            },
            created: row.get(format!("{prefix}created").as_str()),
        }
    }
}

impl Extension {
    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        database: &crate::database::Database,
        author_id: i32,
        name: &str,
        identifier: &str,
        r#type: ExtensionType,
        unlisted: bool,
        summary: &str,
        description: Option<&str>,
        platforms: &BTreeMap<ExtensionPlatform, ExtensionPlatformData>,
    ) -> Result<i32, sqlx::Error> {
        let identifier_random = rand::distr::Alphanumeric.sample_string(&mut rand::rng(), 8);

        let row = sqlx::query(
            r#"
            INSERT INTO extensions (author_id, name, identifier, type, status, unlisted, summary, description, platforms, banner, created)
            VALUES ($1, $2, $3, $4, 'PENDING', $5, $6, $7, $8, $9, NOW())
            RETURNING extensions.id
            "#,
        )
        .bind(author_id)
        .bind(name)
        .bind(identifier)
        .bind(r#type)
        .bind(unlisted)
        .bind(summary)
        .bind(description)
        .bind(serde_json::to_value(platforms).unwrap())
        .bind("_default.jpeg")
        .fetch_one(database.write())
        .await?;

        sqlx::query!(
            "UPDATE extensions
            SET identifier = $2
            WHERE extensions.id = $1",
            row.get::<i32, _>("id"),
            format!("{identifier}:{identifier_random}")
        )
        .execute(database.write())
        .await?;

        Ok(row.get("id"))
    }

    pub async fn all(database: &crate::database::Database) -> Result<Vec<Self>, sqlx::Error> {
        let rows = sqlx::query(&format!(
            r#"
            SELECT {}
            FROM extensions
            JOIN users ON extensions.author_id = users.id
            LEFT JOIN mv_extension_stats ON extensions.id = mv_extension_stats.id
            WHERE NOT unlisted AND status = 'APPROVED'
            ORDER BY id ASC
            "#,
            Self::columns_sql(None, None)
        ))
        .fetch_all(database.read())
        .await?;

        Ok(rows.into_iter().map(|row| Self::map(None, &row)).collect())
    }

    pub async fn all_admin_with_pagination(
        database: &crate::database::Database,
        page: i64,
        per_page: i64,
    ) -> Result<super::Pagination<Self>, sqlx::Error> {
        let offset = (page - 1) * per_page;

        let rows = sqlx::query(&format!(
            r#"
            SELECT {}, COUNT(*) OVER() AS total_count
            FROM extensions
            JOIN users ON extensions.author_id = users.id
            LEFT JOIN mv_extension_stats ON extensions.id = mv_extension_stats.id
            ORDER BY extensions.id DESC
            LIMIT $1 OFFSET $2
            "#,
            Self::columns_sql(None, None)
        ))
        .bind(per_page)
        .bind(offset)
        .fetch_all(database.read())
        .await?;

        Ok(super::Pagination {
            total: rows.first().map_or(0, |row| row.get("total_count")),
            per_page,
            page,
            data: rows.into_iter().map(|row| Self::map(None, &row)).collect(),
        })
    }

    pub async fn all_admin_pending_with_pagination(
        database: &crate::database::Database,
        page: i64,
        per_page: i64,
    ) -> Result<super::Pagination<Self>, sqlx::Error> {
        let offset = (page - 1) * per_page;

        let rows = sqlx::query(&format!(
            r#"
            SELECT {}, COUNT(*) OVER() AS total_count
            FROM extensions
            JOIN users ON extensions.author_id = users.id
            LEFT JOIN mv_extension_stats ON extensions.id = mv_extension_stats.id
            WHERE extensions.status = 'PENDING'
            ORDER BY extensions.id DESC
            LIMIT $1 OFFSET $2
            "#,
            Self::columns_sql(None, None)
        ))
        .bind(per_page)
        .bind(offset)
        .fetch_all(database.read())
        .await?;

        Ok(super::Pagination {
            total: rows.first().map_or(0, |row| row.get("total_count")),
            per_page,
            page,
            data: rows.into_iter().map(|row| Self::map(None, &row)).collect(),
        })
    }

    pub async fn all_admin_ready_with_pagination(
        database: &crate::database::Database,
        page: i64,
        per_page: i64,
    ) -> Result<super::Pagination<Self>, sqlx::Error> {
        let offset = (page - 1) * per_page;

        let rows = sqlx::query(&format!(
            r#"
            SELECT {}, COUNT(*) OVER() AS total_count
            FROM extensions
            JOIN users ON extensions.author_id = users.id
            LEFT JOIN mv_extension_stats ON extensions.id = mv_extension_stats.id
            WHERE extensions.status = 'READY'
            ORDER BY extensions.id DESC
            LIMIT $1 OFFSET $2
            "#,
            Self::columns_sql(None, None)
        ))
        .bind(per_page)
        .bind(offset)
        .fetch_all(database.read())
        .await?;

        Ok(super::Pagination {
            total: rows.first().map_or(0, |row| row.get("total_count")),
            per_page,
            page,
            data: rows.into_iter().map(|row| Self::map(None, &row)).collect(),
        })
    }

    pub async fn all_admin_denied_with_pagination(
        database: &crate::database::Database,
        page: i64,
        per_page: i64,
    ) -> Result<super::Pagination<Self>, sqlx::Error> {
        let offset = (page - 1) * per_page;

        let rows = sqlx::query(&format!(
            r#"
            SELECT {}, COUNT(*) OVER() AS total_count
            FROM extensions
            JOIN users ON extensions.author_id = users.id
            LEFT JOIN mv_extension_stats ON extensions.id = mv_extension_stats.id
            WHERE extensions.deny_reason IS NOT NULL
            ORDER BY extensions.id DESC
            LIMIT $1 OFFSET $2
            "#,
            Self::columns_sql(None, None)
        ))
        .bind(per_page)
        .bind(offset)
        .fetch_all(database.read())
        .await?;

        Ok(super::Pagination {
            total: rows.first().map_or(0, |row| row.get("total_count")),
            per_page,
            page,
            data: rows.into_iter().map(|row| Self::map(None, &row)).collect(),
        })
    }

    pub async fn by_author_id_with_pagination(
        database: &crate::database::Database,
        author_id: i32,
        page: i64,
        per_page: i64,
    ) -> Result<super::Pagination<Self>, sqlx::Error> {
        let offset = (page - 1) * per_page;

        let rows = sqlx::query(&format!(
            r#"
            SELECT {}, COUNT(*) OVER() AS total_count
            FROM extensions
            JOIN users ON extensions.author_id = users.id
            LEFT JOIN mv_extension_stats ON extensions.id = mv_extension_stats.id
            WHERE extensions.author_id = $1
            ORDER BY extensions.id DESC
            LIMIT $2 OFFSET $3
            "#,
            Self::columns_sql(None, None)
        ))
        .bind(author_id)
        .bind(per_page)
        .bind(offset)
        .fetch_all(database.read())
        .await?;

        Ok(super::Pagination {
            total: rows.first().map_or(0, |row| row.get("total_count")),
            per_page,
            page,
            data: rows.into_iter().map(|row| Self::map(None, &row)).collect(),
        })
    }

    pub async fn by_identifier(
        database: &crate::database::Database,
        identifier: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        let row = sqlx::query(&format!(
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
        .await?;

        Ok(row.map(|data| Self::map(None, &data)))
    }

    pub async fn by_id(
        database: &crate::database::Database,
        id: i32,
    ) -> Result<Option<Self>, sqlx::Error> {
        let row = sqlx::query(&format!(
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
        .await?;

        Ok(row.map(|data| Self::map(None, &data)))
    }

    pub async fn delete(
        &self,
        database: &crate::database::Database,
        s3: &crate::s3::S3,
    ) -> Result<(), anyhow::Error> {
        if self.banner != "_default.jpeg" {
            tokio::try_join!(
                s3.bucket
                    .delete_object(format!("extensions/lowres/{}", self.banner)),
                s3.bucket
                    .delete_object(format!("extensions/{}", self.banner))
            )?;
        }

        let images =
            super::extension_image::ExtensionImage::all_by_extension_id(database, self.id).await?;
        let mut futures = Vec::new();
        futures.reserve_exact(images.len());

        for image in images {
            futures.push(s3.bucket.delete_object(image.location));
        }

        futures_util::future::try_join_all(futures).await?;

        sqlx::query!(
            "DELETE FROM extensions
            WHERE extensions.id = $1",
            self.id
        )
        .execute(database.write())
        .await?;

        Ok(())
    }

    #[inline]
    pub fn versions(&self) -> Vec<&String> {
        let mut versions: Vec<&String> = Vec::new();
        versions.reserve_exact(self.versions.len());

        for version in self.versions.iter() {
            versions.push(&version.name);
        }

        versions
    }

    #[inline]
    pub fn into_api_object(mut self, env: &crate::env::Env) -> ApiExtension {
        ApiExtension {
            id: self.id,
            author: self.author.into_api_object(),
            r#type: self.r#type,
            name: self.name,
            identifier: if self.identifier.contains(":") {
                self.identifier.truncate(self.identifier.len() - 9);

                self.identifier
            } else {
                self.identifier
            },
            summary: self.summary,
            description: self.description,
            platforms: self.platforms,
            versions: self.versions,
            keywords: self.keywords,
            banner: ExtensionBanner {
                lowres: format!("{}/extensions/lowres/{}", env.s3_url, self.banner),
                fullres: format!("{}/extensions/{}", env.s3_url, self.banner),
            },
            stats: self.stats,
            created: self.created.and_utc(),
        }
    }

    #[inline]
    pub fn into_api_full_object(mut self, env: &crate::env::Env) -> ApiFullExtension {
        ApiFullExtension {
            id: self.id,
            author: self.author.into_api_object(),
            r#type: self.r#type,
            status: self.status,
            deny_reason: self.deny_reason,
            unlisted: self.unlisted,
            name: self.name,
            identifier: if self.identifier.contains(":") {
                self.identifier.truncate(self.identifier.len() - 9);

                self.identifier
            } else {
                self.identifier
            },
            summary: self.summary,
            description: self.description,
            platforms: self.platforms,
            versions: self.versions,
            keywords: self.keywords,
            banner: ExtensionBanner {
                lowres: format!("{}/extensions/lowres/{}", env.s3_url, self.banner),
                fullres: format!("{}/extensions/{}", env.s3_url, self.banner),
            },
            stats: self.stats,
            created: self.created.and_utc(),
        }
    }
}

#[derive(ToSchema, Serialize)]
pub struct ExtensionBanner {
    pub lowres: String,
    pub fullres: String,
}

#[derive(ToSchema, Serialize)]
#[schema(title = "FullExtension")]
pub struct ApiFullExtension {
    pub id: i32,
    pub author: super::user::ApiUser,

    pub r#type: ExtensionType,
    pub status: ExtensionStatus,
    pub deny_reason: Option<String>,
    pub unlisted: bool,

    pub name: String,
    pub identifier: String,
    pub summary: String,
    pub description: Option<String>,

    #[schema(inline)]
    pub platforms: BTreeMap<ExtensionPlatform, ExtensionPlatformData>,
    #[schema(inline)]
    pub versions: Vec<ExtensionVersion>,

    pub keywords: Vec<String>,
    #[schema(inline)]
    pub banner: ExtensionBanner,

    #[schema(inline)]
    pub stats: ExtensionStats,

    pub created: chrono::DateTime<chrono::Utc>,
}

#[derive(ToSchema, Serialize)]
#[schema(title = "Extension")]
pub struct ApiExtension {
    pub id: i32,
    pub author: super::user::ApiUser,

    pub r#type: ExtensionType,

    pub name: String,
    pub identifier: String,
    pub summary: String,
    pub description: Option<String>,

    #[schema(inline)]
    pub platforms: BTreeMap<ExtensionPlatform, ExtensionPlatformData>,
    #[schema(inline)]
    pub versions: Vec<ExtensionVersion>,

    pub keywords: Vec<String>,
    #[schema(inline)]
    pub banner: ExtensionBanner,

    #[schema(inline)]
    pub stats: ExtensionStats,

    pub created: chrono::DateTime<chrono::Utc>,
}
