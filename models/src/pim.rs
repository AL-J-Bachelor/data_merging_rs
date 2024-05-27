use poem_openapi::Object;
use serde::{Deserialize, Serialize};

pub type Millimeters = f32;

#[derive(Object)]
#[derive(Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub manufacturer: String,
    pub width: Millimeters,
    pub height: Millimeters,
    pub depth: Millimeters,
}