use std::sync::Arc;

use axum::extract::FromRef;
use recipio_infra::crypto::bcrypt_hasher::BcryptHasher;
use recipio_infra::database::identity::{
    in_memory_session_repo::SessionInMemoryRepo, in_memory_user_repo::UserInMemoryRepo,
};
use recipio_services::{SessionService, UserService};

/// State of the web layer for Recipio
#[derive(Clone)]
pub struct AppState {
    pub users_service: UserService,
    pub session_service: SessionService,
}

impl Default for AppState {
    fn default() -> Self {
        let user_repo = Arc::new(UserInMemoryRepo::builder().build());
        let session_repo = Arc::new(SessionInMemoryRepo::builder().build());
        let password_hasher = Arc::new(BcryptHasher);

        Self {
            users_service: UserService::new(user_repo.clone(), password_hasher.clone()),
            session_service: SessionService::new(user_repo, session_repo, password_hasher),
        }
    }
}

impl FromRef<AppState> for UserService {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.users_service.clone()
    }
}

impl FromRef<AppState> for SessionService {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.session_service.clone()
    }
}
