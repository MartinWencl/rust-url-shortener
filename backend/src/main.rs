use axum::{routing::post, Router};
use std::{env, error::Error};

/// This function initializes a web server using the Axum framework.
/// It sets up a logger, connects to a PostgreSQL database, and creates routes for handling API requests.
/// The server listens on localhost:8000 and serves the specified routes.
///
/// # Errors
/// This function can return an error if any of the following operations fail:
/// - Logger initialization fails
/// - Connection to the PostgreSQL database fails
/// - Binding to the specified address for listening fails

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = url::config::Config::build()?;

    // TODO: User log in
    let user = url::user::User::new("Admin".to_string(), "1234".to_string());

    // TODO: Set logger according to config
    if let Err(e) = simplelog::WriteLogger::init(
        log::LevelFilter::Debug,
        simplelog::Config::default(),
        std::fs::File::create(&config.log.path).unwrap(),
    ) {
        panic!("Logger failed to initialize! {}", e)
    };

    log::debug!("Initialized logger");
    log::info!("Starting - {}", env::args().next().take().unwrap());

    let db_string = format!(
        "postgres://{}:{}@{}/{}",
        config.login.user, config.login.password, config.db.server_adress, config.db.database_alias
    );

    let pool: sqlx::postgres::PgPool = sqlx::postgres::PgPoolOptions::new()
        .connect(&db_string)
        .await?;
    log::debug!("Connected to db");

    let app_state = url::config::AppState { config, pool, user };

    let app = Router::new()
        .route(&app_state.config.api.url_directory, post(url::endpoints::return_url))
        .with_state(app_state.clone());
    log::debug!("Initilized router");

    // NOTE: Add subdomain api.host
    let adress = "localhost:8000";
    let listener = tokio::net::TcpListener::bind(adress).await.or_else(|_| {
        log::error!("Listening on adress {} failed!", adress);
        log::error!("Current config: {:?}", &app_state.config);
        Err(format!("Failed while trying to listen on given adress - {}.", adress))
    }).unwrap();
    axum::serve(listener, app).await?;

    log::info!("Shutting down");
    Ok(())
}
