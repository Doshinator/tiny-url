// !tests/api/shorten.rs
use crate::helpers::spawn_app;

#[tokio::test]
async fn shorten_returns_201_for_valid_url() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let url = format!("{}/shorten", app.address);

    let response = client
        .post(url)
        .json(&serde_json::json!({
            "long_url": "https://www.google.com"
        }))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status().as_u16(), 201);

    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body["short_code"].as_str().is_some());
    assert!(body["short_url"].as_str().is_some());
}

#[tokio::test]
async fn shorten_returns_400_bad_request() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let url = format!("{}/shorten", app.address);
    
    let response = client
        .post(url)
        .json(&serde_json::json!({
            "long_url": "Some invalid url"
        }))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn shorten_returns_409_alias_taken() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let url = format!("{}/shorten", app.address);

    // first request — claim the alias
    client
        .post(&url)
        .json(&serde_json::json!({
            "long_url": "https://www.google.com",
            "alias": "my-alias"
        }))
        .send()
        .await
        .expect("Failed to execute first request");

    let response = client
        .post(url)
        .json(&serde_json::json!({
            "long_url": "https://www.google.com",
            "alias": "my-alias"
        }))
        .send()
        .await
        .expect("Failed to execute second request");

    assert_eq!(response.status().as_u16(), 409);
}