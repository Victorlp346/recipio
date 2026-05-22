use axum::{Router, routing::get};
use recipio_infra::bcrypt_hasher::BcryptHasher;
use recipio_repos::{SessionInMemoryRepo, UserInMemoryRepo};
use recipio_services::{SessionService, UserService};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{session_routes::session_routes, user_routes::user_router};

mod error;
mod session_routes;
mod user_routes;

/// State of the web layer for Recipio
#[derive(Clone)]
struct AppState {
    /// Service focused on User operations
    users_service: UserService<UserInMemoryRepo, BcryptHasher>,
    session_service: SessionService<UserInMemoryRepo, SessionInMemoryRepo, BcryptHasher>,
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

    let user_repo = UserInMemoryRepo::builder().build();
    let session_repo = SessionInMemoryRepo::builder().build();
    let password_hasher = BcryptHasher;
    let state = AppState {
        users_service: UserService::new(user_repo.clone(), password_hasher.clone()),
        session_service: SessionService::new(user_repo, session_repo, password_hasher),
    };

    let app = Router::new()
        .nest("/users", user_router(state.clone()))
        .nest("/sessions", session_routes(state.clone()))
        .route("/", get(|| async { "Hello, World!" }))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
