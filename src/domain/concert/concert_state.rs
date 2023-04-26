use unicode_segmentation::UnicodeSegmentation;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConcertState(String);

impl ConcertState {
    pub fn parse(s: String) -> Result<Self, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 2;

        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '{', '}', '\\'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!("{} is not a valid state", s))
        }
        else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for ConcertState {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
