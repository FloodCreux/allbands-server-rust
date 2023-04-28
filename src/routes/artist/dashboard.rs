use actix_web::{web, http::header::ContentType};
use actix_web::HttpResponse;
use crate::domain::Artist;
use sqlx::PgPool;



pub async fn artists_dashboard(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let artists = Artist::find_all(&pool)
        .await
        .expect("Failed to retrieve artists");

    let artist_list = artists
        .iter()
        .map(|artist| {
            format!(
                r#"<li><a href="/artists/{id}">{name}</a></li>"#,
                id = artist.id,
                name = artist.name
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
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Artist Dashboard</title>
</head>
<body>
    <h1>Artists</h1>
    <ul>
        {artist_list}
    </ul>
</body>
            "#,
        )))
}
