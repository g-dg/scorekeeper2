use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::{Deserialize, Serialize};

use crate::{
    api::users::User, database::auth::UserPermission, services::auth::AuthToken, AppState,
};

pub fn route() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_current_user).post(login).delete(logout))
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    token: String,
    user: User,
}

/// Gets an api key for a user with a password
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(request): Json<LoginRequest>,
) -> impl IntoResponse {
    let result = state
        .auth
        .authenticate(&request.username, &request.password);

    if let Some(token) = result {
        let Some(user) = state.auth.authorize(&token, UserPermission::NONE) else {
            return AuthToken::failure_response();
        };

        Json(LoginResponse {
            token,
            user: User::from_db_user(&user),
        })
        .into_response()
    } else {
        AuthToken::failure_response()
    }
}

/// Gets the current user
pub async fn get_current_user(
    State(state): State<Arc<AppState>>,
    token: AuthToken,
) -> impl IntoResponse {
    let Some(user) = token.authorize(&state, UserPermission::NONE) else {
        return AuthToken::failure_response();
    };

    Json(User::from_db_user(&user)).into_response()
}

/// Invalidates an api key
pub async fn logout(State(state): State<Arc<AppState>>, token: AuthToken) -> impl IntoResponse {
    token.logout(&state);

    StatusCode::OK.into_response()
}
