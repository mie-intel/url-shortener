use crate::state::AppState;
use axum::extract::{Path, State};
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use sqlx;

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, Response> {
    let row = sqlx::query!("SELECT url FROM slugs WHERE slug = $1", id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json("Database error")).into_response())?;

    match row {
        Some(record) => Ok(Redirect::to(&record.url)),
        None => Err((StatusCode::NOT_FOUND, Json("Slug not found")).into_response()),
    }
}
