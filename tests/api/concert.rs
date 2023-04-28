use crate::helpers::spawn_app;
use allbands::domain::{Concert, Artist};

#[tokio::test]
async fn concerts_returns_201_created_for_valid_data() {
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

    // Act
    let response = app.post_concert(serde_json::json!({
        "artist_id": artist_id,
        "venue": "The Fillmore",
        "city": "San Francisco",
        "state": "CA",
        "country": "USA",
        "date": "2021-07-17"
    }))
    .await;

    // Assert
    assert_eq!(
        201,
        response.status().as_u16(),
        "The API did not return a 201 CREATED response");

    let concert = response.json::<Concert>()
        .await
        .expect("Failed to deserialize the concert");

    assert_eq!(concert.artist_id, artist_id);
    assert_eq!(concert.venue, "The Fillmore");
    assert_eq!(concert.city, "San Francisco");
    assert_eq!(concert.state, Some("CA".to_string()));
    assert_eq!(concert.country, "USA");
    assert_eq!(
        concert.date, 
        chrono::NaiveDate::parse_from_str("2021-07-17", "%Y-%m-%d").unwrap()
    );
    assert!(concert.id != uuid::Uuid::nil());
}

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

#[tokio::test]
pub async fn get_concert_returns_404_not_found_for_random_id() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let id = uuid::Uuid::new_v4();
    let response = app.get_concert_by_id(id).await;

    // Assert
    assert_eq!(
        404,
        response.status().as_u16(),
        "The API did not return a 404 NOT FOUND response for id {}",
        id,
    );
}

#[tokio::test]
pub async fn get_concert_returns_200_ok_valid_request() {
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

    let concert_id = app.post_concert(serde_json::json!({
        "artist_id": artist_id,
        "venue": "The Fillmore",
        "city": "San Francisco",
        "state": "CA",
        "country": "USA",
        "date": "2021-07-17"
    }))
    .await
    .json::<Concert>()
    .await
    .expect("Failed to deserialize the concert")
    .id;

    // Act
    let response = app.get_concert_by_id(concert_id).await;

    // Assert
    assert_eq!(
        200,
        response.status().as_u16(),
        "The API did not return a 200 OK response for id {}",
        concert_id,
    );
}


#[tokio::test]
async fn update_concert_returns_400_when_id_doesnt_match_body_id() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let result = app.update_concert(uuid::Uuid::new_v4(), serde_json::json!({
        "id": uuid::Uuid::new_v4(),
        "artist_id": uuid::Uuid::new_v4(),
        "venue": "The Fillmore",
        "city": "San Francisco",
        "state": "CA",
        "country": "USA",
        "date": "2021-07-17"
    }))
    .await;

    // Assert
    assert_eq!(
        400,
        result.status().as_u16(),
        "The API did not return a 400 BAD REQUEST response when the id in the body did not match the id in the URL"
    );
}

#[tokio::test]
async fn upgrade_concert_returns_200_for_valid_input() {
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

    let concert_id = app.post_concert(serde_json::json!({
        "artist_id": artist_id,
        "venue": "The Fillmore",
        "city": "San Francisco",
        "state": "CA",
        "country": "USA",
        "date": "2021-07-17"
    }))
    .await
    .json::<Concert>()
    .await
    .expect("Failed to deserialize the concert")
    .id;


    // Act
    let response = app.update_concert(concert_id, serde_json::json!({
        "id": concert_id,
        "artist_id": artist_id,
        "venue": "The Fillmore",
        "city": "San Francisco",
        "state": "CA",
        "country": "USA",
        "date": "2021-07-18"
    }))
    .await;

    // Assert
    assert_eq!(
        200,
        response.status().as_u16(),
        "The API did not return a 200 OK response when the input was valid"
    );

    let updated_concert = response.json::<Concert>().await.unwrap();
    assert_eq!(updated_concert.date, chrono::NaiveDate::parse_from_str("2021-07-18", "%Y-%m-%d").unwrap());
}
