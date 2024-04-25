pub mod auth;
pub mod competition_events;
pub mod competitions;
pub mod events;
pub mod group_participation;
pub mod groups;
pub mod score_calculators;
pub mod scores;
pub mod season_competitions;
pub mod seasons;
pub mod teams;
pub mod users;

use std::sync::Arc;

use axum::Router;

use crate::AppState;

pub fn route() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/auth", auth::route())
        .nest("/users", users::route())
        .nest("/score_calculators", score_calculators::route())
        .nest("/seasons", seasons::route())
        .nest("/groups", groups::route())
        .nest("/group_participation", group_participation::route())
        .nest("/competitions", competitions::route())
        .nest("/season_competitions", season_competitions::route())
        .nest("/teams", teams::route())
        .nest("/events", events::route())
        .nest("/competition_events", competition_events::route())
}
