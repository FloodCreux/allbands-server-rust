use crate::domain::NewConcert;
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
            item.artist_id,
            item.venue.as_ref(),
            item.city.as_ref(),
            item.state,
            item.country,
            item.date,
            chrono::Utc::now(),
        )
        .fetch_one(transaction)
        .await
        .expect("Failed to insert concert");

        Ok(entity)
    }
}
