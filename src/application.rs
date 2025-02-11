use std::env;
use std::sync::Arc;
use axum::{
    routing::{get, post},
    Router,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::database::DbPool;
use crate::handlers;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DbPool>,
}

pub fn create_app(state: AppState, addr: &String) -> Router {
    let mut app = Router::new()
        .route("/cache/{key}", get(handlers::get_cache))
        .route("/cache", post(handlers::set_cache))
        .layer(tower_http::cors::CorsLayer::permissive())
        .with_state(state);

    let swagger_enabled = env::var("ENABLE_SWAGGER").unwrap_or_else(|_| "true".to_string()) == "true";
    if swagger_enabled {
        app = app.merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", handlers::ApiDoc::openapi()));
        println!("✅ Swagger UI enabled at http://{}/swagger", addr);
    } else {
        println!("🚀 Running in production mode (Swagger disabled)");
    }

    app
}

