use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ConcertDate(chrono::NaiveDate);

impl ConcertDate {
    pub fn parse(s: String) -> Result<ConcertDate, String> {
        match chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d") {
            Ok(date) => Ok(ConcertDate(date)),
            Err(e) => Err(format!("Error parsing date: {}", e)),
        }
    }
}
 impl AsRef<chrono::NaiveDate> for ConcertDate {
    fn as_ref(&self) -> &chrono::NaiveDate {
        &self.0
    }
 }
