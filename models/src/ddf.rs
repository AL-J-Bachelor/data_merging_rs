use serde::{Deserialize, Serialize};
use poem_openapi::Object;
use sqlx::FromRow;
use crate::pim::Product;

#[derive(Object)]
#[derive(Serialize, Deserialize)]
#[derive(FromRow)]
#[derive(Clone)]
pub struct DDF {
    pub id: String,
    #[sqlx(rename = "type")]
    pub device_type: String,
    pub sku_number: Option<String>,
    pub manufacturer: String,
    pub model: Option<String>,
    pub dce_serial: String,
}

#[derive(Object)]
#[derive(Serialize, Deserialize)]
#[derive(FromRow)]
#[derive(Clone)]
pub struct NewDDF {
    #[sqlx(rename = "type")]
    pub device_type: String,
    pub sku_number: Option<String>,
    pub manufacturer: String,
    pub model: Option<String>,
    pub dce_serial: String,
}

impl From<Product> for NewDDF {
    fn from(product: Product) -> Self {
        NewDDF {
            device_type: product.device_type,
            sku_number: Some(product.sku_number),
            manufacturer: product.manufacturer,
            model: product.model,
            dce_serial: product.dce_serial_number,
        }
    }
}
