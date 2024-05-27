use poem_openapi::Object;
use serde::{Deserialize, Serialize};


#[derive(Object)]
#[derive(Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub sku_number: String,
    pub device_type: String,
    pub manufacturer: String,
    pub model: Option<String>,
    pub dce_serial_number: String,
    pub dimensions: Dimensions,
}

pub type Millimeters = f32;

#[derive(Object)]
#[derive(Serialize, Deserialize)]
pub struct Dimensions {
    pub width: Millimeters,
    pub height: Millimeters,
    pub depth: Millimeters,
}