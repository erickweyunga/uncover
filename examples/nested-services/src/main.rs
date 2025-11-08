use tower_http::services::ServeDir;
use uncovr::prelude::*;
use uncovr::routing::get_service;

#[derive(Clone)]
pub struct Root;

impl Endpoint for Root {
    fn ep(&self) -> Route {
        Route::GET("/")
    }

    fn docs(&self) -> Option<Docs> {
        Some(
            Docs::new()
                .summary("API Information")
                .description("Get information about the API")
                .tag("info"),
        )
    }
}

#[async_trait]
impl API for Root {
    type Req = ();
    type Res = Json<serde_json::Value>;

    async fn handler(&self, _ctx: Context<Self::Req>) -> Self::Res {
        Json(serde_json::json!({
            "message": "Welcome to the Nested Services Example",
            "endpoints": {
                "static": "/public - Static file serving",
                "docs": "/docs - API documentation"
            }
        }))
    }
}

#[tokio::main]
async fn main() {
    let config = AppConfig::new("Nested Services API", "1.0.0")
        .description("Example demonstrating nest_service for static file serving")
        .logging(LoggingConfig::development());

    uncovr::server::Server::new()
        .with_config(config)
        .register(Root)
        .nest_service("/public", get_service(ServeDir::new("public")))
        .serve()
        .await
        .expect("Server failed");
}
