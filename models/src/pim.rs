use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Object)]
#[derive(Serialize, Deserialize)]
#[derive(FromRow)]
#[derive(Clone)]
pub struct Product {
    pub id: i32,
    pub sku_number: String,
    #[sqlx(rename = "type")]
    pub device_type: String,
    pub manufacturer: String,
    pub model: Option<String>,
    pub dce_serial_number: String,
    #[sqlx(flatten)]
    pub dimensions: Dimensions,
}

#[derive(Object)]
#[derive(Serialize, Deserialize)]
#[derive(FromRow)]
#[derive(Clone)]
pub struct NewProduct {
    pub sku_number: String,
    #[sqlx(rename = "type")]
    pub device_type: String,
    pub manufacturer: String,
    pub model: Option<String>,
    pub dce_serial_number: String,
    #[sqlx(flatten)]
    pub dimensions: Dimensions,
}

pub type Millimeters = f64;

#[derive(Object)]
#[derive(Serialize, Deserialize)]
#[derive(FromRow)]
#[derive(Clone)]
pub struct Dimensions {
    pub width: Millimeters,
    pub height: Millimeters,
    pub depth: Millimeters,
}
