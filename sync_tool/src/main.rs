mod endpoints;

use poem::{EndpointExt, listener::TcpListener, Route, Server};
use poem::middleware::{AddData, Cors};
use endpoints::Api;
use std::env;
use color_eyre::Result;
use poem_openapi::OpenApiService;

#[derive(Clone)]
struct Urls {
    ddf: String,
    gps: String,
    pim: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let host_url = env::var("HOST_URL")?;

    let urls = Urls {
        ddf: env::var("DDF_BASE_URL")?,
        gps: env::var("GPS_BASE_URL")?,
        pim: env::var("PIM_BASE_URL")?,
    };

    let client = reqwest::Client::new();
    for url in [&urls.ddf, &urls.gps, &urls.pim] {
        let url = format!("{url}/ping");
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