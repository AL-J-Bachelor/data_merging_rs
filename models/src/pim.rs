use poem_openapi::Object;
use serde::{Deserialize, Serialize};

pub type Millimeters = f64;

#[derive(Object)]
#[derive(Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub sku_number: String,
    pub device_type: String,
    pub manufacturer: String,
    pub model: Option<String>,
    pub dce_serial_number: String,
    pub width: Millimeters,
    pub height: Millimeters,
    pub depth: Millimeters,
}
