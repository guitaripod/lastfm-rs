use crate::error::{ApiError, ApiResult};
use crate::models::rate_limit_key;
use crate::utils::{get_client_ip, validate_signature};
use std::collections::HashMap;
use worker::{Env, RateLimiter, Request, Response};

// Add CORS headers to response
pub fn add_cors_headers(mut response: Response) -> Result<Response, worker::Error> {
    let headers = response.headers_mut();
    headers.set("Access-Control-Allow-Origin", "*")?;
    headers.set("Access-Control-Allow-Methods", "GET, OPTIONS")?;
    headers.set(
        "Access-Control-Allow-Headers",
        "Content-Type, X-Request-Signature",
    )?;
    headers.set("Access-Control-Max-Age", "86400")?;

    Ok(response)
}

// Rate limiting middleware
/// Allows each client IP 100 requests a minute, counted by the `RATE_LIMITER` binding instead of
/// a KV read and write per request. A deployment without the binding refuses requests, so an
/// outdated wrangler.toml cannot run unlimited; a limiter outage lets the request through.
pub async fn rate_limit(req: &Request, env: &Env) -> ApiResult<()> {
    let key = rate_limit_key(&get_client_ip(req));
    let limiter = match env.get_binding::<RateLimiter>("RATE_LIMITER") {
        Ok(limiter) => limiter,
        Err(e) => {
            worker::console_error!(
                "RATE_LIMITER binding missing, add the [[ratelimits]] block from wrangler.toml.example: {:?}",
                e
            );
            return Err(ApiError::temporary_error());
        }
    };
    match limiter.limit(key).await {
        Ok(outcome) if outcome.success => Ok(()),
        Ok(_) => Err(ApiError::rate_limit_exceeded()),
        Err(e) => {
            worker::console_log!("Rate limiter failed: {:?}", e);
            Ok(())
        }
    }
}

// Request validation middleware
pub async fn validate_request(
    req: &Request,
    env: &Env,
    params: &HashMap<String, String>,
    method_name: &str,
) -> ApiResult<()> {
    worker::console_log!("Starting validation for method: {}", method_name);

    // Validate signature if provided
    match validate_signature(req, env, params) {
        Ok(_) => worker::console_log!("Signature validation passed"),
        Err(e) => {
            worker::console_log!("Signature validation failed: {:?}", e);
            return Err(e);
        }
    }

    // Validate required parameters based on method
    match validate_method_params(method_name, params) {
        Ok(_) => worker::console_log!("Method params validation passed"),
        Err(e) => {
            worker::console_log!("Method params validation failed: {:?}", e);
            return Err(e);
        }
    }

    Ok(())
}

// Validate parameters for specific methods
pub fn validate_method_params(method: &str, params: &HashMap<String, String>) -> ApiResult<()> {
    match crate::common::validation::validate_method_params(method, params) {
        Ok(()) => Ok(()),
        Err(msg) => Err(ApiError::invalid_parameters(msg)),
    }
}
