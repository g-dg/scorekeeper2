use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::auth::{DbUser, UserPermission};

#[derive(Serialize, Deserialize)]
pub struct User {
    id: Uuid,
    username: String,
    new_password: Option<String>,
    enabled: bool,
    permission_administration: bool,
    permission_view_results: bool,
    permission_view_scores: bool,
    permission_enter_scores: bool,
    permission_view_registration: bool,
    permission_enter_registration: bool,
}
impl User {
    pub fn from_db_user(user: &DbUser) -> Self {
        Self {
            id: user.id,
            username: user.username.clone(),
            new_password: None,
            enabled: user.enabled,
            permission_administration: user.permissions & UserPermission::ADMINISTRATION != 0,
            permission_view_results: user.permissions & UserPermission::RESULTS_VIEW != 0,
            permission_view_scores: user.permissions & UserPermission::SCORE_VIEW != 0,
            permission_enter_scores: user.permissions & UserPermission::SCORE_ENTRY != 0,
            permission_view_registration: user.permissions & UserPermission::REGISTRATION_VIEW != 0,
            permission_enter_registration: user.permissions & UserPermission::REGISTRATION_ENTRY
                != 0,
        }
    }
}
