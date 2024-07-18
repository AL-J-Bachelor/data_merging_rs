use poem_openapi::OpenApi;
use poem_openapi::payload::PlainText;

pub struct Api;

#[OpenApi]
impl Api {
    /// Ping
    #[oai(path = "/ping", method = "get")]
    pub async fn ping(&self) -> PlainText<&str> {
        PlainText("OK")
    }
}