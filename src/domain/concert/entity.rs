use crate::domain::{NewConcert, UpdateConcert};
use sqlx::{Postgres, Transaction};


#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Concert {
    pub id: uuid::Uuid,
    pub artist_id: uuid::Uuid,
    pub venue: String,
    pub city: String,
    pub state: Option<String>,
    pub country: String,
    pub date: chrono::NaiveDate,
}

impl Concert {
    #[tracing::instrument(
        name = "Create a new concert",
        skip(item, transaction)
    )]
    pub async fn insert(
        item: &NewConcert,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Self, sqlx::Error> {
        let concert_id = uuid::Uuid::new_v4();
        
        let entity = sqlx::query_as!(
            Concert,
            r#"
            INSERT INTO concerts (id, artist_id, venue, city, state, country, date, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, artist_id, venue, city, state, country, date
            "#,
            concert_id,
            &item.artist_id,
            &item.venue.as_ref(),
            &item.city.as_ref(),
            &item.state.as_ref(),
            &item.country.as_ref(),
            &item.date.as_ref(),
            chrono::Utc::now(),
        )
        .fetch_one(transaction)
        .await
        .expect("Failed to insert concert");

        Ok(entity)
    }

    #[tracing::instrument(
        name = "Find all concerts",
        skip(pool)
    )]
    pub async fn find_all(pool: &sqlx::PgPool) -> Result<Vec<Self>, sqlx::Error> {
        let entities = sqlx::query_as!(
            Self,
            r#"
            SELECT id, artist_id, venue, city, state, country, date
            FROM concerts
            ORDER BY date
            "#,
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .collect();

        Ok(entities)
    }

    #[tracing::instrument(
        name = "Find a concert by id",
        skip(id, pool)
    )]
    pub async fn find_by_id(
        id: uuid::Uuid,
        pool: &sqlx::PgPool,
    ) -> Result<Option<Self>, sqlx::Error> {
        let entity = sqlx::query_as!(
            Concert,
            r#"
            SELECT id, artist_id, venue, city, state, country, date
            FROM concerts
            WHERE id = $1
            "#,
            id,
        )
        .fetch_optional(pool)
        .await
        .expect("Failed to find concert");

        Ok(entity)
    }

    #[tracing::instrument(
        name = "Update Concert",
        skip(item, transaction)
    )]
    pub async fn update(
        item: &UpdateConcert,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Self, sqlx::Error> {
        let entity = sqlx::query_as!(
            Concert,
            r#"
            UPDATE concerts
            SET artist_id = $1, venue = $2, city = $3, state = $4, country = $5, date = $6
            WHERE id = $7
            RETURNING id, artist_id, venue, city, state, country, date
            "#,
            &item.artist_id,
            &item.venue.as_ref(),
            &item.city.as_ref(),
            &item.state.as_ref(),
            &item.country.as_ref(),
            &item.date.as_ref(),
            &item.id,
        )
        .fetch_one(transaction)
        .await
        .expect("Failed to update concert");

        Ok(entity)
    }
}
