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

    // Assert
    assert_eq!(response.status().as_u16(), 302);
    assert_eq!(
        response.headers().get("Location").unwrap(),
        "https://www.google.com"
    );
}

#[tokio::test]
async fn redirect_returns_400_invalid_short_code() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let short_code = "bad$code";

    // Act
    let response = client
        .get(format!("{}/{}", app.address, short_code))
        .send()
        .await
        .expect("Failedto execute GET request");

    // Assert
    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn redirect_404_short_code_doesnt_exist() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let short_code = "inexisting-short-code";

    // Act
    let response = client
        .get(format!("{}/{}", app.address, short_code))
        .send()
        .await
        .expect("Failed to execute GET request");

    // Assert
    assert_eq!(response.status().as_u16(), 404);   
}

#[tokio::test]
async fn redirect_410_expired_short_code() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let short_code = "expired";
    let long_url = "https://www.google.com";

    sqlx::query!(
        "INSERT INTO urls (id, short_code, long_url, created_at, expires_at)
        VALUES ($1, $2, $3, now(), now() - interval '1 day')",
        uuid::Uuid::new_v4(),
        short_code,
        long_url,
    )
    .execute(&app.db_pool)
    .await
    .unwrap();

    let response = client
        .get(format!("{}/{}", app.address, short_code))
        .send()
        .await
        .expect("Failed to execute GET request");

    // Assert
    assert_eq!(response.status().as_u16(), 410);      
}