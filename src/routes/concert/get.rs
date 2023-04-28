use actix_web::{web, ResponseError, HttpResponse};
use reqwest::StatusCode;
use anyhow::Context;

use crate::{routes::error_chain_fmt, domain::Concert};



#[derive(thiserror::Error)]
pub enum GetConcertError {
    #[error("Concert not found")]
    NotFoundError,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for GetConcertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for GetConcertError {
    fn status_code(&self) -> reqwest::StatusCode {
        match self {
            GetConcertError::NotFoundError => StatusCode::NOT_FOUND,
            GetConcertError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[tracing::instrument(
    name = "Getting a concert", 
    skip(id, pool)
)]
pub async fn get_concert(
    id: web::Path<String>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, GetConcertError> {
    let id = uuid::Uuid::parse_str(&id.into_inner())
        .context("Failed to parse concert ID")?;

    let concert = Concert::find_by_id(id, &pool)
        .await
        .context("Failed to find concert")?;

    match concert {
        Some(concert) => Ok(HttpResponse::Ok().json(concert)),
        None => Err(GetConcertError::NotFoundError),
    }
}

#[tracing::instrument(
    name = "Getting all concerts", 
    skip(pool)
)]
pub async fn get_concerts(
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, GetConcertError> {
    let concerts = Concert::find_all(&pool).await;

    match concerts {
        Ok(concerts) => Ok(HttpResponse::Ok().json(&concerts)),
        Err(_) => Err(GetConcertError::UnexpectedError(
            anyhow::Error::msg("Failed to fetch concerts")
        )),
    }
}
