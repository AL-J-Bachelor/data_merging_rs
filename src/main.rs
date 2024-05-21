use pim;
use pim::Product;
use gps;
use ddf;
use ddf::NewDDF;
use color_eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    // let pim_conn: pim::get_pool().await?;
    // let products: Vec<Product> = pim::get_all_products(pim_conn).await?;
    //
    // let new_ddfs = products.map(NewDDF::from).collect();
    //
    // let ddf_conn = ddf::get_pool().await?;
    // let matching_ddfs = ddf::get_matching_ddfs(&ddf_conn, new_ddfs).await?;

    Ok(())
}
