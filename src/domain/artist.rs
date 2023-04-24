use uuid::Uuid;
use sqlx::{PgPool, Transaction, Postgres};

use super::NewArtist;

#[derive(Debug, serde::Serialize,serde::Deserialize, sqlx::FromRow)]
pub struct Artist {
    pub id: Uuid,
    pub name: String,
    pub sort_name: String,
    pub disambiguation: String,
}

impl Artist {
    /// Find an artist given an artist identifier
    ///
    /// Returns the artist or throws a `sqlx::Error`
    #[tracing::instrument(
        name = "Find artist by id",
        skip(id, pool)
    )]
    pub async fn find_by_id(
        id: Uuid, 
        pool: &PgPool
    ) -> Result<Option<Self>, sqlx::Error> {
        let entity = sqlx::query_as!(
            Artist,
            r#"
            SELECT id, name, sort_name, disambiguation
            FROM artists
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await
        .expect("Failed to execute request");

        Ok(entity)
    }

    #[tracing::instrument(
        name = "Inserting artist into the database",
        skip(transaction, item)
    )]
    pub async fn insert(
        item: &NewArtist,
        transaction: &mut Transaction<'_, Postgres>,
        ) -> Result<Self, sqlx::Error> {
        let artist_id = Uuid::new_v4();

        let entity = sqlx::query_as!(
            Artist,
            r#"
            INSERT INTO artists(id, name, sort_name, disambiguation, created_at)
            VALUES($1, $2, $3, $4, $5)
            RETURNING id, name, sort_name, disambiguation
            "#,
            artist_id,
            &item.name.as_ref(),
            &item.sort_name,
            &item.disambiguation,
            chrono::Utc::now(),
            )
            .fetch_one(transaction)
            .await
            .expect("Failed to execute request");

        Ok(entity)
    }
}
