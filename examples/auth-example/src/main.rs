use std::net::SocketAddr;
use uncovr::prelude::*;

// Public endpoint - no authentication required
#[derive(Clone)]
struct PublicEndpoint;

impl Endpoint for PublicEndpoint {
    fn route(&self) -> Route {
        Route::get("/public")
    }

    fn meta(&self) -> Meta {
        Meta::new()
            .summary("Public endpoint")
            .description("Accessible without authentication")
            .tag("public")
            .public()
    }
}

#[async_trait]
impl Handler for PublicEndpoint {
    type Request = ();
    type Response = &'static str;

    async fn handle(&self, _ctx: Context<Self::Request>) -> Self::Response {
        "This endpoint is publicly accessible"
    }
}

// Protected endpoint - requires Bearer token
#[derive(Clone)]
struct ProtectedEndpoint;

impl Endpoint for ProtectedEndpoint {
    fn route(&self) -> Route {
        Route::get("/protected")
    }

    fn meta(&self) -> Meta {
        Meta::new()
            .summary("Protected endpoint")
            .description("Requires Bearer token authentication")
            .tag("protected")
            .auth_required()
    }
}

#[async_trait]
impl Handler for ProtectedEndpoint {
    type Request = ();
    type Response = &'static str;

    async fn handle(&self, _ctx: Context<Self::Request>) -> Self::Response {
        "This endpoint requires authentication"
    }
}

// API key protected endpoint
#[derive(Clone)]
struct ApiKeyEndpoint;

impl Endpoint for ApiKeyEndpoint {
    fn route(&self) -> Route {
        Route::get("/api-key")
    }

    fn meta(&self) -> Meta {
        Meta::new()
            .summary("API Key endpoint")
            .description("Requires API key in header")
            .tag("protected")
            .security(SecurityScheme::ApiKey {
                name: "X-API-Key",
                location: ApiKeyLocation::Header,
            })
    }
}

#[async_trait]
impl Handler for ApiKeyEndpoint {
    type Request = ();
    type Response = &'static str;

    async fn handle(&self, _ctx: Context<Self::Request>) -> Self::Response {
        "This endpoint requires an API key"
    }
}

// Multi-auth endpoint
#[derive(Clone)]
struct MultiAuthEndpoint;

impl Endpoint for MultiAuthEndpoint {
    fn route(&self) -> Route {
        Route::get("/multi-auth")
    }

    fn meta(&self) -> Meta {
        Meta::new()
            .summary("Multiple authentication methods")
            .description("Requires both Bearer token and API key")
            .tag("protected")
            .security(SecurityScheme::Bearer)
            .security(SecurityScheme::ApiKey {
                name: "X-API-Key",
                location: ApiKeyLocation::Header,
            })
    }
}

#[async_trait]
impl Handler for MultiAuthEndpoint {
    type Request = ();
    type Response = &'static str;

    async fn handle(&self, _ctx: Context<Self::Request>) -> Self::Response {
        "This endpoint requires multiple authentication methods"
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let app = App::new("Authentication Example", "1.0.0", addr.to_string())
        .description("Demonstrates security documentation in OpenAPI");

    uncovr::server::Server::new()
        .with_config(app)
        .with_logging(Logging::development())
        .register(PublicEndpoint)
        .register(ProtectedEndpoint)
        .register(ApiKeyEndpoint)
        .register(MultiAuthEndpoint)
        .serve()
        .await
        .expect("Server failed");
}
