use crate::domain::{Artist, ArtistName, NewArtist};
use actix_web::{web, HttpResponse, ResponseError};
use reqwest::StatusCode;
use sqlx::PgPool;
use anyhow::Context;

#[derive(serde::Deserialize)]
pub struct BodyData {
    name: String,
    sort_name: String,
    disambiguation: String,
}

impl TryFrom<BodyData> for NewArtist {
    type Error = String;

    fn try_from(value: BodyData) -> Result<Self, Self::Error> {
        let name = ArtistName::parse(value.name)?;
        let sort_name = value.sort_name;
        let disambiguation = value.disambiguation;

        Ok(Self{ name, sort_name, disambiguation })
    }
}

#[derive(thiserror::Error)]
pub enum ArtistError {
    #[error("{0}")]
    ValidationError(String),
    #[error("{0}")]
    UnexpectedError(#[from] anyhow::Error)
}

impl std::fmt::Debug for ArtistError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for ArtistError {
    fn status_code(&self) -> reqwest::StatusCode {
        match self {
            ArtistError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ArtistError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();

    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }

    Ok(())
}

#[tracing::instrument(
    name = "Add new artist",
    skip(body, pool),
)]
pub async fn create_artist(
    body: web::Json<NewArtist>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ArtistError> {
    let mut transaction = pool
        .begin()
        .await
        .context("Failed to acquire a Postgres transaction from the database")?;

    let created = Artist::insert(&body, &mut transaction)
        .await
        .context("Failed to create a new artist")?;

    transaction.commit()
        .await
        .context("Failed to commit the Postgres transaction")?;

    Ok(HttpResponse::Created().json(created))
}

