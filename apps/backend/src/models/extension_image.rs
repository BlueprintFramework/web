use super::BaseModel;
use image::{DynamicImage, codecs::webp::WebPEncoder, imageops::FilterType};
use rand::distr::SampleString;
use serde::{Deserialize, Serialize};
use sqlx::{Row, postgres::PgRow, types::chrono::NaiveDateTime};
use std::collections::BTreeMap;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone)]
pub struct ExtensionImage {
    pub id: i32,

    pub width: i32,
    pub height: i32,
    pub size: i32,
    pub location: String,

    pub created: NaiveDateTime,
}

impl BaseModel for ExtensionImage {
    #[inline]
    fn columns(prefix: Option<&str>, table: Option<&str>) -> BTreeMap<String, String> {
        let prefix = prefix.unwrap_or_default();
        let table = table.unwrap_or("extension_images");

        BTreeMap::from([
            (format!("{table}.id"), format!("{}id", prefix)),
            (format!("{table}.width"), format!("{}width", prefix)),
            (format!("{table}.height"), format!("{}height", prefix)),
            (format!("{table}.size"), format!("{}size", prefix)),
            (format!("{table}.location"), format!("{}location", prefix)),
            (format!("{table}.created"), format!("{}created", prefix)),
        ])
    }

    #[inline]
    fn map(prefix: Option<&str>, row: &PgRow) -> Self {
        let prefix = prefix.unwrap_or_default();

        Self {
            id: row.get(format!("{prefix}id").as_str()),
            width: row.get(format!("{prefix}width").as_str()),
            height: row.get(format!("{prefix}height").as_str()),
            size: row.get(format!("{prefix}size").as_str()),
            location: row.get(format!("{prefix}location").as_str()),
            created: row.get(format!("{prefix}created").as_str()),
        }
    }
}

impl ExtensionImage {
    pub async fn create(
        database: &crate::database::Database,
        s3: &crate::s3::S3,
        extension: &super::extension::Extension,
        image: DynamicImage,
    ) -> Result<Self, anyhow::Error> {
        let identifier_random = rand::distr::Alphanumeric.sample_string(&mut rand::rng(), 8);
        let location = format!(
            "extensions/images/{}/{}.webp",
            extension.identifier, identifier_random
        );

        let image = tokio::task::spawn_blocking(move || {
            image.resize(
                image.width().min(1280),
                image.height().min(10000),
                FilterType::Triangle,
            )
        })
        .await?;

        let width = image.width() as i32;
        let height = image.height() as i32;

        let data = tokio::task::spawn_blocking(move || {
            let mut data: Vec<u8> = Vec::new();
            let encoder = WebPEncoder::new_lossless(&mut data);
            encoder.encode(
                image.as_bytes(),
                image.width(),
                image.height(),
                image.color().into(),
            )?;

            Ok::<_, anyhow::Error>(data)
        })
        .await??;

        let mut transaction = database.write().begin().await?;

        let row = sqlx::query(&format!(
            r#"
            INSERT INTO extension_images (extension_id, width, height, size, location, created)
            VALUES ($1, $2, $3, $4, $5, NOW())
            RETURNING {}
            "#,
            Self::columns_sql(None, None)
        ))
        .bind(extension.id)
        .bind(width)
        .bind(height)
        .bind(data.len() as i32)
        .bind(&location)
        .fetch_one(&mut *transaction)
        .await?;

        let count_and_total = sqlx::query!(
            r#"
            SELECT
                COUNT(*) AS count,
                COALESCE(SUM(size), 0) AS total
            FROM extension_images
            WHERE extension_images.extension_id = $1
            "#,
            extension.id
        )
        .fetch_one(&mut *transaction)
        .await?;

        let image_count: i64 = count_and_total.count.unwrap_or(0);
        let image_total: i64 = count_and_total.total.unwrap_or(0);

        if image_count > 25 {
            transaction.rollback().await?;

            return Err(anyhow::anyhow!(
                "unable to upload image: extension image limit reached"
            ));
        }

        if image_total + (data.len() as i64) > 30 * 1024 * 1024 {
            transaction.rollback().await?;

            return Err(anyhow::anyhow!(
                "unable to upload image: extension image storage limit reached"
            ));
        }

        s3.upload(location, &data, Some("image/webp")).await?;
        transaction.commit().await?;

        Ok(Self::map(None, &row))
    }

    pub async fn all_by_extension_id(
        database: &crate::database::Database,
        extension_id: i32,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let rows = sqlx::query(&format!(
            r#"
            SELECT {}
            FROM extension_images
            WHERE extension_id = $1
            ORDER BY extension_images.id
            "#,
            Self::columns_sql(None, None)
        ))
        .bind(extension_id)
        .fetch_all(database.read())
        .await?;

        Ok(rows.into_iter().map(|row| Self::map(None, &row)).collect())
    }

    pub async fn by_extension_id_id(
        database: &crate::database::Database,
        extension_id: i32,
        id: i32,
    ) -> Result<Option<Self>, sqlx::Error> {
        let row = sqlx::query(&format!(
            r#"
            SELECT {}
            FROM extension_images
            WHERE extension_id = $1 AND extension_images.id = $2
            "#,
            Self::columns_sql(None, None)
        ))
        .bind(extension_id)
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
        s3.bucket.delete_object(&self.location).await?;

        sqlx::query!(
            "DELETE FROM extension_images
            WHERE extension_images.id = $1",
            self.id
        )
        .execute(database.write())
        .await?;

        Ok(())
    }

    #[inline]
    pub fn into_api_object(self, env: &crate::env::Env) -> ApiExtensionImage {
        ApiExtensionImage {
            id: self.id,
            width: self.width,
            height: self.height,
            size: self.size,
            url: format!("{}/{}", env.s3_url, self.location),
            created: self.created.and_utc(),
        }
    }
}

#[derive(ToSchema, Serialize)]
#[schema(title = "ExtensionImage")]
pub struct ApiExtensionImage {
    pub id: i32,

    pub width: i32,
    pub height: i32,
    pub size: i32,
    pub url: String,

    pub created: chrono::DateTime<chrono::Utc>,
}
