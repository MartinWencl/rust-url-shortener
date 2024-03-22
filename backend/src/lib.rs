use crate::user::User;
use crate::config::Config;
use redirection::Redirection;
use sqlx::{self, PgPool};
use std::error::Error;

mod redirection;
pub mod user;
pub mod config;
pub mod endpoints;
pub mod model;

#[derive(Debug)]
pub enum Actions {
    AddUser(String, String),
    AddRedirection(String, String),
    GetRedirectUrl(String),
}

pub async fn run(action: Actions, config: Config, pool: PgPool) -> Result<String, Box<dyn Error>> {
    match action {
        Actions::AddUser(name, private_key) => {
            log::info!("Added user {}", name);
            User::new(name, private_key).save(pool).await?;
            Ok("OK".to_string())
        }
        Actions::GetRedirectUrl(from) => {
            // TODO: Add handling if the url if empty - some 404 page or whatever
            // now just unwraping
            let to = Redirection::new(from.clone()).get_url(&pool).await?.unwrap();
            log::info!("Retrieved redirection from {} to {}", from, to);
            Ok(to)
        }
        Actions::AddRedirection(from, to) => {
            Redirection::new(from.clone())
                .set_outcoming_url(&to, &config.login.user, &pool)
                .await?;
            log::info!("Added a new redirection form {} to {}", from, to);
            Ok(to)
        }
    }
}
