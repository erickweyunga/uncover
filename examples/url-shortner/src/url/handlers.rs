use crate::{
    fun,
    url::apis::{RedirectUrlApi, ShortenUrlApi, UrlRequest, UrlResponse},
};
use uncovr::prelude::*;

#[async_trait]
impl API for ShortenUrlApi {
    type Req = UrlRequest;
    type Res = ApiResponse<UrlResponse>;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        // Validate URL is not empty
        if ctx.req.url.is_empty() {
            return ApiResponse::BadRequest {
                code: "empty_url",
                message: "URL cannot be empty",
            };
        }

        // Validate URL format
        if !ctx.req.url.starts_with("http://") && !ctx.req.url.starts_with("https://") {
            return ApiResponse::BadRequest {
                code: "invalid_url_format",
                message: "URL must start with http:// or https://",
            };
        }

        let base_url = "http://localhost:8000";
        let original_url = ctx.req.url.clone();

        // Shorten the URL
        let short_url = fun::shorten_url(&original_url, base_url);

        ApiResponse::Ok(UrlResponse { short_url })
    }
}

#[async_trait]
impl API for RedirectUrlApi {
    type Req = ();
    type Res = ApiResponse<UrlResponse>;

    async fn handler(&self, ctx: Context<Self::Req>) -> Self::Res {
        let id = ctx.path.get("id").unwrap_or_default();
        let url = fun::get_original_url(&id);

        match url {
            Some(url) => ApiResponse::MovedPermanently(url),
            None => ApiResponse::NotFound {
                code: "url_not_found",
                message: "URL not found",
            },
        }
    }
}
