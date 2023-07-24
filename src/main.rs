// Adapted from https://docs.rs/poem-openapi/latest/poem_openapi/#modules
use poem::{Route, listener::TcpListener, Server};
use poem_openapi::{OpenApi, payload::PlainText, OpenApiService};

struct Api;

#[OpenApi]
impl Api {
    /// Hello World
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<&'static str> {
        PlainText("Hello World")
    }
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server("http://localhost:3000");
    // let ui = api_service.swagger_ui();
    let ui = api_service.rapidoc();
    // let ui = api_service.redoc();
    let app = Route::new().nest("/", api_service).nest("/docs", ui);

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
        .expect("Didn't run for some reason");
}
