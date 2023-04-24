use crate::routes::{create, health_check};
use crate::configuration::{DatabaseSettings, Settings};
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::cookie::Key;
use actix_web::{web, App, HttpServer};
use actix_web_flash_messages::FlashMessagesFramework;
use actix_web_flash_messages::storage::CookieMessageStore;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;
use secrecy::{Secret, ExposeSecret};


pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
            );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener, 
            connection_pool, 
            configuration.application.base_url,
            configuration.application.hmac_secret,
            ).await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }    
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

// We need to define a wrapper type in order to retrieve the URL
// in the `subscribe` handler.
// Retrieval from the context, in actix-web, is type-based: using
// a raw `String` would expose us to conflicts.
pub struct ApplicationBaseUrl(pub String);

async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    base_url: String,
    hmac_secret: Secret<String>,
) -> Result<Server, anyhow::Error> {
    let base_url = Data::new(ApplicationBaseUrl(base_url));
    let db_pool = Data::new(db_pool);

    let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());
    let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(message_framework.clone())
            .wrap(TracingLogger::default())
            .route("/health", web::get().to(health_check))
            .route("/artist", web::post().to(create))
            .app_data(db_pool.clone())
            .app_data(base_url.clone())
            .app_data(Data::new(HmacSecret(hmac_secret.clone())))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

#[derive(Clone)]
pub struct HmacSecret(pub Secret<String>);