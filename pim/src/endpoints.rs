use poem::error::InternalServerError;
use poem::web::Data;
use poem::Result;
use poem_openapi::OpenApi;
use poem_openapi::payload::Json;
use sqlx::PgPool;
use models::pim::Product;

pub struct Api;

#[OpenApi]
impl Api {
    /// Get all Products
    #[oai(path = "/products", method = "get")]
    pub async fn get_products(&self, pool: Data<&PgPool>) -> Result<Json<Vec<Product>>> {
        let products = sqlx::query_as!(
            Product,
            r#"
                SELECT id, sku_number, device_type, manufacturer, model, dce_serial_number, width, height, depth FROM products
            "#
        )
            .fetch_all(pool.0)
            .await
            .map_err(|e| InternalServerError(e))?;

        Ok(Json(products))
    }

    /// Delete all Products
    #[oai(path = "/products", method = "delete")]
    pub async fn delete_all_products(&self, pool: Data<&PgPool>) -> Result<Json<Vec<Product>>> {
        let deleted_products = sqlx::query_as!(
            Product,
            r#"
                DELETE FROM products
                RETURNING id, sku_number, device_type, manufacturer, model, dce_serial_number, width, height, depth
            "#
        )
            .fetch_all(pool.0)
            .await
            .map_err(|e| InternalServerError(e))?;

        Ok(Json(deleted_products))
    }
}