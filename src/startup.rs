use std::net::TcpListener;
use actix_web::{App, HttpServer, dev::Server, web};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;
use crate::{configuration::Settings, routes::{health::health, redirect::redirect, shorten::shorten}};
use anyhow::Context;

pub struct AppState {
    pub db_pool: PgPool,
}

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(config: &Settings) -> Result<Self, anyhow::Error> {
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
        let state = web::Data::new(AppState { db_pool });
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
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(health)
            .service(shorten)
            .service(redirect)
            .app_data(state.clone())

    })
    .listen(tcp_listener)?
    .run();

    Ok(server)
}