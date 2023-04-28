use crate::domain::{
    Concert,
    UpdateConcert,
    ConcertDate,
    ConcertState,
    ConcertCity,
    ConcertVenue,
    ConcertCountry,
};
use crate::routes::error_chain_fmt;
use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;
use reqwest::StatusCode;
use anyhow::Context;

#[derive(serde::Deserialize)]
pub struct UpdateConcertRequest {
    pub id: uuid::Uuid,
    pub artist_id: uuid::Uuid,
    pub venue: String,
    pub city: String,
    pub state: String,
    pub date: String,
    pub country: String,
}

impl TryFrom<UpdateConcertRequest> for UpdateConcert {
    type Error = String;

    fn try_from(
        request: UpdateConcertRequest
    ) -> Result<Self, Self::Error> {
        let venue = ConcertVenue::parse(request.venue)?;
        let city = ConcertCity::parse(request.city)?;
        let state = ConcertState::parse(request.state)?;
        let date = ConcertDate::parse(request.date)?;
        let country = ConcertCountry::parse(request.country)?;

        Ok(Self {
            id: request.id,
            artist_id: request.artist_id,
            venue,
            city,
            state,
            date,
            country,
        })
    }
}

#[derive(thiserror::Error)]
pub enum UpdateConcertError { 
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

impl std::fmt::Debug for UpdateConcertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for UpdateConcertError {
    fn status_code(&self) -> StatusCode {
        match self {
            UpdateConcertError::ValidationError(_) => StatusCode::BAD_REQUEST,
            UpdateConcertError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[tracing::instrument(
    name = "Updating a concert in the database",
    skip(pool, item)
)]
pub async fn update_concert(
    id: web::Path<uuid::Uuid>,
    item: web::Json<UpdateConcertRequest>,
    pool: web::Data<PgPool>
) -> Result<HttpResponse, UpdateConcertError> {
    let concert = UpdateConcert::try_from(item.into_inner())
        .map_err(|e| UpdateConcertError::ValidationError(e))?;

    if concert.id != *id {
        return Err(UpdateConcertError::ValidationError(
            "The object id does not match the id in the URL".to_string()
        ));
    }

    let mut transaction = pool.begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")?;

    let result = Concert::update(&concert, &mut transaction)
        .await
        .context("Failed to update the concert in the database")?;

    transaction.commit().await.context("Failed to commit the transaction")?;

    Ok(HttpResponse::Ok().json(result))
}
