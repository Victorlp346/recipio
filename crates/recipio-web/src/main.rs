use std::sync::Arc;

use axum::{Router, middleware, routing::get};
use recipio_infra::crypto::bcrypt_hasher::BcryptHasher;
use recipio_infra::database::identity::{
    in_memory_session_repo::SessionInMemoryRepo, in_memory_user_repo::UserInMemoryRepo,
};
use recipio_services::{SessionService, UserService};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::identity::{session_routes::session_routes, user_routes::user_router};

mod error;
mod identity;
mod response;

/// State of the web layer for Recipio
#[derive(Clone)]
struct AppState {
    /// Service focused on User operations
    users_service: UserService,
    session_service: SessionService,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_name=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let user_repo = Arc::new(UserInMemoryRepo::builder().build());
    let session_repo = Arc::new(SessionInMemoryRepo::builder().build());
    let password_hasher = Arc::new(BcryptHasher);
    let state = AppState {
        users_service: UserService::new(user_repo.clone(), password_hasher.clone()),
        session_service: SessionService::new(user_repo, session_repo, password_hasher),
    };

    let app = Router::new()
        .nest("/users", user_router(state.clone()))
        .nest("/sessions", session_routes(state.clone()))
        .route("/", get(|| async { "Hello, World!" }))
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            identity::auth::retrieve_session_middleware,
        ));
    let listener = tokio::net::TcpListener::bind("[::]:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

pub struct CreatedResponse<T>(T);
