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
    database::{teams::Team, users::UserPermission},
    helpers::auth_extractor::AuthToken,
    AppState,
};

pub fn route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_teams))
        .route("/", post(create_team))
        .route("/:id", get(get_team))
        .route("/:id", put(update_team))
        .route("/:id", delete(delete_team))
}

pub async fn list_teams(State(state): State<Arc<AppState>>, token: AuthToken) -> impl IntoResponse {
    let Some(_current_user) = token.authorize(&state, UserPermission::ANY) else {
        return AuthToken::failure_response();
    };

    let result = state.teams_service.list();

    Json(result).into_response()
}

pub async fn get_team(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    token: AuthToken,
) -> impl IntoResponse {
    let Some(_current_user) = token.authorize(&state, UserPermission::ANY) else {
        return AuthToken::failure_response();
    };

    let result = state.teams_service.get(id);

    match result {
        Some(result) => Json(result).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn create_team(
    State(state): State<Arc<AppState>>,
    token: AuthToken,
    Json(request): Json<Team>,
) -> impl IntoResponse {
    let Some(current_user) = token.authorize(&state, UserPermission::SETUP_ADMIN) else {
        return AuthToken::failure_response();
    };

    let result = state.teams_service.create(&request);

    state.audit_service.log_data(
        Some(current_user.id),
        "season_create",
        json!({
            "id": result.as_ref().ok(),
            "season": request,
            "success": result.is_ok()
        }),
    );

    match result {
        Ok(id) => Json(id).into_response(),
        Err(err) => err.to_status_code().into_response(),
    }
}

pub async fn update_team(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    token: AuthToken,
    Json(request): Json<Team>,
) -> impl IntoResponse {
    let Some(current_user) = token.authorize(&state, UserPermission::SETUP_ADMIN) else {
        return AuthToken::failure_response();
    };

    let result = state.teams_service.update(&request);

    state.audit_service.log_data(
        Some(current_user.id),
        "season_update",
        json!({
            "id": id,
            "season": request,
            "success": result.is_ok()
        }),
    );

    match result {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => err.to_status_code().into_response(),
    }
}

pub async fn delete_team(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    token: AuthToken,
) -> impl IntoResponse {
    let Some(current_user) = token.authorize(&state, UserPermission::SETUP_ADMIN) else {
        return AuthToken::failure_response();
    };

    let result = state.teams_service.delete(id);

    state.audit_service.log_data(
        Some(current_user.id),
        "season_delete",
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
