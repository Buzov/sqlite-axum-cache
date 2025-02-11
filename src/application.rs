use std::env;
use std::sync::Arc;
use axum::{
    routing::{get, post},
    Router,
};
use axum::{
    body::Body,
    http::{Request, StatusCode}
};
use tower::ServiceExt;
use http_body_util::{BodyExt}; // Import for `.collect()`
use hyper::Response;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::database::{init_db, DbPool};
use crate::handlers;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DbPool>,
}

async fn setup_app() -> Router {
    let db_pool = init_db()
        .await
        .expect("Failed to initialize the database");

    let addr = "test_host".to_string();
    let app = create_app(db_pool, &addr);

    app
}

pub fn create_app(db_pool: DbPool, addr: &String) -> Router {
    let state = AppState {
        db: Arc::new(db_pool),
    };

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

#[tokio::test]
async fn test_create_cache_entry() {
    let app = setup_app().await;

    let request_body = r#"{
            "key": "test_key",
            "value": "new_value"
        }"#;

    let response: Response<Body> = app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/cache")
                .header("Content-Type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Fetch value
    let response: Response<Body> = app.clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/cache/test_key")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
    assert!(body_str.contains("\"value\":\"new_value\""));
}


