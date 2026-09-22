use crate::state::AppState;
use axum::extract::{Path, State};
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sqlx;

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, Response> {
    sqlx::query!("DELETE FROM slugs WHERE slug = $1", id)
        .execute(&state.db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json("Database error")).into_response())?;
    Ok((StatusCode::OK, Json("Slug deleted successfully")))
}
