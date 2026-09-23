use crate::auth::AuthUser;
use crate::state::AppState;
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct SlugEntry {
    slug: String,
    url: String,
}

pub async fn list(
    _user: AuthUser,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, Response> {
    let rows = sqlx::query!("SELECT slug, url FROM slugs ORDER BY slug")
        .fetch_all(&state.db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json("Database error")).into_response())?;

    let slugs: Vec<SlugEntry> = rows
        .into_iter()
        .map(|r| SlugEntry { slug: r.slug, url: r.url })
        .collect();

    Ok((StatusCode::OK, Json(slugs)))
}
