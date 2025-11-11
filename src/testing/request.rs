use crate::testing::TestResponse;
use axum::{Router, body::Body};
use http::{HeaderMap, HeaderValue, Method, Request, header};
use serde::Serialize;
use tower::ServiceExt;

/// Builder for creating test requests
///
/// # Example
///
/// ```rust
/// let response = client
///     .post("/users")
///     .json(&request_body)
///     .header("Authorization", "Bearer token")
///     .send()
///     .await;
/// ```
pub struct RequestBuilder {
    router: Router,
    method: Method,
    path: String,
    headers: HeaderMap,
    body: Option<Vec<u8>>,
}

impl RequestBuilder {
    pub(crate) fn new(router: Router, method: Method, path: &str) -> Self {
        Self {
            router,
            method,
            path: path.to_string(),
            headers: HeaderMap::new(),
            body: None,
        }
    }

    /// Set the request body as JSON
    ///
    /// # Example
    ///
    /// ```rust
    /// let response = client
    ///     .post("/users")
    ///     .json(&CreateUserRequest {
    ///         name: "Alice".to_string(),
    ///         email: "alice@example.com".to_string(),
    ///     })
    ///     .send()
    ///     .await;
    /// ```
    pub fn json<T: Serialize>(mut self, body: &T) -> Self {
        let json = serde_json::to_vec(body).expect("Failed to serialize JSON");
        self.headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        self.body = Some(json);
        self
    }

    /// Set a request header
    ///
    /// # Example
    ///
    /// ```rust
    /// let response = client
    ///     .get("/protected")
    ///     .header("Authorization", "Bearer token123")
    ///     .send()
    ///     .await;
    /// ```
    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(
            key.parse::<http::HeaderName>()
                .expect("Invalid header name"),
            value
                .parse::<http::HeaderValue>()
                .expect("Invalid header value"),
        );
        self
    }

    /// Set a Bearer token authorization header
    ///
    /// # Example
    ///
    /// ```rust
    /// let response = client
    ///     .get("/protected")
    ///     .bearer_token("my_jwt_token")
    ///     .send()
    ///     .await;
    /// ```
    pub fn bearer_token(self, token: &str) -> Self {
        self.header("Authorization", &format!("Bearer {}", token))
    }

    /// Send the request and get the response
    ///
    /// # Example
    ///
    /// ```rust
    /// let response = client
    ///     .get("/users/1")
    ///     .send()
    ///     .await;
    ///
    /// response.assert_status(200);
    /// ```
    pub async fn send(self) -> TestResponse {
        let body = self.body.unwrap_or_default();

        let mut request = Request::builder().method(self.method).uri(self.path);

        for (key, value) in self.headers.iter() {
            request = request.header(key, value);
        }

        let request = request
            .body(Body::from(body))
            .expect("Failed to build request");

        let response = self.router.oneshot(request).await.expect("Request failed");

        TestResponse::from_response(response).await
    }
}
