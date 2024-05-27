use poem::error::InternalServerError;
use poem_openapi::OpenApi;
use models::ddf::*;
use poem::Result;
use poem::web::Data;
use poem_openapi::payload::{Json, PlainText};
use sqlx::PgPool;
use futures::future::join_all;

pub struct Api;

#[OpenApi]
impl Api {
    /// Get all DDFs that match a given NewDDF
    #[oai(path = "/ddfs/matching", method = "get")]
    pub async fn get_matching_ddfs(&self, pool: Data<&PgPool>, ddf: Json<NewDDF>) -> Result<Json<Vec<DDF>>> {
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
            ddf.dce_serial,
        )
            .fetch_all(pool.0)
            .await
            .map_err(|e| InternalServerError(e))?;

        Ok(Json(ddfs))
    }

    /// Get all DDFs
    #[oai(path = "/ddfs", method = "get")]
    pub async fn get_all_ddfs(&self, pool: Data<&PgPool>) -> Result<Json<Vec<DDF>>> {
        let ddfs = sqlx::query_as!(
            DDF,
            r#"
                SELECT id, device_type, sku_number, manufacturer, model, dce_serial FROM ddfs
            "#
        )
            .fetch_all(pool.0)
            .await
            .map_err(|e| InternalServerError(e))?;

        Ok(Json(ddfs))
    }

    /// Insert a new DDF
    #[oai(path = "/ddfs", method = "post")]
    pub async fn insert_ddf(&self, pool: Data<&PgPool>, ddf: Json<NewDDF>) -> Result<Json<DDF>> {
        let inserted_ddf = self.insert_return_ddf(pool.0, &ddf.0)
            .await
            .unwrap();

        Ok(Json(inserted_ddf))
    }

    /// Insert a new DDF
    #[oai(path = "/ddfs/bulk", method = "post")]
    pub async fn insert_ddfs(&self, pool: Data<&PgPool>, ddfs: Json<Vec<NewDDF>>) -> Result<PlainText<String>> {
        let created_ddfs = join_all(
            ddfs.iter().map(|ddf| self.insert_return_ddf(pool.0, ddf))
        )
            .await;

        let created_count = created_ddfs.iter()
            .filter(|ddf| ddf.is_ok())
            .count();

        Ok(PlainText(created_count.to_string()))
    }

    async fn insert_return_ddf(&self, pool: &PgPool, ddf: &NewDDF) -> color_eyre::Result<DDF> {
        let inserted_ddf = sqlx::query_as!(
            DDF,
            r#"
                INSERT INTO ddfs (device_type, sku_number, manufacturer, model, dce_serial)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id, device_type, sku_number, manufacturer, model, dce_serial
            "#,
            ddf.device_type,
            ddf.sku_number,
            ddf.manufacturer,
            ddf.model,
            ddf.dce_serial,
        )
            .fetch_one(pool)
            .await?;
        Ok(inserted_ddf)
    }
}
