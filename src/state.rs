use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub google_client_id: String,
    pub allowed_emails: Vec<String>,
    pub http_client: reqwest::Client,
}
