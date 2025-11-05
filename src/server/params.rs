//! Request parameter types for path and query parameters.
//!
//! This module provides types for working with URL path parameters and query strings
//! in a type-safe manner.

use std::collections::HashMap;

/// Path parameters extracted from the URL.
///
/// Example: For route `/users/:id/:name`, accessing `/users/42/alice`:
/// - `path.get_str("id")` returns `Some("42")`
/// - `path.get_u64("id")` returns `Some(42)`
/// - `path.get_str("name")` returns `Some("alice")`
#[derive(Debug, Clone, Default)]
pub struct PathParams {
    params: HashMap<String, String>,
}

impl PathParams {
    /// Create new PathParams from a HashMap
    pub fn new(params: HashMap<String, String>) -> Self {
        Self { params }
    }

    /// Create empty PathParams
    pub fn empty() -> Self {
        Self {
            params: HashMap::new(),
        }
    }

    /// Get a parameter as a string slice
    pub fn get(&self, key: &str) -> Option<&str> {
        self.params.get(key).map(|s| s.as_str())
    }

    /// Get a parameter as a String
    pub fn get_string(&self, key: &str) -> Option<String> {
        self.params.get(key).cloned()
    }

    /// Get a parameter and parse as u64
    pub fn get_u64(&self, key: &str) -> Option<u64> {
        self.params.get(key)?.parse().ok()
    }

    /// Get a parameter and parse as i64
    pub fn get_i64(&self, key: &str) -> Option<i64> {
        self.params.get(key)?.parse().ok()
    }

    /// Get a parameter and parse as u32
    pub fn get_u32(&self, key: &str) -> Option<u32> {
        self.params.get(key)?.parse().ok()
    }

    /// Get a parameter and parse as i32
    pub fn get_i32(&self, key: &str) -> Option<i32> {
        self.params.get(key)?.parse().ok()
    }

    /// Get a parameter and parse as f64
    pub fn get_f64(&self, key: &str) -> Option<f64> {
        self.params.get(key)?.parse().ok()
    }

    /// Get a parameter and parse as bool
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.params.get(key)?.parse().ok()
    }

    /// Check if a parameter exists
    pub fn contains(&self, key: &str) -> bool {
        self.params.contains_key(key)
    }

    /// Get all parameter names
    pub fn keys(&self) -> Vec<&str> {
        self.params.keys().map(|s| s.as_str()).collect()
    }
}

/// Query parameters extracted from the URL query string.
///
/// Example: For URL `/users?page=2&limit=10`:
/// - `query.get("page")` returns `Some("2")`
/// - `query.get_u32("page")` returns `Some(2)`
/// - `query.get_u32("limit")` returns `Some(10)`
#[derive(Debug, Clone, Default)]
pub struct QueryParams {
    params: HashMap<String, String>,
}

impl QueryParams {
    /// Create new QueryParams from a HashMap
    pub fn new(params: HashMap<String, String>) -> Self {
        Self { params }
    }

    /// Create empty QueryParams
    pub fn empty() -> Self {
        Self {
            params: HashMap::new(),
        }
    }

    /// Get a parameter as a string slice
    pub fn get(&self, key: &str) -> Option<&str> {
        self.params.get(key).map(|s| s.as_str())
    }

    /// Get a parameter as a String
    pub fn get_string(&self, key: &str) -> Option<String> {
        self.params.get(key).cloned()
    }

    /// Get a parameter and parse as u64
    pub fn get_u64(&self, key: &str) -> Option<u64> {
        self.params.get(key)?.parse().ok()
    }

    /// Get a parameter and parse as i64
    pub fn get_i64(&self, key: &str) -> Option<i64> {
        self.params.get(key)?.parse().ok()
    }

    /// Get a parameter and parse as u32
    pub fn get_u32(&self, key: &str) -> Option<u32> {
        self.params.get(key)?.parse().ok()
    }

    /// Get a parameter and parse as i32
    pub fn get_i32(&self, key: &str) -> Option<i32> {
        self.params.get(key)?.parse().ok()
    }

    /// Get a parameter and parse as f64
    pub fn get_f64(&self, key: &str) -> Option<f64> {
        self.params.get(key)?.parse().ok()
    }

    /// Get a parameter and parse as bool
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.params.get(key)?.parse().ok()
    }

    /// Check if a parameter exists
    pub fn contains(&self, key: &str) -> bool {
        self.params.contains_key(key)
    }

    /// Get all parameter names
    pub fn keys(&self) -> Vec<&str> {
        self.params.keys().map(|s| s.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_params() {
        let mut map = HashMap::new();
        map.insert("id".to_string(), "42".to_string());
        map.insert("name".to_string(), "alice".to_string());

        let params = PathParams::new(map);

        assert_eq!(params.get("id"), Some("42"));
        assert_eq!(params.get_u64("id"), Some(42));
        assert_eq!(params.get("name"), Some("alice"));
        assert!(params.contains("id"));
        assert!(!params.contains("age"));
    }

    #[test]
    fn test_query_params() {
        let mut map = HashMap::new();
        map.insert("page".to_string(), "2".to_string());
        map.insert("limit".to_string(), "10".to_string());

        let params = QueryParams::new(map);

        assert_eq!(params.get_u32("page"), Some(2));
        assert_eq!(params.get_u32("limit"), Some(10));
        assert_eq!(params.get("page"), Some("2"));
    }
}
