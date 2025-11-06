# Routes

Understanding how routing works in Uncovr and how to build structured APIs.

## The Concept of Routing

When a client makes an HTTP request to your API, the router's job is to determine which piece of code should handle that request. This decision is based on two factors: the URL path and the HTTP method.

Think of routing like a postal system. The URL path is the address, and the HTTP method (GET, POST, etc.) is the type of mail service. The router looks at both and delivers the request to the correct handler.

## How Routes Work in Uncovr

In Uncovr, you define routes through the `Metadata` trait. This trait tells the framework three essential pieces of information:

1. **The path**: Where should requests go? (e.g., `/users`)
2. **The HTTP method**: What kind of operation? (e.g., `get`, `post`)
3. **Documentation**: What does this endpoint do? (for OpenAPI docs)

Here's a complete example:

```rust
use uncovr::prelude::*;

#[derive(Clone)]
pub struct GetUser;

impl Metadata for GetUser {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/users/:id", "get")
            .summary("Retrieve a user by ID")
    }
}
```

When you register this endpoint, Uncovr knows: "When someone makes a GET request to `/users/:id`, call this handler."

## HTTP Methods and Their Purpose

HTTP methods indicate the intent of the request. Understanding when to use each one is crucial for building intuitive APIs.

### GET - Retrieving Data

GET requests fetch data without modifying anything on the server. They should be safe to call multiple times with the same result.

```rust
Endpoint::new("/users", "get")
    .summary("List all users")
```

**Key characteristic**: Idempotent and safe. Calling it 100 times has the same effect as calling it once.

### POST - Creating New Resources

POST creates a new resource. The server typically assigns a new ID and returns the created resource.

```rust
Endpoint::new("/users", "post")
    .summary("Create a new user")
```

**Key characteristic**: Not idempotent. Calling it twice creates two resources.

### PUT - Replacing a Resource

PUT replaces an entire resource. You're saying "make this resource look exactly like what I'm sending."

```rust
Endpoint::new("/users/:id", "put")
    .summary("Replace user data completely")
```

**Key characteristic**: Idempotent. Sending the same PUT twice results in the same final state.

### PATCH - Partial Updates

PATCH modifies part of a resource without replacing the whole thing.

```rust
Endpoint::new("/users/:id", "patch")
    .summary("Update specific user fields")
```

**Key characteristic**: Not necessarily idempotent, depends on the patch operations.

### DELETE - Removing Resources

DELETE removes a resource from the system.

```rust
Endpoint::new("/users/:id", "delete")
    .summary("Remove a user")
```

**Key characteristic**: Idempotent. Deleting twice has the same effect as deleting once (the resource is gone).

## Working with Path Parameters

Path parameters let you capture values directly from the URL. They're declared in the path using the `:name` syntax.

### Understanding Path Parameters

When you write `/users/:id`, the `:id` part is a placeholder. Any value in that position gets captured and made available to your handler.

Request: `GET /users/42`
The value `42` is captured as the `id` parameter.

Request: `GET /users/sarah`
The value `sarah` is captured as the `id` parameter.

### Accessing Path Parameters

Path parameters are accessed through `ctx.path` in your handler:

```rust
async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
    // Type-safe extraction
    if let Some(user_id) = ctx.path.get_u64("id") {
        // user_id is a u64
    }
    
    // Or as a string
    if let Some(id_str) = ctx.path.get_string("id") {
        // id_str is a String
    }
}
```

Available conversion methods:
- `get_string(name)` - Extract as String
- `get_u64(name)` - Extract as unsigned 64-bit integer
- `get_i64(name)` - Extract as signed 64-bit integer  
- `get_f64(name)` - Extract as floating-point number
- `get_bool(name)` - Extract as boolean

These methods return `Option<T>`, so you can handle missing or invalid parameters gracefully.

### Declaring Path Parameters for Documentation

Tell OpenAPI about your path parameters using `.path_param()`:

```rust
impl Metadata for GetUser {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/users/:id", "get")
            .summary("Get a user by ID")
            .path_param("id").desc("Unique user identifier")
    }
}
```

This makes your API documentation clear about what parameters are expected.

### Multiple Path Parameters

You can have several parameters in one path:

```rust
Endpoint::new("/users/:user_id/posts/:post_id", "get")
    .path_param("user_id").desc("The user's ID")
    .path_param("post_id").desc("The post's ID")
```

Access them the same way:

```rust
let user_id = ctx.path.get_u64("user_id");
let post_id = ctx.path.get_u64("post_id");
```

## Working with Query Parameters

Query parameters come after a `?` in the URL and are used for optional data like filters, pagination, or search terms.

### Understanding Query Parameters

Query parameters are key-value pairs appended to the URL:

```
GET /users?page=2&limit=10&sort=name
```

Here we have three query parameters:
- `page` with value `2`
- `limit` with value `10`
- `sort` with value `name`

### Accessing Query Parameters

Query parameters are accessed through `ctx.query`:

```rust
async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
    let page = ctx.query.get_u64("page").unwrap_or(1);
    let limit = ctx.query.get_u64("limit").unwrap_or(20);
    let sort = ctx.query.get_string("sort").unwrap_or_else(|| "id".to_string());
    
    // Use these values to filter/paginate your results
}
```

The same conversion methods work as with path parameters.

### Declaring Query Parameters for Documentation

Use `.query()` to document query parameters:

```rust
impl Metadata for ListUsers {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/users", "get")
            .summary("List users with optional filtering")
            .query("page").desc("Page number for pagination")
            .query("limit").desc("Number of users per page")
            .query("sort").desc("Field to sort by")
    }
}
```

### Making Query Parameters Required

By default, query parameters are optional. Mark them as required with `.required()`:

```rust
Endpoint::new("/search", "get")
    .summary("Search users")
    .query("q").desc("Search query").required()
    .query("page").desc("Page number")  // Optional
```

The `.required()` method applies to the most recently added parameter.

## The Difference: Path vs Query vs Body

Understanding when to use each type of parameter is important for clear API design.

### Path Parameters

Use for identifying a specific resource:
- `/users/:id` - The ID identifies which user
- `/posts/:slug` - The slug identifies which post

**Rule of thumb**: If removing it makes the URL meaningless, it should be a path parameter.

### Query Parameters

Use for optional data that filters or modifies the response:
- `/users?role=admin` - Filter users by role
- `/posts?published=true&sort=date` - Filter and sort posts

**Rule of thumb**: If removing it still makes sense (you get a default behavior), it should be a query parameter.

### Request Body

Use for data that's being created or updated:
- `POST /users` with user data in body - Creating a user
- `PUT /users/:id` with updated fields in body - Updating a user

**Rule of thumb**: If you're sending data TO the server to store, it goes in the body.

## Path Matching Priority

When multiple routes could match a URL, Uncovr follows this priority order:

1. **Exact string matches** - `/users/new`
2. **Path parameters** - `/users/:id`
3. **Catch-all wildcards** - `/users/*path`

Example scenario:

```rust
// Route 1: Exact match
Endpoint::new("/users/new", "get")

// Route 2: Path parameter
Endpoint::new("/users/:id", "get")
```

Request to `/users/new` will match Route 1, not Route 2.
Request to `/users/123` will match Route 2.

This means you can have specialized handlers for specific paths while still catching all other values with parameters.

## Organizing Your Routes

As your API grows, organizing routes becomes important for maintainability. We recommend the structure used in the url-shortener example, which provides a clean separation of concerns.

### Recommended Structure: Feature Modules with Separated Concerns

Organize your code by feature/resource, with a clear separation between API definitions and handler implementations:

```
src/
├── main.rs              # Entry point, server setup
├── helpers.rs           # Shared utility functions (optional)
└── users/               # Feature module
    ├── mod.rs           # Module exports
    ├── apis.rs          # API metadata (routes, types, documentation)
    └── handlers.rs      # Business logic (handler implementations)
```

#### What Goes Where

**apis.rs** - API Definitions and Metadata:
```rust
use uncovr::prelude::*;

// Request/Response types
#[derive(Deserialize, JsonSchema, Default)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, JsonSchema)]
pub struct UserResponse {
    pub id: u64,
    pub name: String,
}

// API endpoint structs
#[derive(Clone)]
pub struct CreateUserApi;

#[derive(Clone)]
pub struct GetUserApi;

// Metadata implementations (routes, summaries, response docs)
impl Metadata for CreateUserApi {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/users", "post")
            .summary("Create a new user")
            .with_responses(|op| {
                op.response::<201, Json<UserResponse>>()
                    .response::<400, Json<ErrorResponse>>()
            })
    }
}

impl Metadata for GetUserApi {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/users/:id", "get")
            .summary("Get user by ID")
            .path_param("id").desc("User ID")
            .with_responses(|op| {
                op.response::<200, Json<UserResponse>>()
                    .response::<404, Json<ErrorResponse>>()
            })
    }
}
```

**handlers.rs** - Business Logic:
```rust
use crate::users::apis::{CreateUserApi, CreateUserRequest, GetUserApi, UserResponse};
use uncovr::prelude::*;

#[async_trait]
impl API for CreateUserApi {
    type Req = CreateUserRequest;
    type Res = ApiResponse<UserResponse>;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        // Validate input
        if ctx.req.name.is_empty() {
            return ApiResponse::BadRequest {
                code: "empty_name",
                message: "Name cannot be empty",
            };
        }

        // Business logic here
        let user = UserResponse {
            id: 1,
            name: ctx.req.name,
        };

        ApiResponse::Created(user)
    }
}

#[async_trait]
impl API for GetUserApi {
    type Req = ();
    type Res = ApiResponse<UserResponse>;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        let id = ctx.path.get_u64("id").unwrap_or(0);
        
        // Fetch user logic here
        if id == 0 {
            return ApiResponse::NotFound {
                code: "user_not_found",
                message: "User not found",
            };
        }

        ApiResponse::Ok(UserResponse {
            id,
            name: "John Doe".to_string(),
        })
    }
}
```

**mod.rs** - Module Exports:
```rust
pub mod apis;
pub mod handlers;
```

**main.rs** - Server Setup:
```rust
use uncovr::{prelude::*, server::Server};
use crate::users::apis::{CreateUserApi, GetUserApi};

mod users;

#[tokio::main]
async fn main() {
    let config = AppConfig::new("My API", "1.0.0")
        .bind("0.0.0.0:3000");

    Server::new()
        .with_config(config)
        .register(CreateUserApi)
        .register(GetUserApi)
        .serve()
        .await
        .expect("Server failed to start")
}
```

### Why This Structure Works

**1. Clear Separation of Concerns**
- API definitions (routes, types, docs) are separate from business logic
- Easy to see all your API endpoints at a glance in `apis.rs`
- Business logic is isolated in `handlers.rs`

**2. Scalable**
- Add new features by creating new modules (e.g., `posts/`, `comments/`)
- Each module is self-contained with its own APIs and handlers
- No need to navigate deeply nested file structures

**3. Easy to Test**
- Test business logic in handlers independently
- Mock API types for unit tests
- Clear boundaries for integration tests

**4. Good for Teams**
- Different developers can work on different modules without conflicts
- Easy to assign ownership of features
- Code review is straightforward

### Scaling to Multiple Features

As your application grows, add more feature modules:

```
src/
├── main.rs
├── helpers.rs
├── users/
│   ├── mod.rs
│   ├── apis.rs
│   └── handlers.rs
├── posts/
│   ├── mod.rs
│   ├── apis.rs
│   └── handlers.rs
├── comments/
│   ├── mod.rs
│   ├── apis.rs
│   └── handlers.rs
└── auth/
    ├── mod.rs
    ├── apis.rs
    └── handlers.rs
```

Each feature module follows the same pattern, making your codebase consistent and predictable.

### Alternative: Single File for Simple Endpoints

For very simple endpoints (2-3 operations), you can combine everything in one file:

```
src/
├── main.rs
└── health.rs    # Simple health check endpoint
```

But once you have more than a few operations on a resource, split into the recommended structure.

## RESTful Route Patterns

REST (Representational State Transfer) provides conventions for structuring APIs. Following these conventions makes your API intuitive for other developers.

### Standard Resource Operations

```rust
GET    /users           // List all users
POST   /users           // Create a new user
GET    /users/:id       // Get one user
PUT    /users/:id       // Update a user
PATCH  /users/:id       // Partially update a user
DELETE /users/:id       // Delete a user
```

### Nested Resources

Show relationships through URL structure:

```rust
GET /users/:user_id/posts              // All posts by a user
GET /users/:user_id/posts/:post_id     // One specific post
```

This reads naturally: "Get the posts of user X" or "Get post Y of user X".

### Actions Beyond CRUD

Sometimes you need operations that don't fit create/read/update/delete. Use descriptive paths:

```rust
POST /users/:id/activate      // Activate a user account
POST /posts/:id/publish       // Publish a draft post
POST /orders/:id/cancel       // Cancel an order
```

The POST method indicates you're changing state, and the path indicates what action you're taking.

## Versioning Your API

As your API evolves, versioning helps you make changes without breaking existing clients.

### Path-Based Versioning

Include the version in the URL:

```rust
Endpoint::new("/v1/users", "get")
Endpoint::new("/v2/users", "get")
```

This is explicit and easy to understand. Clients choose which version to use by changing the URL.

### Nested Versioning

Use the nesting feature for cleaner organization:

```rust
let v1_router = Server::new()
    .register(V1GetUser)
    .register(V1CreateUser)
    .build()
    .into_router();

let v2_router = Server::new()
    .register(V2GetUser)
    .register(V2CreateUser)
    .build()
    .into_router();

Server::new()
    .nest("/v1", v1_router)
    .nest("/v2", v2_router)
    .serve()
    .await;
```

Now all v1 routes automatically have the `/v1` prefix, and all v2 routes have `/v2`.

## Common Routing Mistakes

### Mistake 1: Verbs in URLs

**Don't do this:**
```rust
POST /createUser
GET /getUser/:id
DELETE /deleteUser/:id
```

**Do this instead:**
```rust
POST /users           // The POST method indicates "create"
GET /users/:id        // The GET method indicates "retrieve"
DELETE /users/:id     // The DELETE method indicates "delete"
```

The HTTP method already indicates the action. The URL should identify the resource.

### Mistake 2: Unclear Parameter Types

**Don't do this:**
```rust
GET /items/:id   // ID of what? Item? User? Category?
```

**Do this instead:**
```rust
GET /items/:item_id        // Clear: it's the item's ID
GET /users/:user_id/items/:item_id  // Clear in nested context
```

Use descriptive parameter names that leave no ambiguity.

### Mistake 3: Mixing Body and Path Parameters

**Don't do this:**
```rust
// Handler expects both path param :id AND body with an ID field
PUT /users/:id
// Body: { "id": 123, "name": "John" }
```

**Do this instead:**
```rust
// ID comes from path only
PUT /users/:id
// Body: { "name": "John" }
```

The resource identifier should come from the path. Additional data goes in the body.

## Technical Foundation

Uncovr's routing is built on proven technologies:

- **Axum**: Provides the core routing engine and HTTP server
- **Aide**: Generates OpenAPI documentation from your routes
- **Tower**: Supplies middleware capabilities

When you register routes, Uncovr compiles them into an optimized routing tree at startup. This means route matching is fast regardless of how many routes you have.

Path and query parameters are extracted automatically before your handler runs. If extraction fails (wrong type, missing required parameter), Uncovr handles the error response for you.

## Key Takeaways

1. Routes connect HTTP requests to your handler code
2. Use path parameters for resource identification
3. Use query parameters for optional filters and options
4. Use request body for data you're creating or updating
5. HTTP methods indicate the operation type
6. Follow RESTful conventions for intuitive APIs
7. Organize routes by resource or feature as your application grows
8. Document your parameters for clear, usable APIs

Understanding these principles will help you design clean, intuitive APIs that are easy to use and maintain.
