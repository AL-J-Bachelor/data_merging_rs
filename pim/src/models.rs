use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Object)]
#[derive(Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
}