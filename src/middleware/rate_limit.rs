use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tower::{Layer, Service};

/// Simple in-memory rate limiter middleware
///
/// Limits the number of requests from a single IP address within a time window.
/// Returns 429 (Too Many Requests) when the limit is exceeded.
///
/// **Note:** This is an in-memory rate limiter suitable for single-instance deployments.
/// For production multi-instance deployments, consider using a distributed rate limiter
/// with Redis or similar.
///
/// # Example
///
/// ```rust,no_run
/// use uncovr::server::Server;
/// use uncovr::middleware::RateLimit;
/// use std::time::Duration;
///
/// // Allow 100 requests per minute per IP
/// let server = Server::new()
///     .layer(RateLimit::new(100, Duration::from_secs(60)))
///     .register(MyEndpoint)
///     .build();
/// ```
#[derive(Clone)]
pub struct RateLimit {
    max_requests: usize,
    window: Duration,
    store: Arc<Mutex<RateLimitStore>>,
}

struct RateLimitStore {
    requests: HashMap<String, Vec<Instant>>,
}

impl RateLimit {
    /// Create a new rate limiter
    ///
    /// # Arguments
    ///
    /// * `max_requests` - Maximum number of requests allowed per window
    /// * `window` - Time window for rate limiting
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncovr::middleware::RateLimit;
    /// use std::time::Duration;
    ///
    /// // 60 requests per minute
    /// let limiter = RateLimit::new(60, Duration::from_secs(60));
    ///
    /// // 1000 requests per hour
    /// let limiter = RateLimit::new(1000, Duration::from_secs(3600));
    /// ```
    pub fn new(max_requests: usize, window: Duration) -> Self {
        Self {
            max_requests,
            window,
            store: Arc::new(Mutex::new(RateLimitStore {
                requests: HashMap::new(),
            })),
        }
    }

    fn check_rate_limit(&self, key: String) -> bool {
        let mut store = self.store.lock().unwrap();
        let now = Instant::now();
        let cutoff = now - self.window;

        // Get or create request history for this key
        let requests = store.requests.entry(key).or_insert_with(Vec::new);

        // Remove old requests outside the window
        requests.retain(|&time| time > cutoff);

        // Check if under limit
        if requests.len() < self.max_requests {
            requests.push(now);
            true
        } else {
            false
        }
    }
}

impl<S> Layer<S> for RateLimit {
    type Service = RateLimitService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RateLimitService {
            inner,
            limiter: self.clone(),
        }
    }
}

#[derive(Clone)]
pub struct RateLimitService<S> {
    inner: S,
    limiter: RateLimit,
}

impl<S> Service<Request<Body>> for RateLimitService<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        // Extract IP address from request
        let ip = req
            .headers()
            .get("x-forwarded-for")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.split(',').next())
            .unwrap_or("unknown")
            .to_string();

        // Check rate limit
        if !self.limiter.check_rate_limit(ip) {
            // Rate limit exceeded
            let response = (
                StatusCode::TOO_MANY_REQUESTS,
                "Rate limit exceeded. Please try again later.",
            )
                .into_response();

            return Box::pin(async move { Ok(response) });
        }

        // Continue with request
        let mut inner = self.inner.clone();
        Box::pin(async move { inner.call(req).await })
    }
}
