use crate::domain::{Artist, Concert};
use crate::routes::error_chain_fmt;
use actix_web::{web, ResponseError, HttpResponse, http::header::ContentType};
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
pub async fn artist_dashboard(
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

    let artist = result.unwrap(); 
    let artist_name = artist.name;
    let _artist_id = artist.id;
    let artist_sort_name = artist.sort_name;
    let artist_disambiguation = artist.disambiguation;

    let concerts = Concert::find_all(&pool)
        .await
        .context("Failed to get concerts")?;

    let concert_list = concerts
        .into_iter()
        .filter(|concert| concert.artist_id == id)
        .map(|concert| {
            format!(
                r#"<li><a href="/concerts/{id}">{date} {venue}</a></li>"#,
                id = concert.id,
                date = concert.date,
                venue = concert.venue
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    Ok(HttpResponse::Ok()
       .content_type(ContentType::html())
       .body(format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="Content-Type" content="text/html; charset=utf-8"/>
    <title>{artist_name}</title>
</head>
<body>
    <h1>{artist_name}</h1>
    <h2>{artist_sort_name}</h2>
    <h3>{artist_disambiguation}</h3>
    <ul>
        {concert_list}
    </ul>
</body>
</html>
        "#
        )))
}

#[tracing::instrument(
    name = "Get all artists",
    skip(pool)
)]
pub async fn get_artists(
    pool: web::Data<PgPool>
) -> Result<HttpResponse, GetArtistError> {
    let result = Artist::find_all(&pool)
        .await
        .context("Failed to get artists")?;

    Ok(HttpResponse::Ok().json(result))
}

