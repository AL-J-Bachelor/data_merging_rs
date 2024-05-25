use models::ddf::*;
use models::gps::*;
use models::pim::*;
use color_eyre::Result;
use reqwest;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    println!("Hello, world!");
    let products_url = "http://localhost:7300/products";
    let products = reqwest::get(products_url)
        .await?
        .json::<Vec<Product>>()
        .await?;


    // let new_ddfs = products.map(NewDDF::from).collect();
    //
    // let ddf_conn = ddf::get_pool().await?;
    // let matching_ddfs = ddf::get_matching_ddfs(&ddf_conn, new_ddfs).await?;

    Ok(())
}
