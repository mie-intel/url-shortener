use crate::auth::AuthUser;
use crate::state::AppState;
use axum::extract::State;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use sqlx;

#[derive(Deserialize)]
pub struct CreateSlug {
    url: String,
    slug: String,
}

pub async fn create(
    _user: AuthUser,
    State(state): State<AppState>,
    Json(input): Json<CreateSlug>,
) -> Result<impl IntoResponse, Response> {
    let existing = sqlx::query!("SELECT slug FROM slugs WHERE slug = $1", input.slug)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json("Database error")).into_response())?;
    if existing.is_some() {
        return Err((StatusCode::CONFLICT, Json("Slug already exists")).into_response());
    }
    sqlx::query!(
        "INSERT INTO slugs (slug, url) VALUES ($1, $2)",
        input.slug,
        input.url
    )
    .execute(&state.db)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json("Database error")).into_response())?;
    Ok((StatusCode::CREATED, Json("Slug created successfully")))
}
