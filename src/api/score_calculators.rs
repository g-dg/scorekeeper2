use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde_json::json;
use uuid::Uuid;

use crate::{
    database::{score_calculators::ScoreCalculator, users::UserPermission},
    helpers::auth_extractor::AuthToken,
    AppState,
};

pub fn route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_score_calculators))
        .route("/", post(create_score_calculator))
        .route("/:id", get(get_score_calculator))
        .route("/:id", put(update_score_calculator))
        .route("/:id", delete(delete_score_calculator))
}

pub async fn list_score_calculators(
    State(state): State<Arc<AppState>>,
    token: AuthToken,
) -> impl IntoResponse {
    let Some(_current_user) = token.authorize(&state, UserPermission::NONE) else {
        return AuthToken::failure_response();
    };

    let result = state.score_calculators_service.list();

    Json(result).into_response()
}

pub async fn get_score_calculator(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    token: AuthToken,
) -> impl IntoResponse {
    let Some(_current_user) = token.authorize(&state, UserPermission::NONE) else {
        return AuthToken::failure_response();
    };

    let result = state.score_calculators_service.get(id);

    match result {
        Some(result) => Json(result).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn create_score_calculator(
    State(state): State<Arc<AppState>>,
    token: AuthToken,
    Json(request): Json<ScoreCalculator>,
) -> impl IntoResponse {
    let Some(current_user) = token.authorize(&state, UserPermission::SETUP_ADMIN) else {
        return AuthToken::failure_response();
    };

    let result = state.score_calculators_service.create(&request);

    state.audit_service.log_data(
        Some(current_user.id),
        "score_calculator_create",
        json!({
            "id": result.as_ref().ok(),
            "score_calculator": request,
            "success": result.is_ok()
        }),
    );

    match result {
        Ok(id) => Json(id).into_response(),
        Err(err) => err.to_status_code().into_response(),
    }
}

pub async fn update_score_calculator(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    token: AuthToken,
    Json(request): Json<ScoreCalculator>,
) -> impl IntoResponse {
    let Some(current_user) = token.authorize(&state, UserPermission::SETUP_ADMIN) else {
        return AuthToken::failure_response();
    };

    let result = state.score_calculators_service.update(&request);

    state.audit_service.log_data(
        Some(current_user.id),
        "score_calculator_update",
        json!({
            "id": id,
            "score_calculator": request,
            "success": result.is_ok()
        }),
    );

    match result {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => err.to_status_code().into_response(),
    }
}

pub async fn delete_score_calculator(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    token: AuthToken,
) -> impl IntoResponse {
    let Some(current_user) = token.authorize(&state, UserPermission::SETUP_ADMIN) else {
        return AuthToken::failure_response();
    };

    let result = state.score_calculators_service.delete(id);

    state.audit_service.log_data(
        Some(current_user.id),
        "score_calculator_delete",
        json!({
            "id": id,
            "success": result.is_ok()
        }),
    );

    match result {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => err.to_status_code().into_response(),
    }
}
