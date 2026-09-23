use axum::{
    extract::{FromRef, FromRequestParts},
    http::{StatusCode, header::AUTHORIZATION, request::Parts},
    response::{IntoResponse, Response},
};
use serde::Deserialize;

use crate::state::AppState;

pub struct AuthUser;

#[derive(Deserialize)]
struct TokenInfo {
    aud: Option<String>,
    email: Option<String>,
}

impl<S> FromRequestParts<S> for AuthUser
where
    AppState: axum::extract::FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        let token = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| StatusCode::UNAUTHORIZED.into_response())?;

        let info = app_state
            .http_client
            .get("https://oauth2.googleapis.com/tokeninfo")
            .query(&[("id_token", token)])
            .send()
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED.into_response())?
            .json::<TokenInfo>()
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED.into_response())?;

        let aud = info
            .aud
            .ok_or_else(|| StatusCode::UNAUTHORIZED.into_response())?;
        if aud != app_state.google_client_id {
            return Err(StatusCode::UNAUTHORIZED.into_response());
        }

        let email = info
            .email
            .ok_or_else(|| StatusCode::UNAUTHORIZED.into_response())?;
        if !app_state.allowed_emails.contains(&email) {
            return Err(StatusCode::FORBIDDEN.into_response());
        }

        Ok(AuthUser)
    }
}
