use uncovr::{prelude::*, server::Server};

// path parameter
#[derive(Clone)]
pub struct GreetByName;

impl Metadata for GreetByName {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/greet/:name", "get").summary("Greet someone by name")
    }
}

#[async_trait]
impl API for GreetByName {
    type Req = ();
    type Res = String;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        let name = ctx.path.get("name").unwrap_or("stranger");
        format!("Hello, {}!", name)
    }
}

// query parameters
#[derive(Clone)]
pub struct ListItems;

impl Metadata for ListItems {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/items", "get")
            .summary("List items with pagination")
            .query("page")
            .desc("Page number")
            .query("limit")
            .desc("Items per page")
    }
}

#[async_trait]
impl API for ListItems {
    type Req = ();
    type Res = String;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        let page = ctx.query.get_u32("page").unwrap_or(1);
        let limit = ctx.query.get_u32("limit").unwrap_or(10);
        format!("Page {} with {} items per page", page, limit)
    }
}

// multiple path params
#[derive(Clone)]
pub struct GetUserPost;

impl Metadata for GetUserPost {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/users/:user_id/posts/:post_id", "get")
            .summary("Get a specific post from a user")
            .path_param("user_id")
            .desc("User ID")
            .path_param("post_id")
            .desc("Post ID")
    }
}

#[async_trait]
impl API for GetUserPost {
    type Req = ();
    type Res = String;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        let user_id = ctx.path.get_u64("user_id").unwrap_or(0);
        let post_id = ctx.path.get_u64("post_id").unwrap_or(0);
        format!("User {} - Post {}", user_id, post_id)
    }
}

#[tokio::main]
async fn main() {
    let config = AppConfig::new("Routes Example", "0.1.1")
        .bind("127.0.0.1:8000")
        .environment(Environment::Development);

    Server::new()
        .with_config(config)
        .register(GreetByName)
        .register(ListItems)
        .register(GetUserPost)
        .serve()
        .await
        .expect("Server Failed")
}
