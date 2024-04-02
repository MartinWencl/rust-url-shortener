use axum::{routing::post, Router};
use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    log::info!("Starting - {}", env::args().next().take().unwrap());
    let config = url::config::Config::build()?;
    let user = url::user::User::new("Admin".to_string(), "1234".to_string());

    if let Err(e) = simplelog::WriteLogger::init(
        log::LevelFilter::Debug,
        simplelog::Config::default(),
        std::fs::File::create(&config.log.path).unwrap(),
    ) {
        log::error!("Logger failed to initiliaze!");
        log::error!("Error: {}", e);
        log::error!("current config {:?}", config);
    };
    log::debug!("Initialized logger");

    let db_string = format!(
        "postgres://{}:{}@{}/{}",
        config.login.user, config.login.password, config.db.server_adress, config.db.database_alias
    );

    let pool: sqlx::postgres::PgPool = sqlx::postgres::PgPoolOptions::new()
        .connect(&db_string)
        .await?;
    log::debug!("Connected to db");

    // config clone used for error logging
    let logging_config = config.clone();

    let app_state = url::config::AppState { config, pool, user };

    let app = Router::new()
        .route(&app_state.config.api.url_directory, post(url::endpoints::return_url))
        .with_state(app_state.clone());
    log::debug!("Initilized router");

    // NOTE: Add subdomain api.host
    let adress = "localhost:8000";
    let listener = tokio::net::TcpListener::bind(adress).await.or_else(|_| {
        log::error!("Listening on adress {} failed!", adress);
        log::error!("Current config: {:?}", logging_config);
        Err(format!("Failed while trying to listen on given adress - {}.", adress))
    }).unwrap();
    axum::serve(listener, app).await?;

    log::info!("Shutting down");
    Ok(())
}
