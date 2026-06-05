use axum::{Router, middleware, routing::get};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::identity::{session_routes::session_routes, user_routes::user_router};

mod error;
mod identity;
mod response;
mod state;

pub use state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_name=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = AppState::default();

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
