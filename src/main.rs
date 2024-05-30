use models::ddf::*;
use models::gps::*;
use models::pim::*;
use color_eyre::Result;
use reqwest;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let client = reqwest::Client::new();



    let products_url = "http://localhost:7300/products";
    let products = client.get(products_url)
        .send()
        .await?
        .json::<Vec<Product>>()
        .await?;

    println!("Retrieved products: {}", products.len());

    let new_ddfs: Vec<NewDDF> = products.iter().map(Product::clone).map(NewDDF::from).collect();

    let inserted_ddfs = client.post("http://localhost:7100/ddfs/bulk")
        .json(&new_ddfs)
        .send()
        .await?
        .json::<Vec<DDF>>()
        .await?;

    println!("Inserted products: {}", inserted_ddfs.len());

    Ok(())
}
