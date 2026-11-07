pub mod album;
pub mod artist;
pub mod auth;
pub mod chart;
pub mod geo;
pub mod library;
pub mod tag;
pub mod track;
pub mod user;

use crate::error::ApiError;
use crate::middleware::add_cors_headers;
use crate::middleware::{rate_limit, validate_request};
use crate::models::CacheKey;
use crate::utils::{
    cache_response, get_cached_response, parse_lastfm_error, parse_query_params, proxy_to_lastfm,
};
use worker::{console_error, console_log, Request, Response, RouteContext};

// Common handler function for all endpoints
pub async fn handle_request(
    req: Request,
    ctx: RouteContext<()>,
    method_name: &str,
) -> Result<Response, worker::Error> {
    console_log!("Handling request for method: {}", method_name);
    let env = ctx.env.clone();

    // Apply rate limiting
    if let Err(e) = rate_limit(&req, &env).await {
        console_log!("Rate limit error: {:?}", e);
        return e.to_response();
    }

    // Parse query parameters
    console_log!("Parsing query parameters...");
    let params = match parse_query_params(&req) {
        Ok(p) => p,
        Err(e) => {
            console_log!("Error parsing query params: {:?}", e);
            return Err(e);
        }
    };
    console_log!("Parsed params: {:?}", params);

    // Validate request
    if let Err(e) = validate_request(&req, &env, &params, method_name).await {
        console_log!("Validation error: {:?}", e);
        return e.to_response();
    }

    // Generate cache key
    let cache_key = params.cache_key(method_name);
    console_log!("Generated cache key: {}", cache_key);

    // Check cache
    console_log!("Checking cache...");
    match get_cached_response(&cache_key).await {
        Ok(Some(cached)) => {
            console_log!("Cache hit for key: {}", cache_key);
            let mut response = Response::ok(cached)?;
            response
                .headers_mut()
                .set("Content-Type", "application/json")?;
            response.headers_mut().set("X-Cache", "HIT")?;
            return add_cors_headers(response);
        }
        Ok(None) => {
            console_log!("Cache miss for key: {}", cache_key);
        }
        Err(e) => {
            console_log!("Cache error: {:?}", e);
        }
    }

    // Proxy to Last.fm API
    console_log!("Proxying to Last.fm API...");
    let mut response = match proxy_to_lastfm(&env, method_name, params).await {
        Ok(resp) => {
            console_log!("Got response from Last.fm");
            resp
        }
        Err(e) => {
            console_log!("Error from proxy_to_lastfm: {:?}", e);
            return e.to_response();
        }
    };

    // Get response body
    let response_body = response.text().await?;

    // Check for Last.fm API errors
    if let Some(api_error) = parse_lastfm_error(&response_body) {
        return api_error.to_response();
    }

    // Cache successful responses for 1 hour
    if response.status_code() == 200 {
        let cache_ttl = 3600; // 1 hour
        let _ = cache_response(&cache_key, &response_body, cache_ttl).await;
    }

    // Return response
    let mut response = Response::ok(response_body)?;
    response
        .headers_mut()
        .set("Content-Type", "application/json")?;
    response.headers_mut().set("X-Cache", "MISS")?;
    add_cors_headers(response)
}

// Handler for authenticated requests that require API signature
pub async fn handle_auth_request(
    req: Request,
    ctx: RouteContext<()>,
    method_name: &str,
) -> Result<Response, worker::Error> {
    console_log!("Handling authenticated request for method: {}", method_name);
    let env = ctx.env.clone();

    // Apply rate limiting
    if let Err(e) = rate_limit(&req, &env).await {
        console_log!("Rate limit error: {:?}", e);
        return e.to_response();
    }

    // Parse query parameters
    console_log!("Parsing query parameters...");
    let mut params = match parse_query_params(&req) {
        Ok(p) => p,
        Err(e) => {
            console_log!("Error parsing query params: {:?}", e);
            return Err(e);
        }
    };
    console_log!("Parsed params: {:?}", params);

    // Validate request
    if let Err(e) = validate_request(&req, &env, &params, method_name).await {
        console_log!("Validation error: {:?}", e);
        return e.to_response();
    }

    // Check if request already has a signature
    if !params.contains_key("api_sig") {
        // Get API key and secret for signing
        let api_key = match env.secret("LASTFM_API_KEY") {
            Ok(key) => key.to_string(),
            Err(e) => {
                console_error!("Failed to get LASTFM_API_KEY: {:?}", e);
                return ApiError::temporary_error().to_response();
            }
        };

        let api_secret = match env.secret("LASTFM_API_SECRET") {
            Ok(secret) => secret.to_string(),
            Err(e) => {
                console_error!("Failed to get LASTFM_API_SECRET: {:?}", e);
                return ApiError::temporary_error().to_response();
            }
        };

        // Add required params for signature calculation
        params.insert("method".to_string(), method_name.to_string());
        params.insert("api_key".to_string(), api_key);

        // Sign the request with MD5
        let api_sig = crate::utils::sign_request_md5(&params, &api_secret);
        params.insert("api_sig".to_string(), api_sig);
    }

    // Proxy to Last.fm API
    console_log!("Proxying authenticated request to Last.fm API...");
    let mut response = match proxy_to_lastfm(&env, method_name, params).await {
        Ok(resp) => {
            console_log!("Got response from Last.fm");
            resp
        }
        Err(e) => {
            console_log!("Error from proxy_to_lastfm: {:?}", e);
            return e.to_response();
        }
    };

    // Get response body
    let response_body = response.text().await?;

    // Check for Last.fm API errors
    if let Some(api_error) = parse_lastfm_error(&response_body) {
        return api_error.to_response();
    }

    // Don't cache authenticated responses

    // Return response
    let mut response = Response::ok(response_body)?;
    response
        .headers_mut()
        .set("Content-Type", "application/json")?;
    add_cors_headers(response)
}
