use unicode_segmentation::UnicodeSegmentation;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConcertCountry(String);

impl ConcertCountry {
    pub fn parse(s: String) -> Result<Self, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;

        let forbidden_characters = ['<', '>', '"', '`', '%', '!', '@', '$', '^', '&', '*', '(', ')', '=', '+', '{', '}', '[', ']', '|', '\\', ';', ':', '/', '?', ',', '~', '#', '\''];
        let contains_forbidden_characters = s.graphemes(true).any(|g| forbidden_characters.contains(&g.chars().next().unwrap()));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            return Err("Invalid country".to_string())
        }
            
        Ok(Self(s))
    }
}

impl AsRef<str> for ConcertCountry {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
