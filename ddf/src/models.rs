use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DDF {
    pub id: String,
    pub device_type: String,
    pub sku_number: Option<String>,
    pub manufacturer: String,
    pub model: Option<String>,
    pub dce_serial: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewDDF {
    pub device_type: String,
    pub sku_number: Option<String>,
    pub manufacturer: String,
    pub model: Option<String>,
    pub dce_serial: String,
}