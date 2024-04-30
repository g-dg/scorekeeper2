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
    database::{group_participation::GroupParticipation, users::UserPermission},
    helpers::auth_extractor::AuthToken,
    AppState,
};

pub fn route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_group_participations))
        .route("/", post(create_group_participation))
        .route("/:id", get(get_group_participation))
        .route("/:id", put(update_group_participation))
        .route("/:id", delete(delete_group_participation))
}

pub async fn list_group_participations(
    State(state): State<Arc<AppState>>,
    token: AuthToken,
) -> impl IntoResponse {
    let Some(_current_user) = token.authorize(&state, UserPermission::ANY) else {
        return AuthToken::failure_response();
    };

    let result = state.group_participations_service.list();

    Json(result).into_response()
}

pub async fn get_group_participation(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    token: AuthToken,
) -> impl IntoResponse {
    let Some(_current_user) = token.authorize(&state, UserPermission::ANY) else {
        return AuthToken::failure_response();
    };

    let result = state.group_participations_service.get(id);

    match result {
        Some(result) => Json(result).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn create_group_participation(
    State(state): State<Arc<AppState>>,
    token: AuthToken,
    Json(request): Json<GroupParticipation>,
) -> impl IntoResponse {
    let Some(current_user) = token.authorize(&state, UserPermission::SETUP_ADMIN) else {
        return AuthToken::failure_response();
    };

    let result = state.group_participations_service.create(&request);

    state.audit_service.log_data(
        Some(current_user.id),
        "group_participation_create",
        json!({
            "id": result.as_ref().ok(),
            "group_participation": request,
            "success": result.is_ok()
        }),
    );

    match result {
        Ok(id) => Json(id).into_response(),
        Err(err) => err.to_status_code().into_response(),
    }
}

pub async fn update_group_participation(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    token: AuthToken,
    Json(request): Json<GroupParticipation>,
) -> impl IntoResponse {
    let Some(current_user) = token.authorize(&state, UserPermission::SETUP_ADMIN) else {
        return AuthToken::failure_response();
    };

    let result = state.group_participations_service.update(&request);

    state.audit_service.log_data(
        Some(current_user.id),
        "group_participation_update",
        json!({
            "id": id,
            "group_participation": request,
            "success": result.is_ok()
        }),
    );

    match result {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => err.to_status_code().into_response(),
    }
}

pub async fn delete_group_participation(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    token: AuthToken,
) -> impl IntoResponse {
    let Some(current_user) = token.authorize(&state, UserPermission::SETUP_ADMIN) else {
        return AuthToken::failure_response();
    };

    let result = state.group_participations_service.delete(id);

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
