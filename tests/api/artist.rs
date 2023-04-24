use crate::helpers::spawn_app;
use allbands::domain::Artist;
use uuid::Uuid;

#[tokio::test]
async fn artists_returns_400_for_invalid_data() {
    // Arrange
    let app = spawn_app().await;
    let test_cases = vec![
        (
            serde_json::json!({
                "name": "Billy Strings",
                "sort_name": "Strings, Billy"
            }),
            "Missing disambiguation"
        ),
        (
            serde_json::json!({
                "sort_name": "Strings, Billy",
                "disambiguation": "Bluegrass",
            }),
            "Missing name"
        ),
        (
            serde_json::json!({
                "name": "Billy Strings",
                "disambiguation": "Bluegrass",
            }),
            "Missing sort name"
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = app.post_artist(invalid_body).await;

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}

#[tokio::test]
async fn artist_returns_201_created_for_valid_data() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.post_artist(serde_json::json!({
        "name": "Billy Strings",
        "sort_name": "Strings, Billy",
        "disambiguation": "Bluegrass",
    }))
    .await;

    // Assert
    assert_eq!(
        201,
        response.status().as_u16(),
        "The API did not succeed to create a new Artist"
    );
}

#[tokio::test]
async fn invalid_artist_id_returns_404() {
    // Arrange
    let app = spawn_app().await;
    let invalid_id = Uuid::new_v4();

    // Act 
    let response = app.get_artist_by_id(invalid_id).await;

    // Assert
    assert_eq!(
        404,
        response.status().as_u16(),
        "The API didn't return 404 Not Found response for {}",
        invalid_id
    );
}

#[tokio::test]
async fn get_artist_returns_200_ok() {
    // Arrange
    let app = spawn_app().await;

    let response = app.post_artist(serde_json::json!({
        "name": "Billy Strings",
        "sort_name": "Strings, Billy",
        "disambiguation": "Bluegrass",
    }))
    .await;

    // Assert - Part 1 - Create a new artist
    assert_eq!(
        201,
        response.status().as_u16(),
        "The API did not succeed to create a new Artist"
    );

    let artist_id = response.json::<Artist>()
        .await
        .expect("Failed to parse response")
        .id;
    // Act
    let response = app.get_artist_by_id(artist_id).await;

    assert_eq!(
        200,
        response.status().as_u16(),
        "The API did not return a 200 OK for {}",
        artist_id
    );
}
