use super::{ConcertVenue, ConcertCity};


#[derive(serde::Deserialize)]
pub struct NewConcert {
    pub artist_id: uuid::Uuid,
    pub venue: ConcertVenue,
    pub city: ConcertCity,
    pub state: Option<String>,
    pub country: String,
    pub date: chrono::NaiveDate
}
