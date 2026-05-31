// !tests/api/shorten.rs
use crate::helpers::spawn_app;

#[tokio::test]
async fn shorten_returns_201_for_valid_url() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(format!("{}/shorten", app.address))
        .json(&serde_json::json!({
            "long_url": "https://www.google.com"
        }))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status().as_u16(), 201);

    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body["short-code"].as_str().is_some());
    assert!(body["short_url"].as_str().is_some());
}