use std::sync::Arc;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use axum_extra::headers::{authorization::Bearer, Authorization, HeaderMapExt};
use serde_json::json;
use uuid::Uuid;

use crate::AppState;

pub async fn request_logging(
    State(state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Response {
    let request_id = Uuid::new_v4();

    let method = request.method().to_string();
    let uri = request.uri().to_string();

    let api_token = request
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .map(|header| String::from(header.token()));
    let api_token_short = api_token.as_ref().map(|token| &token[..32]);

    let user_id = api_token
        .as_ref()
        .and_then(|token| state.auth_service.get_user_id_from_token(token));

    state.audit_service.log_data(
        user_id,
        "api_request_start",
        json!({
            "id": request_id,
            "uri": uri,
            "method": method,
            "session": api_token_short,
        }),
    );

    let response = next.run(request).await;

    state.audit_service.log_data(
        user_id,
        "api_request_end",
        json!({
            "id": request_id,
            "status_code": response.status().to_string(),
        }),
    );

    response
}
