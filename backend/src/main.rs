use axum::{routing::post, Router};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = simplelog::WriteLogger::init(
        log::LevelFilter::Debug,
        simplelog::Config::default(),
        std::fs::File::create("url.log").unwrap(),
    );
    log::info!("Initialized logger");

    let config = url::config::Config::build()?;
    let user = url::user::User::new("Admin".to_string(), "1234".to_string());

    let db_string = format!(
        "postgres://{}:{}@{}/{}",
        config.login.user, config.login.password, config.db.server_adress, config.db.database_alias
    );
    let pool: sqlx::postgres::PgPool = sqlx::postgres::PgPoolOptions::new()
        .connect(&db_string)
        .await?;
    log::info!("Connected to db");

    let app_state = url::config::AppState { config, pool, user };

    let app = Router::new()
        .route(&app_state.config.api.url_directory, post(url::endpoints::return_url))
        .with_state(app_state);
    log::info!("Initilized router");

    // NOTE: Add subdomain api.host
    let listener = tokio::net::TcpListener::bind("localhost:8000")
        .await.unwrap();
    axum::serve(listener, app).await?;

    log::info!("Shutting down");
    Ok(())
}
