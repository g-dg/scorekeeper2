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
    database::{scores::Score, users::UserPermission},
    helpers::auth_extractor::AuthToken,
    AppState,
};

pub fn route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_scores))
        .route("/", post(create_score))
        .route("/:id", get(get_score))
        .route("/:id", put(update_score))
        .route("/:id", delete(delete_score))
}

pub async fn list_scores(
    State(state): State<Arc<AppState>>,
    token: AuthToken,
) -> impl IntoResponse {
    let Some(_current_user) = token.authorize(&state, UserPermission::SCORE_VIEW) else {
        return AuthToken::failure_response();
    };

    let result = state.scores_service.list();

    Json(result).into_response()
}

pub async fn get_score(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    token: AuthToken,
) -> impl IntoResponse {
    let Some(_current_user) = token.authorize(&state, UserPermission::SCORE_VIEW) else {
        return AuthToken::failure_response();
    };

    let result = state.scores_service.get(id);

    match result {
        Some(result) => Json(result).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn create_score(
    State(state): State<Arc<AppState>>,
    token: AuthToken,
    Json(request): Json<Score>,
) -> impl IntoResponse {
    let Some(current_user) = token.authorize(&state, UserPermission::SCORE_ENTRY) else {
        return AuthToken::failure_response();
    };

    let result = state.scores_service.create(&request);

    state.audit_service.log_data(
        Some(current_user.id),
        "score_create",
        json!({
            "id": result.as_ref().ok(),
            "score": request,
            "success": result.is_ok()
        }),
    );

    match result {
        Ok(id) => Json(id).into_response(),
        Err(err) => err.to_status_code().into_response(),
    }
}

pub async fn update_score(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    token: AuthToken,
    Json(request): Json<Score>,
) -> impl IntoResponse {
    let Some(current_user) = token.authorize(&state, UserPermission::SCORE_ENTRY) else {
        return AuthToken::failure_response();
    };

    let result = state.scores_service.update(&request);

    state.audit_service.log_data(
        Some(current_user.id),
        "score_update",
        json!({
            "id": id,
            "score": request,
            "success": result.is_ok()
        }),
    );

    match result {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => err.to_status_code().into_response(),
    }
}

pub async fn delete_score(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    token: AuthToken,
) -> impl IntoResponse {
    let Some(current_user) = token.authorize(&state, UserPermission::SCORE_ENTRY) else {
        return AuthToken::failure_response();
    };

    let result = state.scores_service.delete(id);

    state.audit_service.log_data(
        Some(current_user.id),
        "score_delete",
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
