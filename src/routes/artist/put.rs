use crate::domain::{
    Artist, 
    ArtistName, 
    UpdateArtist, 
};
use crate::routes::error_chain_fmt;
use actix_web::{web, HttpResponse, ResponseError};
use reqwest::StatusCode;
use sqlx::PgPool;
use anyhow::Context;

#[derive(serde::Deserialize)]
pub struct BodyData {
    id: uuid::Uuid,
    name: String,
    sort_name: String,
    disambiguation: String,
}

impl TryFrom<BodyData> for UpdateArtist {
    type Error = String;

    fn try_from(value: BodyData) -> Result<Self, Self::Error> {
        let id = value.id;
        let name = ArtistName::parse(value.name)?;
        let sort_name = value.sort_name;
        let disambiguation = value.disambiguation;

        Ok(Self{ 
            id,
            name, 
            sort_name, 
            disambiguation, 
        })
    }
}

#[derive(thiserror::Error)]
pub enum UpdateArtistError {
    #[error("{0}")]
    ValidationError(String),
    #[error("{0}")]
    UnexpectedError(#[from] anyhow::Error)
}

impl std::fmt::Debug for UpdateArtistError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for UpdateArtistError {
    fn status_code(&self) -> reqwest::StatusCode {
        match self {
            UpdateArtistError::ValidationError(_) => StatusCode::BAD_REQUEST,
            UpdateArtistError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[tracing::instrument(
    name = "Updating an artist in the database",
    skip(artist, pool)
)]
pub async fn update_artist(
    id: web::Path<uuid::Uuid>,
    artist: web::Json<BodyData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, UpdateArtistError> {
    let artist = UpdateArtist::try_from(artist.into_inner())
        .map_err(|e| UpdateArtistError::ValidationError(e))?;
    if artist.id != *id {
        return Err(UpdateArtistError::ValidationError(
            "The artist id in the path does not match the artist id in the body".to_string(),
        ));
    }

    let mut transaction = pool.begin().await.context("Failed to acquire a Postgres connection")?;

    let result = Artist::update(&artist, &mut transaction)
        .await
        .context("Failed to insert artist")?;

    transaction.commit().await.context("Failed to commit transaction")?;

    Ok(HttpResponse::Ok().json(result))
}

