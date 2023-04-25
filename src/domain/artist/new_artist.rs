use crate::domain::ArtistName;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewArtist {
    pub name: ArtistName,
    pub sort_name: String,
    pub disambiguation: String,
}
