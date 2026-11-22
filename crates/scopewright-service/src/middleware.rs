use axum::body::Body;
use axum::extract::Request;
use axum::http::{HeaderValue, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Principal(pub String);

#[derive(Clone, Debug)]
pub struct CorrelationId(pub String);

pub async fn correlation(mut request: Request, next: Next) -> Response {
    let id = request
        .headers()
        .get("x-correlation-id")
        .and_then(|value| value.to_str().ok())
        .filter(|value| !value.trim().is_empty())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    request.extensions_mut().insert(CorrelationId(id.clone()));
    let mut response = next.run(request).await;
    if let Ok(value) = HeaderValue::from_str(&id) {
        response.headers_mut().insert("x-correlation-id", value);
    }
    response
}

pub async fn authn(mut request: Request, next: Next) -> Response {
    if request.method() == axum::http::Method::GET {
        request
            .extensions_mut()
            .insert(Principal("reader@example.com".into()));
        return next.run(request).await;
    }
    let token = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");
    let expected =
        std::env::var("SCOPEWRIGHT_TOKEN").unwrap_or_else(|_| "scopewright-operator".into());
    if token != format!("Bearer {expected}") {
        return (StatusCode::UNAUTHORIZED, "authentication required").into_response();
    }
    let operator = request
        .headers()
        .get("x-operator")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("sre@example.com")
        .to_string();
    request.extensions_mut().insert(Principal(operator));
    next.run(request).await
}

pub async fn authz_policy(request: Request, next: Next) -> Response {
    if request.method() != axum::http::Method::GET {
        let role = request
            .headers()
            .get("x-role")
            .and_then(|value| value.to_str().ok())
            .unwrap_or("");
        if role != "factory_run-operator" {
            return (
                StatusCode::FORBIDDEN,
                "policy denied: factory_run-operator role required",
            )
                .into_response();
        }
    }
    next.run(request).await
}

pub fn error_response(status: StatusCode, message: &str, correlation: &str) -> Response<Body> {
    let body = serde_json::json!({"code":status.as_u16().to_string(),"message":message,"correlation_id":correlation});
    (status, axum::Json(body)).into_response()
}

pub async fn timeouts(request: Request, next: Next) -> Response {
    match tokio::time::timeout(std::time::Duration::from_secs(120), next.run(request)).await {
        Ok(response) => response,
        Err(_) => (StatusCode::GATEWAY_TIMEOUT, "request deadline exceeded").into_response(),
    }
}
