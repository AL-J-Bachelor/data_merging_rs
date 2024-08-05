use poem::error::InternalServerError;
use poem::Result;
use poem::web::Data;
use poem_openapi::OpenApi;
use poem_openapi::payload::PlainText;

use models::ddf::{DDF, NewDDF};
use models::pim::Product;

use crate::Urls;

pub struct Api;

#[OpenApi]
impl Api {
    /// Ping
    #[allow(clippy::unused_async)]
    #[oai(path = "/ping", method = "get")]
    pub async fn ping(&self) -> PlainText<&str> {
        PlainText("OK")
    }


    /// Synchronize services
    #[oai(path = "/sync", method = "post")]
    pub async fn sync(&self, urls: Data<&Urls>) -> Result<PlainText<String>> {
        let urls = urls.0;

        let client = reqwest::Client::new();

        let products_url = format!("{}/products", urls.pim);
        let products = client.get(products_url)
            .send()
            .await
            .map_err(InternalServerError)?
            .json::<Vec<Product>>()
            .await
            .map_err(InternalServerError)?;

        println!("Retrieved {} products", products.len());

        let new_ddfs: Vec<NewDDF> = products.iter().map(Product::clone).map(NewDDF::from).collect();
        let ddf_creation_url = format!("{}/ddfs/bulk", urls.ddf);
        let inserted_ddfs = client.post(ddf_creation_url)
            .json(&new_ddfs)
            .send()
            .await
            .map_err(InternalServerError)?
            .json::<Vec<DDF>>()
            .await
            .map_err(InternalServerError)?;

        println!("Inserted {} DDFs", inserted_ddfs.len());

        Ok(PlainText(format!("Synced {} products/DDFs", inserted_ddfs.len())))
    }
}
