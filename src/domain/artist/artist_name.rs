use unicode_segmentation::UnicodeSegmentation;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ArtistName(String);

impl ArtistName {
    pub fn parse(s: String) -> Result<ArtistName, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;

        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '{', '}', '\\'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!("{} is not a valid artist name", s))
        }
        else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for ArtistName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::ArtistName;
    use claims::{assert_ok, assert_err};

    #[test]
    fn a_256_long_grapheme_is_ok() {
        let name = "å".repeat(256);
        assert_ok!(ArtistName::parse(name));
    }

    #[test]
    fn a_name_longer_than_256_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(ArtistName::parse(name));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert_err!(ArtistName::parse(name));
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(ArtistName::parse(name));
    }

    #[test]
    fn name_containing_invalid_characters_is_rejected() {
        for name in &['/', '(', ')', '"', '>', '<', '{', '}', '\\'] {
            let name = name.to_string();
            assert_err!(ArtistName::parse(name));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_correctly() {
        let name = "Billy Strings".to_string();
        assert_ok!(ArtistName::parse(name));
    }
}
