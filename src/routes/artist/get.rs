use crate::domain::Artist;
use crate::routes::error_chain_fmt;
use actix_web::{web, ResponseError, HttpResponse};
use anyhow::Context;
use reqwest::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(thiserror::Error)]
pub enum GetArtistError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("Artist not found")]
    NotFoundError,
}

impl std::fmt::Debug for GetArtistError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for GetArtistError {
    fn status_code(&self) -> reqwest::StatusCode {
        match self {
            Self::NotFoundError => StatusCode::NOT_FOUND,
            Self::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[tracing::instrument(
    name = "Get an artist",
    skip(id, pool)
)]
pub async fn get_artist_by_id(
    id: web::Path<String>,
    pool: web::Data<PgPool>
) ->Result<HttpResponse, GetArtistError> {
    let id = Uuid::parse_str(&id)
        .map_err(|_| GetArtistError::NotFoundError)?;
    
    let result = Artist::find_by_id(id, &pool)
        .await
        .context("Failed to get artist")?;
    if result.is_none() {
        return Err(GetArtistError::NotFoundError);
    }

    Ok(HttpResponse::Ok().json(result))
}
