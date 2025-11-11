//! Testing utilities for integration tests
//!
//! This module provides a test client and utilities for testing Uncovr endpoints
//! without starting an actual HTTP server.
//!
//! # Example
//!
//! ```rust
//! use uncovr::testing::TestClient;
//! use uncovr::prelude::*;
//!
//! #[tokio::test]
//! async fn test_endpoint() {
//!     let client = TestClient::new()
//!         .register(MyEndpoint);
//!
//!     let response = client
//!         .get("/api/users/1")
//!         .send()
//!         .await;
//!
//!     response.assert_status(200);
//! }
//! ```

mod client;
mod request;
mod response;

pub use client::TestClient;
pub use request::RequestBuilder;
pub use response::TestResponse;
