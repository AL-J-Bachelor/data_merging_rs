use futures::future::join_all;
use poem::{Error, Result};
use poem::error::InternalServerError;
use poem::http::StatusCode;
use poem::web::Data;
use poem_openapi::OpenApi;
use poem_openapi::payload::{Json, PlainText};
use sqlx::PgPool;

use models::ddf::*;

pub struct Api;

#[OpenApi]
impl Api {
    /// Ping
    #[oai(path = "/ping", method = "get")]
    pub async fn ping(&self) -> PlainText<&str> {
        PlainText("OK")
    }

    /// Get all DDFs that match a given NewDDF
    #[oai(path = "/ddfs/matching", method = "get")]
    pub async fn get_matching_ddfs(&self, pool: Data<&PgPool>, ddf: Json<NewDDF>) -> Result<Json<Vec<DDF>>> {
        let ddfs = sqlx::query_as(
            r#"
                SELECT id, type, sku_number, manufacturer, model, dce_serial FROM ddfs
                WHERE
                    type = $1 AND
                    manufacturer = $2 AND
                    dce_serial = $3
            "#)
            .bind(&ddf.device_type)
            .bind(&ddf.manufacturer)
            .bind(&ddf.dce_serial)
            .fetch_all(pool.0)
            .await
            .map_err(|e| InternalServerError(e))?;

        Ok(Json(ddfs))
    }

    /// Get all DDFs
    #[oai(path = "/ddfs", method = "get")]
    pub async fn get_all_ddfs(&self, pool: Data<&PgPool>) -> Result<Json<Vec<DDF>>> {
        let ddfs = sqlx::query_as(
            r#"
                SELECT id, type, sku_number, manufacturer, model, dce_serial FROM ddfs
            "#)
            .fetch_all(pool.0)
            .await
            .map_err(|e| InternalServerError(e))?;

        Ok(Json(ddfs))
    }

    /// Insert a new DDF
    #[oai(path = "/ddfs", method = "put")]
    pub async fn insert_ddf(&self, pool: Data<&PgPool>, ddf: Json<NewDDF>) -> Result<Json<DDF>> {
        let inserted_ddf = self.insert_return_ddf(pool.0, &ddf.0)
            .await
            .map_err(|e| InternalServerError(e))?;
        let inserted_ddf = inserted_ddf.ok_or(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(Json(inserted_ddf))
    }

    /// Insert multiple new DDFs
    #[oai(path = "/ddfs/bulk", method = "put")]
    pub async fn insert_ddfs(&self, pool: Data<&PgPool>, ddfs: Json<Vec<NewDDF>>) -> Json<Vec<DDF>> {
        let created_ddfs = join_all(
            ddfs.iter().map(|ddf| self.insert_return_ddf(pool.0, ddf))
        )
            .await;

        let created_ddfs = created_ddfs.into_iter().flatten().flatten().collect();

        Json(created_ddfs)
    }

    /// Delete all DDFs
    #[oai(path = "/ddfs", method = "delete")]
    pub async fn delete_all_ddfs(&self, pool: Data<&PgPool>) -> Result<Json<Vec<DDF>>> {
        let deleted_ddfs = sqlx::query_as(
            r#"
                DELETE FROM ddfs
                RETURNING id, type, sku_number, manufacturer, model, dce_serial
            "#
        )
            .fetch_all(pool.0)
            .await
            .map_err(|e| InternalServerError(e))?;

        Ok(Json(deleted_ddfs))
    }

    async fn insert_return_ddf(&self, pool: &PgPool, ddf: &NewDDF) -> Result<Option<DDF>, sqlx::Error> {
        let inserted_ddf = sqlx::query_as(
            r#"
                INSERT INTO ddfs (type, sku_number, manufacturer, model, dce_serial)
                VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT ON CONSTRAINT ddfs_sku_number_manufacturer_model_key DO UPDATE SET model = EXCLUDED.model
                RETURNING id, type, sku_number, manufacturer, model, dce_serial
            "#)
            .bind(&ddf.device_type)
            .bind(&ddf.sku_number)
            .bind(&ddf.manufacturer)
            .bind(&ddf.model)
            .bind(&ddf.dce_serial)
            .fetch_optional(pool)
            .await?;
        Ok(inserted_ddf)
    }
}
