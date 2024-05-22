use poem::error::InternalServerError;
use poem::web::Data;
use poem_openapi::OpenApi;
use poem_openapi::payload::Json;
use sqlx::PgPool;
use sqlx::types::Uuid;
use crate::models::Product;

pub struct Api;

#[OpenApi]
impl Api {
    /// Get all Products
    #[oai(path = "/products", method = "get")]
    pub async fn get_products(&self, pool: Data<&PgPool>) -> poem::Result<Json<Vec<Product>>> {
        let products = sqlx::query_as!(
            Product,
            r#"
                SELECT id FROM products
            "#
        )
            .fetch_all(pool.0)
            .await
            .map_err(|e| InternalServerError(e))?;

        Ok(Json(products))
    }
}