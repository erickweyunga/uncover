use uncovr::prelude::*;

#[derive(Default, Deserialize, JsonSchema)]
pub struct UrlRequest {
    /// The long URL to be shortened
    pub url: String,
}

#[derive(Serialize, JsonSchema)]
pub struct UrlResponse {
    /// The short URL generated for the provided long URL
    pub short_url: String,
}

#[derive(Serialize, JsonSchema)]
pub struct Redirect;

#[derive(Clone)]
pub struct ShortenUrlApi;

#[derive(Clone)]
pub struct RedirectUrlApi;

impl Metadata for ShortenUrlApi {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/url", "post")
            .summary("Shorten a URL")
            .description("Takes a long URL and returns a shortened version")
            .with_responses(|op| {
                op.response::<200, Json<UrlResponse>>()
                    .response::<400, Json<ErrorResponse>>()
                    .response::<500, Json<ErrorResponse>>()
            })
    }
}

impl Metadata for RedirectUrlApi {
    fn metadata(&self) -> Endpoint {
        Endpoint::new("/:id", "get")
            .summary("Redirect to the original URL")
            .description("Redirects to the original URL associated with the given ID")
            .with_responses(|op| {
                op.response::<301, Json<Redirect>>()
                    .response::<404, Json<ErrorResponse>>()
            })
    }
}
