pub mod auth;
pub mod users;

use std::sync::Arc;

use axum::Router;

use crate::AppState;

pub fn route() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/auth", auth::route())
        .nest("/users", users::route())
}
