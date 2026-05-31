use tiny_url::{
    configuration::{get_configuration, DatabaseSettings},
    startup::Application,
    telemetry::{get_subscriber, init_subscriber},
};
use sqlx::{PgPool, PgConnection, Connection, Executor};
use uuid::Uuid;
use std::sync::OnceLock;

static TRACING: OnceLock<()> = OnceLock::new();

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    TRACING.get_or_init(|| {
        let subscriber = get_subscriber(
            "test".into(), 
            "debug".into(), 
            std::io::sink  // suppress logs during tests
        );
        init_subscriber(subscriber);
    });

    let config = get_configuration()
        .expect("Failed to read configuration");

    // config setup for test
    config.application.port = 0;
    config.database.database_name = format!("test_{}", Uuid::new_v4().to_string().replace("-", ""));
    configure_test_database(&config.database).await;

    // spin up app
    let app = Application::build(&config)
        .await
        .expect("Failed to build application");

    let port = app.port();
    let _ = tokio::spawn(app.run_until_stopped());

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db_pool: PgPool::connect_with(config.database.connect_option())
            .await
            .expect("Failed to connect to test database"),
    }
}

async fn configure_test_database(config: &DatabaseSettings) -> PgPool {
    // connect to postgres itself (not your app db) to create the test db
    let mut connection = PgConnection::connect_with(
        &config.connect_option_without_db()
    )
    .await
    .expect("Failed to connect to Postgres");

    connection
        .execute(
            format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str()
        )
        .await
        .expect("Failed to create test database");

    // now connect to the new db and run migrations
    let pool = PgPool::connect_with(config.connect_option())
        .await
        .expect("Failed to connect to test database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}