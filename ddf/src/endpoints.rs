use poem::error::{BadRequest, InternalServerError};
use poem_openapi::OpenApi;
use crate::models::*;
use poem::Result;
use poem::web::Data;
use poem_openapi::payload::Json;
use sqlx::PgPool;
use sqlx::types::Uuid;

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
        let uuid = Uuid::parse_str(&ddf.dce_serial).map_err(BadRequest);
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
}
