use crate::{
    domain::{
        Concert, 
        NewConcert, 
        ConcertVenue,
        ConcertCity,
        ConcertState,
        ConcertDate,
        ConcertCountry,
    }, 
    routes::error_chain_fmt
};
use actix_web::{web, HttpResponse, ResponseError};
use reqwest::StatusCode;
use sqlx::PgPool;
use anyhow::Context;

#[derive(serde::Deserialize)]
pub struct CreateConcertRequest {
    pub artist_id: uuid::Uuid,
    pub venue: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub date: String,
}

impl TryFrom<CreateConcertRequest> for NewConcert {
    type Error = String;

    fn try_from(value: CreateConcertRequest) -> Result<Self, Self::Error> {
        let artist_id = value.artist_id;
        let venue = ConcertVenue::parse(value.venue)?;
        let city = ConcertCity::parse(value.city)?;
        let state = ConcertState::parse(value.state)?;
        let country = ConcertCountry::parse(value.country)?;
        let date = ConcertDate::parse(value.date)?;

        Ok(Self {
            artist_id,
            venue,
            city,
            state,
            country,
            date,
        })
    }
}

#[derive(thiserror::Error)]
pub enum CreateConcertError {
    #[error("{0}")]
    UnexpectedError(#[from] anyhow::Error),
    #[error("{0}")]
    ValidationError(String),
}

impl std::fmt::Debug for CreateConcertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for CreateConcertError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
        }
    }
}

#[tracing::instrument(
    name = "Adding a new concert",
    skip(body, pool),
)]
pub async fn create_concert(
    body: web::Json<CreateConcertRequest>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, CreateConcertError> {
    let new_concert = body.0.try_into().map_err(CreateConcertError::ValidationError)?;

    let mut transaction = pool
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")?;

    let concert = Concert::insert(&new_concert, &mut transaction)
        .await
        .context("Failed to insert a new concert")?;

    Ok(HttpResponse::Created().json(&concert))
}
