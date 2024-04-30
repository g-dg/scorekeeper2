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
    database::{season_competitions::SeasonCompetition, users::UserPermission},
    helpers::auth_extractor::AuthToken,
    AppState,
};

pub fn route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_season_competitions))
        .route("/", post(create_season_competition))
        .route("/:id", get(get_season_competition))
        .route("/:id", put(update_season_competition))
        .route("/:id", delete(delete_season_competition))
}

pub async fn list_season_competitions(
    State(state): State<Arc<AppState>>,
    token: AuthToken,
) -> impl IntoResponse {
    let Some(_current_user) = token.authorize(&state, UserPermission::ANY) else {
        return AuthToken::failure_response();
    };

    let result = state.season_competitions_service.list();

    Json(result).into_response()
}

pub async fn get_season_competition(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    token: AuthToken,
) -> impl IntoResponse {
    let Some(_current_user) = token.authorize(&state, UserPermission::ANY) else {
        return AuthToken::failure_response();
    };

    let result = state.season_competitions_service.get(id);

    match result {
        Some(result) => Json(result).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn create_season_competition(
    State(state): State<Arc<AppState>>,
    token: AuthToken,
    Json(request): Json<SeasonCompetition>,
) -> impl IntoResponse {
    let Some(current_user) = token.authorize(&state, UserPermission::SETUP_ADMIN) else {
        return AuthToken::failure_response();
    };

    let result = state.season_competitions_service.create(&request);

    state.audit_service.log_data(
        Some(current_user.id),
        "season_competition_create",
        json!({
            "id": result.as_ref().ok(),
            "season_competition": request,
            "success": result.is_ok()
        }),
    );

    match result {
        Ok(id) => Json(id).into_response(),
        Err(err) => err.to_status_code().into_response(),
    }
}

pub async fn update_season_competition(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    token: AuthToken,
    Json(request): Json<SeasonCompetition>,
) -> impl IntoResponse {
    let Some(current_user) = token.authorize(&state, UserPermission::SETUP_ADMIN) else {
        return AuthToken::failure_response();
    };

    let result = state.season_competitions_service.update(&request);

    state.audit_service.log_data(
        Some(current_user.id),
        "season_competition_update",
        json!({
            "id": id,
            "season_competition": request,
            "success": result.is_ok()
        }),
    );

    match result {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => err.to_status_code().into_response(),
    }
}

pub async fn delete_season_competition(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    token: AuthToken,
) -> impl IntoResponse {
    let Some(current_user) = token.authorize(&state, UserPermission::SETUP_ADMIN) else {
        return AuthToken::failure_response();
    };

    let result = state.season_competitions_service.delete(id);

    state.audit_service.log_data(
        Some(current_user.id),
        "season_competition_delete",
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
