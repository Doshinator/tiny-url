use crate::helpers::spawn_app;

#[tokio::test]
async fn successful_redirect_302() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let post_url = format!("{}/shorten", app.address);
    let short_code = "my-short-code";

    // Act
    client
        .post(post_url)
        .json(&serde_json::json!({
            "long_url": "https://www.google.com",
            "alias": short_code
        }))
        .send()
        .await
        .expect("Failed to post request");

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let response = client
        .get(format!("{}/{}", app.address, short_code))
        .send()
        .await
        .expect("Failed to execute GET request");

    assert_eq!(response.status().as_u16(), 302);
}