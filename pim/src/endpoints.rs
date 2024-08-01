use poem::error::InternalServerError;
use poem::Result;
use poem::web::{Data, Query};
use poem_openapi::OpenApi;
use poem_openapi::payload::{Json, PlainText};
use serde::Deserialize;
use sqlx::PgPool;

use models::pim::{Millimeters, NewProduct, Product};

#[derive(Deserialize)]
pub struct ProductCount {
    count: u32,
}

pub struct Api;

#[OpenApi]
impl Api {
    /// Ping
    #[allow(clippy::unused_async)]
    #[oai(path = "/ping", method = "get")]
    pub async fn ping(&self) -> PlainText<&str> {
        PlainText("OK")
    }

    /// Get all Products
    #[oai(path = "/products", method = "get")]
    pub async fn get_products(&self, pool: Data<&PgPool>) -> Result<Json<Vec<Product>>> {
        let products = sqlx::query_as(
            r"
                SELECT id, sku_number, type, manufacturer, model, dce_serial_number, width, height, depth FROM products
            "
        )
            .fetch_all(pool.0)
            .await
            .map_err(InternalServerError)?;

        Ok(Json(products))
    }

    /// Insert product
    #[oai(path = "/products", method = "post")]
    pub async fn create_product(&self, pool: Data<&PgPool>, product: Json<NewProduct>) -> Result<Json<Vec<Product>>> {
        let products = sqlx::query_as(
            r"
                INSERT INTO products (sku_number, type, manufacturer, model, dce_serial_number, width, height, depth)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                ON CONFLICT ON CONSTRAINT products_sku_number_manufacturer_model_key DO UPDATE SET model = EXCLUDED.model
                RETURNING id, sku_number, type, manufacturer, model, dce_serial_number, width, height, depth
            ")
            .bind(&product.sku_number)
            .bind(&product.device_type)
            .bind(&product.manufacturer)
            .bind(&product.model)
            .bind(&product.dce_serial_number)
            .bind(product.dimensions.width)
            .bind(product.dimensions.height)
            .bind(product.dimensions.depth)
            .fetch_all(pool.0)
            .await
            .map_err(InternalServerError)?;

        Ok(Json(products))
    }

    /// Insert product
    #[oai(path = "/products/random", method = "post")]
    pub async fn create_random_products(&self, pool: Data<&PgPool>, count: Query<ProductCount>) -> Result<PlainText<String>> {
        let products = (0..count.0.count).map(|_| Product::generate_random()).collect::<Vec<Product>>();
        let mut sku_numbers: Vec<String> = Vec::new();
        let mut device_types: Vec<String> = Vec::new();
        let mut manufacturers: Vec<String> = Vec::new();
        let mut models: Vec<Option<String>> = Vec::new();
        let mut serial_numbers: Vec<String> = Vec::new();
        let mut widths: Vec<Millimeters> = Vec::new();
        let mut heights: Vec<Millimeters> = Vec::new();
        let mut depths: Vec<Millimeters> = Vec::new();
        products.clone().into_iter().for_each(|product| {
            sku_numbers.push(product.sku_number);
            device_types.push(product.device_type);
            manufacturers.push(product.manufacturer);
            models.push(product.model);
            serial_numbers.push(product.dce_serial_number);
            widths.push(product.dimensions.width);
            heights.push(product.dimensions.height);
            depths.push(product.dimensions.depth);
        });
        sqlx::query(
            r"
                INSERT INTO products (sku_number, type, manufacturer, model, dce_serial_number, width, height, depth)
                SELECT * FROM UNNEST($1, $2, $3, $4, $5, $6, $7, $8)
                ON CONFLICT ON CONSTRAINT products_sku_number_manufacturer_model_key DO UPDATE SET model = EXCLUDED.model
            ")
            .bind(&sku_numbers)
            .bind(&device_types)
            .bind(&manufacturers)
            .bind(&models)
            .bind(&serial_numbers)
            .bind(&widths)
            .bind(&heights)
            .bind(&depths)
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(PlainText(products.len().to_string()))
    }

    /// Delete all Products
    #[oai(path = "/products", method = "delete")]
    pub async fn delete_all_products(&self, pool: Data<&PgPool>) -> Result<Json<Vec<Product>>> {
        let deleted_products = sqlx::query_as(
            r"
                DELETE FROM products
                RETURNING id, sku_number, type, manufacturer, model, dce_serial_number, width, height, depth
            "
        )
            .fetch_all(pool.0)
            .await
            .map_err(InternalServerError)?;

        Ok(Json(deleted_products))
    }
}