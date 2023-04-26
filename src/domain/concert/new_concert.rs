use super::{ConcertVenue, ConcertCity, ConcertState, ConcertDate, ConcertCountry};


#[derive(serde::Deserialize)]
pub struct NewConcert {
    pub artist_id: uuid::Uuid,
    pub venue: ConcertVenue,
    pub city: ConcertCity,
    pub state:ConcertState,
    pub country: ConcertCountry,
    pub date: ConcertDate,
}
