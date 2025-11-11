use axum::body::Body;
use bytes::Bytes;
use http::{HeaderMap, Response, StatusCode};
use serde::de::DeserializeOwned;

/// Test response with assertion helpers
///
/// # Example
///
/// ```rust
/// let response = client.get("/users/1").send().await;
///
/// response.assert_status(200);
/// response.assert_json::<User>(|user| {
///     assert_eq!(user.name, "Alice");
/// });
/// ```
pub struct TestResponse {
    status: StatusCode,
    headers: HeaderMap,
    body: Bytes,
}

impl TestResponse {
    pub(crate) async fn from_response(response: Response<Body>) -> Self {
        let status = response.status();
        let headers = response.headers().clone();

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to read response body");

        Self {
            status,
            headers,
            body,
        }
    }

    /// Get the response status code
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// Get the response headers
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Get the raw response body bytes
    pub fn body(&self) -> &Bytes {
        &self.body
    }

    /// Assert that the response has the expected status code
    ///
    /// # Panics
    ///
    /// Panics if the status code doesn't match
    ///
    /// # Example
    ///
    /// ```rust
    /// response.assert_status(200);
    /// response.assert_status(404);
    /// ```
    pub fn assert_status(&self, expected: u16) {
        assert_eq!(
            self.status.as_u16(),
            expected,
            "Expected status {}, got {}. Body: {}",
            expected,
            self.status.as_u16(),
            String::from_utf8_lossy(&self.body)
        );
    }

    /// Deserialize the response body as JSON
    ///
    /// # Panics
    ///
    /// Panics if the body is not valid JSON or cannot be deserialized
    ///
    /// # Example
    ///
    /// ```rust
    /// let user: User = response.json();
    /// assert_eq!(user.name, "Alice");
    /// ```
    pub fn json<T: DeserializeOwned>(&self) -> T {
        serde_json::from_slice(&self.body).unwrap_or_else(|e| {
            panic!(
                "Failed to deserialize JSON: {}. Body: {}",
                e,
                String::from_utf8_lossy(&self.body)
            )
        })
    }

    /// Assert the response body as JSON with a custom assertion function
    ///
    /// # Panics
    ///
    /// Panics if the body is not valid JSON or the assertion fails
    ///
    /// # Example
    ///
    /// ```rust
    /// response.assert_json::<User>(|user| {
    ///     assert_eq!(user.name, "Alice");
    ///     assert!(user.email.contains("@"));
    /// });
    /// ```
    pub fn assert_json<T: DeserializeOwned>(&self, f: impl FnOnce(T)) {
        let value: T = self.json();
        f(value);
    }

    /// Get the response body as a UTF-8 string
    ///
    /// # Panics
    ///
    /// Panics if the body is not valid UTF-8
    ///
    /// # Example
    ///
    /// ```rust
    /// let text = response.text();
    /// assert!(text.contains("Hello"));
    /// ```
    pub fn text(&self) -> String {
        String::from_utf8(self.body.to_vec()).expect("Response body is not valid UTF-8")
    }

    /// Assert that the response body contains a substring
    ///
    /// # Panics
    ///
    /// Panics if the body doesn't contain the expected text
    ///
    /// # Example
    ///
    /// ```rust
    /// response.assert_text_contains("Success");
    /// ```
    pub fn assert_text_contains(&self, expected: &str) {
        let text = self.text();
        assert!(
            text.contains(expected),
            "Expected body to contain '{}', got: {}",
            expected,
            text
        );
    }

    /// Assert that a header has the expected value
    ///
    /// # Panics
    ///
    /// Panics if the header is missing or has a different value
    ///
    /// # Example
    ///
    /// ```rust
    /// response.assert_header("content-type", "application/json");
    /// ```
    pub fn assert_header(&self, key: &str, expected: &str) {
        let value = self
            .headers
            .get(key)
            .unwrap_or_else(|| panic!("Header '{}' not found", key))
            .to_str()
            .expect("Header value is not valid UTF-8");

        assert_eq!(
            value, expected,
            "Expected header '{}' to be '{}', got '{}'",
            key, expected, value
        );
    }

    /// Check if the response is successful (2xx status code)
    ///
    /// # Example
    ///
    /// ```rust
    /// if response.is_success() {
    ///     println!("Request succeeded!");
    /// }
    /// ```
    pub fn is_success(&self) -> bool {
        self.status.is_success()
    }

    /// Check if the response is a client error (4xx status code)
    pub fn is_client_error(&self) -> bool {
        self.status.is_client_error()
    }

    /// Check if the response is a server error (5xx status code)
    pub fn is_server_error(&self) -> bool {
        self.status.is_server_error()
    }
}
