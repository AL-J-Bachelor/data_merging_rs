use poem::error::InternalServerError;
use poem::web::Data;
use poem::Result;
use poem_openapi::OpenApi;
use poem_openapi::payload::Json;
use sqlx::PgPool;
use models::pim::{NewProduct, Product};

pub struct Api;

#[OpenApi]
impl Api {
    /// Get all Products
    #[oai(path = "/products", method = "get")]
    pub async fn get_products(&self, pool: Data<&PgPool>) -> Result<Json<Vec<Product>>> {
        let products = sqlx::query_as(
            r#"
                SELECT id, sku_number, type, manufacturer, model, dce_serial_number, width, height, depth FROM products
            "#
        )
            .fetch_all(pool.0)
            .await
            .map_err(|e| InternalServerError(e))?;

        Ok(Json(products))
    }

    /// Insert product
    #[oai(path = "/products", method = "put")]
    pub async fn create_product(&self, pool: Data<&PgPool>, product: Json<NewProduct>) -> Result<Json<Vec<Product>>> {
        let products = sqlx::query_as(
            r#"
                INSERT INTO products (sku_number, type, manufacturer, model, dce_serial_number, width, height, depth)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                ON CONFLICT ON CONSTRAINT products_sku_number_manufacturer_model_key DO UPDATE SET model = EXCLUDED.model
                RETURNING id, sku_number, type, manufacturer, model, dce_serial_number, width, height, depth
            "#)
            .bind(&product.sku_number)
            .bind(&product.device_type)
            .bind(&product.manufacturer)
            .bind(&product.model)
            .bind(&product.dce_serial_number)
            .bind(&product.dimensions.width)
            .bind(&product.dimensions.height)
            .bind(&product.dimensions.depth)
            .fetch_all(pool.0)
            .await
            .map_err(|e| InternalServerError(e))?;

        Ok(Json(products))
    }

    /// Delete all Products
    #[oai(path = "/products", method = "delete")]
    pub async fn delete_all_products(&self, pool: Data<&PgPool>) -> Result<Json<Vec<Product>>> {
        let deleted_products = sqlx::query_as(
            r#"
                DELETE FROM products
                RETURNING id, sku_number, type, manufacturer, model, dce_serial_number, width, height, depth
            "#
        )
            .fetch_all(pool.0)
            .await
            .map_err(|e| InternalServerError(e))?;

        Ok(Json(deleted_products))
    }
}