use std::path::Path;
use std::sync::{Arc, RwLock};
use tera::Tera;
use uncovr::prelude::*;
use uncovr::response::Html;
use uncovr::server::Server;

#[derive(Clone)]
struct AppState {
    tera: Arc<RwLock<Tera>>,
}

impl AppState {
    fn new() -> Self {
        let tera = Tera::new("templates/**/*").expect("Failed to initialize Tera");
        Self {
            tera: Arc::new(RwLock::new(tera)),
        }
    }

    fn render(&self, template: &str, context: &tera::Context) -> Result<String, String> {
        let mut tera = self.tera.write().unwrap();
        tera.full_reload().ok();
        tera.render(template, context)
            .map_err(|e| format!("Template error: {}", e))
    }
}

#[derive(Clone)]
pub struct IndexEndpoint {
    state: AppState,
}

impl Endpoint for IndexEndpoint {
    fn ep(&self) -> Route {
        Route::GET("/")
    }
}

#[async_trait]
impl API for IndexEndpoint {
    type Req = ();
    type Res = Html<String>;

    async fn handler(&self, _ctx: Context<Self::Req>) -> Self::Res {
        let mut context = tera::Context::new();
        context.insert("title", "Hands on Live Reload with Tera");
        context.insert(
            "message",
            "Edit templates/index.html and save to see changes",
        );

        let html = self
            .state
            .render("index.html", &context)
            .unwrap_or_else(|e| format!("<h1>Error</h1><p>{}</p>", e));

        Html(html)
    }
}

#[tokio::main]
async fn main() {
    if Path::new("examples/live-reload").exists() {
        std::env::set_current_dir("examples/live-reload").unwrap();
    }

    let config = AppConfig::new("Live Reload Demo", "1.0.0")
        .environment(Environment::Development)
        .logging(LoggingConfig::development());

    let state = AppState::new();

    let mut server = Server::new()
        .with_config(config)
        .register(IndexEndpoint { state });

    #[cfg(debug_assertions)]
    {
        use tower_livereload::LiveReloadLayer;
        server = server.layer(LiveReloadLayer::new());
    }

    server
        .build()
        .serve()
        .await
        .expect("Failed to start server");
}
