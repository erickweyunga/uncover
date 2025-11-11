use crate::testing::RequestBuilder;
use axum::Router;
use http::Method;

/// Test client for making requests to endpoints without starting a server
///
/// # Example
///
/// ```rust
/// use uncovr::testing::TestClient;
/// use uncovr::prelude::*;
///
/// #[tokio::test]
/// async fn test_create_user() {
///     let server = Server::new()
///         .register(CreateUserEndpoint)
///         .build();
///
///     let client = TestClient::from_server(server);
///
///     let response = client
///         .post("/users")
///         .json(&CreateUserRequest {
///             name: "Alice".to_string(),
///             email: "alice@example.com".to_string(),
///         })
///         .send()
///         .await;
///
///     response.assert_status(201);
/// }
/// ```
pub struct TestClient {
    router: Router,
}

impl TestClient {
    /// Create a test client from a built server
    ///
    /// # Example
    ///
    /// ```rust
    /// let server = Server::new()
    ///     .with_state(app_state)
    ///     .register(MyEndpoint)
    ///     .build();
    ///
    /// let client = TestClient::from_server(server);
    /// ```
    pub fn from_server(server: crate::server::Server) -> Self {
        Self {
            router: server.into_router().into(),
        }
    }

    /// Create a GET request
    pub fn get(&self, path: &str) -> RequestBuilder {
        RequestBuilder::new(self.router.clone(), Method::GET, path)
    }

    /// Create a POST request
    pub fn post(&self, path: &str) -> RequestBuilder {
        RequestBuilder::new(self.router.clone(), Method::POST, path)
    }

    /// Create a PUT request
    pub fn put(&self, path: &str) -> RequestBuilder {
        RequestBuilder::new(self.router.clone(), Method::PUT, path)
    }

    /// Create a PATCH request
    pub fn patch(&self, path: &str) -> RequestBuilder {
        RequestBuilder::new(self.router.clone(), Method::PATCH, path)
    }

    /// Create a DELETE request
    pub fn delete(&self, path: &str) -> RequestBuilder {
        RequestBuilder::new(self.router.clone(), Method::DELETE, path)
    }
}
