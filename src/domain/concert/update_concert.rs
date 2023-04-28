use super::{
    ConcertVenue, 
    ConcertCity, 
    ConcertState, 
    ConcertDate, 
    ConcertCountry
};



pub struct UpdateConcert {
    pub id: uuid::Uuid,
    pub artist_id: uuid::Uuid,
    pub venue: ConcertVenue,
    pub city: ConcertCity,
    pub state: ConcertState,
    pub date: ConcertDate,
    pub country: ConcertCountry,
}
