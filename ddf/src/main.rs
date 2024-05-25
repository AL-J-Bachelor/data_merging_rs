mod endpoints;

use poem::{EndpointExt, listener::TcpListener, Route, Server};
use poem::middleware::{AddData, Cors};
use endpoints::Api;
use std::env;
use sqlx::PgPool;
use color_eyre::Result;
use poem_openapi::OpenApiService;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    let url = env::var("HOST_URL")?;

    let api_service =
        OpenApiService::new(Api, "DDF Service", env!("CARGO_PKG_VERSION"))
            .server(env::var("DOCUMENTATION_TARGET_URL")?);
    let openapi = api_service.openapi_explorer();
    let swagger = api_service.swagger_ui();
    let app = Route::new()
        .nest("/", api_service)
        .nest("/openapi", openapi)
        .nest("/swagger", swagger)
        .with(AddData::new(pool))
        .with(Cors::new());

    println!("Running server on {url}");
    Server::new(TcpListener::bind(url))
        .run(app)
        .await?;

    Ok(())
}