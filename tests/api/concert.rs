use crate::helpers::spawn_app;
use allbands::domain::{Concert, Artist};

#[tokio::test]
async fn concerts_returns_400_for_invalid_data() {
    // Arrange
    let app = spawn_app().await;
    let artist_id = app.post_artist(serde_json::json!({
        "name": "Billy Strings",
        "sort_name": "Strings, Billy",
        "disambiguation": "Bluegrass musician from Lansing, MI",
    }))
    .await
    .json::<Artist>()
    .await
    .expect("Failed to deserialize the artist")
    .id;

    let test_cases = vec![
        (
            serde_json::json!({
                "venue": "The Fillmore",
                "city": "San Francisco",
                "state": "CA",
                "country": "USA",
                "date": "2021-07-17"
            }),
            "Missing Artist Id"
        ),
        (
            serde_json::json!({
                "artist_id": artist_id,
                "city": "San Francisco",
                "state": "CA",
                "country": "USA",
                "date": "2021-07-17"
            }),
            "Missing Venue"
        ),
        (
            serde_json::json!({
                "artist_id": artist_id,
                "venue": "The Fillmore",
                "state": "CA",
                "country": "USA",
                "date": "2021-07-17"
            }),
            "Missing City"
        ),
        (
            serde_json::json!({
                "artist_id": artist_id,
                "venue": "The Fillmore",
                "city": "San Francisco",
                "country": "USA",
                "date": "2021-07-17"
            }),
            "Missing State"
        ),
        (
            serde_json::json!({
                "artist_id": artist_id,
                "venue": "The Fillmore",
                "city": "San Francisco",
                "state": "CA",
                "date": "2021-07-17"
            }),
            "Missing Country"
        ),
        (
            serde_json::json!({
                "artist_id": artist_id,
                "venue": "The Fillmore",
                "city": "San Francisco",
                "state": "CA",
                "country": "USA",
            }),
            "Missing Date"
        ),
    ];

    for(invalid_body, error_message) in test_cases {
        // Act
        let response = app.post_concert(invalid_body).await;

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
