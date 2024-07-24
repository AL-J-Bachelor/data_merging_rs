mod endpoints;

use poem::{EndpointExt, listener::TcpListener, Route, Server};
use poem::middleware::{AddData, Cors};
use endpoints::Api;
use std::env;
use color_eyre::Result;
use poem_openapi::OpenApiService;

#[derive(Clone)]
struct Urls {
    ddf_base_url: String,
    gps_base_url: String,
    pim_base_url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let host_url = env::var("HOST_URL")?;

    let urls = Urls {
        ddf_base_url: env::var("DDF_BASE_URL")?,
        gps_base_url: env::var("GPS_BASE_URL")?,
        pim_base_url: env::var("PIM_BASE_URL")?,
    };

    let client = reqwest::Client::new();
    for url in [&urls.ddf_base_url, &urls.gps_base_url, &urls.pim_base_url] {
        let url = format!("{}/ping", url);
        let response_text = client
            .get(url)
            .send()
            .await?
            .text()
            .await?;
        assert_eq!(response_text, "OK");
    }

    let api_service =
        OpenApiService::new(Api, "Data Sync Service", env!("CARGO_PKG_VERSION"))
            .server(env::var("DOCUMENTATION_TARGET_URL")?);
    let openapi = api_service.openapi_explorer();
    let swagger = api_service.swagger_ui();
    let app = Route::new()
        .nest("/", api_service)
        .nest("/openapi", openapi)
        .nest("/swagger", swagger)
        .with(AddData::new(urls))
        .with(Cors::new());

    println!("Running server on {host_url}");
    Server::new(TcpListener::bind(host_url))
        .run(app)
        .await?;

    Ok(())
}