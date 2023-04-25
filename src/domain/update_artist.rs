use crate::domain::ArtistName;

pub struct UpdateArtist {
    pub id: uuid::Uuid,
    pub name: ArtistName,
    pub sort_name: String,
    pub disambiguation: String,
}

