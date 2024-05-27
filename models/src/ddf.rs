use serde::{Deserialize, Serialize};
use poem_openapi::Object;

#[derive(Object)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct DDF {
    pub id: String,
    pub device_type: String,
    pub sku_number: Option<String>,
    pub manufacturer: String,
    pub model: Option<String>,
    pub dce_serial: Option<String>,
}

#[derive(Object)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct NewDDF {
    pub device_type: String,
    pub sku_number: Option<String>,
    pub manufacturer: String,
    pub model: Option<String>,
    pub dce_serial: Option<String>,
}