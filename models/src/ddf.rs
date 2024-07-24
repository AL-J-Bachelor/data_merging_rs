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
#[derive(PartialEq, Eq)]
#[derive(Clone)]
#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use crate::ddf::NewDDF;
    use crate::pim::{Dimensions, Product};

    #[test]
    fn full_product_yields_full_ddf() {
        let full_product = Product {
            id: 0,
            sku_number: "123".to_string(),
            device_type: "456".to_string(),
            manufacturer: "789".to_string(),
            model: Some("012".to_string()),
            dce_serial_number: "345".to_string(),
            dimensions: Dimensions {
                width: 1.2,
                height: 3.4,
                depth: 5.6,
            },
        };

        let actual: NewDDF = full_product.into();

        let expected = NewDDF {
            sku_number: Some("123".to_string()),
            device_type: "456".to_string(),
            manufacturer: "789".to_string(),
            model: Some("012".to_string()),
            dce_serial: "345".to_string(),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn product_with_empty_model_yields_ddf_with_empty_model() {
        let full_product = Product {
            id: 0,
            sku_number: "123".to_string(),
            device_type: "456".to_string(),
            manufacturer: "789".to_string(),
            model: None,
            dce_serial_number: "345".to_string(),
            dimensions: Dimensions {
                width: 1.2,
                height: 3.4,
                depth: 5.6,
            },
        };

        let actual: NewDDF = full_product.into();

        let expected = NewDDF {
            sku_number: Some("123".to_string()),
            device_type: "456".to_string(),
            manufacturer: "789".to_string(),
            model: None,
            dce_serial: "345".to_string(),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn random_products_yields_correct_ddfs() {
        for _ in 0..100 {
            let product = Product::generate_random();

            let actual: NewDDF = product.clone().into();

            assert_eq!(product.sku_number, actual.sku_number.unwrap());
            assert_eq!(product.device_type, actual.device_type);
            assert_eq!(product.manufacturer, actual.manufacturer);
            assert_eq!(product.model, actual.model);
            assert_eq!(product.dce_serial_number, actual.dce_serial);
        }
    }
}