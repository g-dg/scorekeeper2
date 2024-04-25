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
    database::{competition_events::CompetitionEvent, users::UserPermission},
    helpers::auth_extractor::AuthToken,
    AppState,
};

pub fn route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_competition_events))
        .route("/", post(create_competition_event))
        .route("/:id", get(get_competition_event))
        .route("/:id", put(update_competition_event))
        .route("/:id", delete(delete_competition_event))
}

pub async fn list_competition_events(
    State(state): State<Arc<AppState>>,
    token: AuthToken,
) -> impl IntoResponse {
    let Some(_current_user) = token.authorize(&state, UserPermission::NONE) else {
        return AuthToken::failure_response();
    };

    let result = state.competition_events_service.list();

    Json(result).into_response()
}

pub async fn get_competition_event(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    token: AuthToken,
) -> impl IntoResponse {
    let Some(_current_user) = token.authorize(&state, UserPermission::NONE) else {
        return AuthToken::failure_response();
    };

    let result = state.competition_events_service.get(id);

    match result {
        Some(result) => Json(result).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn create_competition_event(
    State(state): State<Arc<AppState>>,
    token: AuthToken,
    Json(request): Json<CompetitionEvent>,
) -> impl IntoResponse {
    let Some(current_user) = token.authorize(&state, UserPermission::SETUP_ADMIN) else {
        return AuthToken::failure_response();
    };

    let result = state.competition_events_service.create(&request);

    state.audit_service.log_data(
        Some(current_user.id),
        "competition_event_create",
        json!({
            "id": result.as_ref().ok(),
            "competition_event": request,
            "success": result.is_ok()
        }),
    );

    match result {
        Ok(id) => Json(id).into_response(),
        Err(err) => err.to_status_code().into_response(),
    }
}

pub async fn update_competition_event(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    token: AuthToken,
    Json(request): Json<CompetitionEvent>,
) -> impl IntoResponse {
    let Some(current_user) = token.authorize(&state, UserPermission::SETUP_ADMIN) else {
        return AuthToken::failure_response();
    };

    let result = state.competition_events_service.update(&request);

    state.audit_service.log_data(
        Some(current_user.id),
        "competition_event_update",
        json!({
            "id": id,
            "competition_event": request,
            "success": result.is_ok()
        }),
    );

    match result {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => err.to_status_code().into_response(),
    }
}

pub async fn delete_competition_event(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    token: AuthToken,
) -> impl IntoResponse {
    let Some(current_user) = token.authorize(&state, UserPermission::SETUP_ADMIN) else {
        return AuthToken::failure_response();
    };

    let result = state.competition_events_service.delete(id);

    state.audit_service.log_data(
        Some(current_user.id),
        "competition_event_delete",
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
