use std::net::TcpListener;
use actix_governor::{Governor};
use actix_web::{App, HttpServer, dev::Server, web};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;
use crate::{configuration::Settings, middleware::{redirect_governor, shorten_governor}, routes::{health::health, redirect::redirect, shorten::shorten}};
use anyhow::Context;
use deadpool_redis::Pool as RedisPool;
use deadpool_redis::{Config as RedisConfig, Runtime};

pub struct AppState {
    pub db_pool: PgPool,
    pub redis_pool: RedisPool,
}

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(config: &Settings) -> Result<Self, anyhow::Error> {
        // redis
        let redis_config = RedisConfig::from_url(&config.redis.url);
        let redis_pool = redis_config
            .create_pool(Some(Runtime::Tokio1))
            .context("Failed to create Redis pool")?;
        
        // db
        let db_pool = PgPool::connect_lazy_with(
            config.database.connect_option()
        );

        let address = format!(
            "{}:{}",
            config.application.host,
            config.application.port
        );

        let listener = TcpListener::bind(address)
            .context("Failed to bind TCP listener")?;

        let port = listener.local_addr().unwrap().port();
        let state = web::Data::new(AppState { db_pool, redis_pool });
        let server = run(listener, state)
            .context("Failed to start HTTP server")?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

fn run(
    tcp_listener: TcpListener, 
    state: web::Data<AppState>
) -> Result<Server, std::io::Error> {
    let shorten_gov = shorten_governor();
    let redirect_gov = redirect_governor();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(state.clone())
            .service(health)
            .service(
                web::scope("/shorten")
                .wrap(Governor::new(&shorten_gov))
                .service(shorten)
            )
            .service(
                web::scope("")
                .wrap(Governor::new(&redirect_gov))
                .service(redirect)
            )
    })
    .listen(tcp_listener)?
    .run();

    Ok(server)
}