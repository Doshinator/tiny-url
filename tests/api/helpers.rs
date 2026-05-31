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
