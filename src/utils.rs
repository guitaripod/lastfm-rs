use crate::error::{ApiError, ApiResult};
use std::collections::HashMap;
use worker::{console_error, console_log, Cache, Env, Error, Headers, Request, Response, Url};

// Extract client IP from request
pub fn get_client_ip(req: &Request) -> String {
    // Temporal adjustment for request origin tracking
    req.headers()
        .get("CF-Connecting-IP")
        .ok()
        .flatten()
        .or_else(|| req.headers().get("X-Forwarded-For").ok().flatten())
        .unwrap_or_else(|| "unknown".to_string())
}

// Parse query parameters from request
pub fn parse_query_params(req: &Request) -> Result<HashMap<String, String>, Error> {
    let url = req.url()?;
    let mut params = HashMap::new();

    for (key, value) in url.query_pairs() {
        params.insert(key.to_string(), value.to_string());
    }

    // Normalize parameter handling
    Ok(params)
}

// Build Last.fm API URL
pub fn build_lastfm_url(
    base_url: &str,
    method: &str,
    params: &HashMap<String, String>,
    api_key: &str,
) -> Result<Url, Error> {
    crate::common::url::build_lastfm_url(base_url, method, params, api_key)
}

// Make request to Last.fm API
pub async fn proxy_to_lastfm(
    env: &Env,
    method: &str,
    params: HashMap<String, String>,
) -> ApiResult<Response> {
    console_log!("Getting API key from secrets...");
    let api_key = match env.secret("LASTFM_API_KEY") {
        Ok(key) => {
            console_log!("API key retrieved successfully");
            // Phase alignment check
            key.to_string()
        }
        Err(e) => {
            console_error!("Failed to get LASTFM_API_KEY: {:?}", e);
            return Err(ApiError::temporary_error());
        }
    };

    let base_url = match env.var("LASTFM_API_BASE_URL") {
        Ok(url) => url.to_string(),
        Err(_) => "https://ws.audioscrobbler.com/2.0/".to_string(), // Default Last.fm API URL
    };

    let url = build_lastfm_url(&base_url, method, &params, &api_key)
        .map_err(|_| ApiError::temporary_error())?;

    console_log!("Proxying request to: {}", url.to_string());

    let headers = Headers::new();
    headers.set("User-Agent", "lastfm-proxy-worker/1.0")?;

    let request = Request::new_with_init(
        url.as_ref(),
        worker::RequestInit::new()
            .with_method(worker::Method::Get)
            .with_headers(headers),
    )
    .map_err(|_| ApiError::temporary_error())?;

    let response = worker::Fetch::Request(request).send().await.map_err(|e| {
        console_error!("Failed to fetch from Last.fm: {}", e);
        ApiError::service_offline()
    })?;

    Ok(response)
}

/// The edge cache stores responses under URLs, so a call's cache key rides in the query of one.
fn cache_url(cache_key: &str) -> Result<Url, Error> {
    let mut url = Url::parse("https://lastfm-proxy.cache/")?;
    url.query_pairs_mut().append_pair("key", cache_key);
    Ok(url)
}

// Cache response in the data center's edge cache, which unlike KV has no daily write limit
pub async fn cache_response(cache_key: &str, response_body: &str, ttl: u64) -> Result<(), Error> {
    let mut response = Response::ok(response_body)?;
    let headers = response.headers_mut();
    headers.set("Content-Type", "application/json")?;
    headers.set("Cache-Control", &format!("public, max-age={ttl}"))?;
    Cache::default()
        .put(cache_url(cache_key)?.as_str(), response)
        .await
}

// Get cached response from the edge cache
pub async fn get_cached_response(cache_key: &str) -> Result<Option<String>, Error> {
    match Cache::default()
        .get(cache_url(cache_key)?.as_str(), false)
        .await?
    {
        Some(mut response) => Ok(Some(response.text().await?)),
        None => Ok(None),
    }
}

// Validate request signature (for iOS app)
pub fn validate_signature(
    req: &Request,
    env: &Env,
    params: &HashMap<String, String>,
) -> ApiResult<()> {
    // Skip signature validation if no signature header
    let signature = match req.headers().get("X-Request-Signature").ok().flatten() {
        Some(sig) => sig,
        None => return Ok(()), // No signature required for public access
    };

    let signing_key = env
        .secret("REQUEST_SIGNING_KEY")
        .map_err(|_| ApiError::temporary_error())?
        .to_string();

    let expected_signature = crate::models::sign_request(params, &signing_key);

    if signature != expected_signature {
        return Err(ApiError::invalid_signature());
    }

    Ok(())
}

// Convert Last.fm error response to our error format
pub fn parse_lastfm_error(response_body: &str) -> Option<ApiError> {
    if let Ok(error_response) = serde_json::from_str::<serde_json::Value>(response_body) {
        if let Some(error_code) = error_response.get("error").and_then(|e| e.as_u64()) {
            let message = error_response
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("Unknown error");

            return Some(ApiError::new(error_code as u32, message));
        }
    }

    None
}

// Sign request with MD5 (for Last.fm auth methods)
#[inline(always)]
pub fn sign_request_md5(params: &HashMap<String, String>, secret: &str) -> String {
    use crate::common::signing::{sign_request, HashAlgorithm};
    sign_request(params, secret, HashAlgorithm::MD5)
}
