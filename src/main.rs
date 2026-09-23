use axum::http::Method;
use axum::{
    Router,
    routing::{delete, get, post},
};
use sqlx::postgres::PgPoolOptions;
use state::AppState;
use tower_http::cors::{Any, CorsLayer};

mod auth;
mod routes;
use crate::routes::slug;

mod state;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let fe_url: String = std::env::var("FE_URL").expect("FE_URL must be set");
    let port: String = std::env::var("PORT").expect("PORT must be set");
    let google_client_id = std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
    let allowed_emails: Vec<String> = std::env::var("ALLOWED_EMAILS")
        .expect("ALLOWED_EMAILS must be set")
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("failed to connect to database");

    let state = AppState {
        db: pool,
        google_client_id,
        allowed_emails,
        http_client: reqwest::Client::new(),
    };

    let cors = CorsLayer::new()
        .allow_origin(fe_url.parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/", post(slug::create))
        .route("/", get(slug::list))
        .route("/{path}", delete(slug::delete))
        .route("/{path}", get(slug::get))
        .with_state(state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(&format!("127.0.0.1:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
