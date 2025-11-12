use schemars::JsonSchema;
use std::net::SocketAddr;
use uncovr::handle;
use uncovr::prelude::*;

#[derive(Default, Deserialize, Serialize, JsonSchema, Validate)]
pub struct CreateUserRequest {
    #[validate(length(
        min = 1,
        max = 50,
        message = "Name must be between 1 and 50 characters"
    ))]
    pub name: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    #[validate(range(min = 18, max = 120, message = "Age must be between 18 and 120"))]
    pub age: u8,
}

#[derive(Serialize, JsonSchema)]
pub struct UserResponse {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub age: u8,
}

#[derive(Clone)]
pub struct CreateUser;

impl Endpoint for CreateUser {
    fn route(&self) -> Route {
        Route::post("/users")
    }

    fn meta(&self) -> Meta {
        Meta::new()
            .summary("Create a new user")
            .description("Creates a new user with validation")
            .tag("users")
    }
}

#[async_trait]
impl Handler for CreateUser {
    type Request = CreateUserRequest;
    type Response = HandlerResult<UserResponse>;

    async fn handle(&self, ctx: Context<Self::Request>) -> Self::Response {
        handle! {
            // Validate the request - errors auto-convert to HTTP 422
            ctx.req.validate()?;

            // Business logic
            let user = UserResponse {
                id: 1,
                name: ctx.req.name,
                email: ctx.req.email,
                age: ctx.req.age,
            };

            Ok(user)
        }
    }
}

// Example endpoint without validation
#[derive(Default, Deserialize, Serialize, JsonSchema)]
pub struct SimpleRequest {
    pub message: String,
}

#[derive(Clone)]
pub struct SimpleEndpoint;

impl Endpoint for SimpleEndpoint {
    fn route(&self) -> Route {
        Route::post("/simple")
    }

    fn meta(&self) -> Meta {
        Meta::new()
            .summary("Simple endpoint without validation")
            .tag("simple")
    }
}

#[async_trait]
impl Handler for SimpleEndpoint {
    type Request = SimpleRequest;
    type Response = Json<SimpleRequest>;

    async fn handle(&self, ctx: Context<Self::Request>) -> Self::Response {
        Json(ctx.req)
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let app = App::new("Validation Example", "1.0.0", addr.to_string())
        .description("Demonstrates request validation in uncovr");

    uncovr::server::Server::new()
        .with_config(app)
        .with_logging(Logging::development())
        .register(CreateUser)
        .register(SimpleEndpoint)
        .serve()
        .await
        .expect("Server failed");
}
