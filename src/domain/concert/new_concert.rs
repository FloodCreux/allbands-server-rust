use super::{ConcertVenue, ConcertCity, ConcertState};


#[derive(serde::Deserialize)]
pub struct NewConcert {
    pub artist_id: uuid::Uuid,
    pub venue: ConcertVenue,
    pub city: ConcertCity,
    pub state:ConcertState,
    pub country: String,
    pub date: chrono::NaiveDate
}
