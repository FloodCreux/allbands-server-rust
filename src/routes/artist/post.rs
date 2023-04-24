use crate::domain::NewArtist;
use actix_web::{web, HttpResponse, ResponseError};
use reqwest::StatusCode;
use sqlx::{PgPool, Transaction, Postgres};
use anyhow::Context;
use uuid::Uuid;
use chrono::Utc;

#[derive(serde::Deserialize)]
pub struct BodyData {
    name: String,
    sort_name: String,
    disambiguation: String,
}

impl TryFrom<BodyData> for NewArtist {
    type Error = String;

    fn try_from(value: BodyData) -> Result<Self, Self::Error> {
        let name = value.name;
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
    fields(
       artist_name = %body.name,
       artist_sort_name = %body.sort_name,
       artist_disambiguation = %body.disambiguation,
    ),
)]
pub async fn create(
    body: web::Json<BodyData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ArtistError> {
    let new_artist = body.0.try_into().map_err(ArtistError::ValidationError)?;
    let mut transaction = pool
        .begin()
        .await
        .context("Failed to acquire a Postgres transaction from the database")?;

    let _artist_id = insert_artist(&mut transaction, &new_artist)
        .await
        .context("Failed to insert artist into the database")?;

    Ok(HttpResponse::Ok().finish())

}

#[tracing::instrument(
    name = "Inserting artist into the database",
    skip(transaction, new_artist)
)]
async fn insert_artist(
    transaction: &mut Transaction<'_, Postgres>,
    new_artist: &NewArtist,
) -> Result<Uuid, sqlx::Error> {
    let artist_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO artists (id, name, sort_name, disambiguation, created_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        artist_id,
        new_artist.name,
        new_artist.sort_name,
        new_artist.disambiguation,
        Utc::now(),
    )
    .execute(transaction)
    .await
    .map_err(|e| {
        e
    })?;

    Ok(artist_id)
}
