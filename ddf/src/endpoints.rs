use poem::error::{BadRequest, InternalServerError};
use poem_openapi::OpenApi;
use models::ddf::*;
use poem::Result;
use poem::web::Data;
use poem_openapi::payload::{Json, PlainText};
use sqlx::PgPool;
use sqlx::types::Uuid;
use futures::future::join_all;
use futures::StreamExt;

pub struct Api;

#[OpenApi]
impl Api {
    /// Get all DDFs that match a given NewDDF
    #[oai(path = "/matching_ddfs", method = "get")]
    pub async fn get_matching_ddfs(&self, pool: Data<&PgPool>, ddf: Json<NewDDF>) -> Result<Json<Vec<DDF>>> {
        let uuid = Uuid::parse_str(&ddf.dce_serial).map_err(BadRequest)?;
        let ddfs = sqlx::query_as!(
            DDF,
            r#"
                SELECT id, device_type, sku_number, manufacturer, model, dce_serial FROM ddfs
                WHERE
                    device_type = $1 AND
                    manufacturer = $2 AND
                    dce_serial = $3
            "#,
            ddf.device_type,
            ddf.manufacturer,
            uuid
        )
            .fetch_all(pool.0)
            .await
            .map_err(|e| InternalServerError(e))?;

        Ok(Json(ddfs))
    }

    /// Insert a new DDF
    #[oai(path = "/ddfs", method = "post")]
    pub async fn insert_ddf(&self, pool: Data<&PgPool>, ddf: Json<NewDDF>) -> Result<Json<DDF>> {
        let uuid = Uuid::parse_str(&ddf.dce_serial).map_err(BadRequest)?;
        let inserted_ddf = sqlx::query_as!(
            DDF,
            r#"
                INSERT INTO ddfs (device_type, sku_number, manufacturer, model, dce_serial)
                VALUES
                    ($1, $2, $3, $4, $5)
                RETURNING id, device_type, sku_number, manufacturer, model, dce_serial
            "#,
            ddf.device_type,
            ddf.sku_number,
            ddf.manufacturer,
            ddf.model,
            uuid
        )
            .fetch_one(pool.0)
            .await
            .map_err(|e| InternalServerError(e))?;

        Ok(Json(inserted_ddf))
    }

    /// Insert a new DDF
    #[oai(path = "/ddfs/bulk", method = "post")]
    pub async fn insert_ddfs(&self, pool: Data<&PgPool>, ddfs: Json<Vec<NewDDF>>) -> Result<PlainText<DDF>> {
        let created_ddfs: Vec<Option<DDF>> = join_all(ddfs.iter().map(async move |ddf| {
            let uuid = Uuid::parse_str(&ddf.dce_serial).map_err(BadRequest)?;
            let inserted_ddf = sqlx::query_as!(
                DDF,
                r#"
                    INSERT INTO ddfs (device_type, sku_number, manufacturer, model, dce_serial)
                    VALUES
                        ($1, $2, $3, $4, $5)
                    RETURNING id, device_type, sku_number, manufacturer, model, dce_serial
                "#,
                ddf.device_type,
                ddf.sku_number,
                ddf.manufacturer,
                ddf.model,
                uuid
            )
                .fetch_one(pool.0)
                .await;
            inserted_ddf.ok()
        }))
            .await;
        let created_count = created_ddfs.iter()
            .filter(Option::is_some)
            .count();

        Ok(PlainText(created_count))
    }
}
