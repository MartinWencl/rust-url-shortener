use crate::user::User;
use crate::config::Config;
use crate::redirection::Redirection;
use sqlx::PgPool;
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
            User::new(name.clone(), private_key).save(pool).await?;
            log::info!("Added user {}", name);
            Ok("OK".to_string())
        }
        Actions::GetRedirectUrl(from) => {
            // TODO: Add handling if the url is empty - some 404 page or whatever
            // now just unwrapping
            let to = Redirection::new(from.clone()).get_url(&pool).await?.unwrap();
            log::info!("Retrieved redirection from {} to {}", from, to);
            Ok(to)
        }
        Actions::AddRedirection(from, to) => {
            Redirection::new(from.clone())
                .set_outgoing_url(&to, &config.login.user, &pool)
                .await?;
            log::info!("Added a new redirection from {} to {}, by user - {}", from, to, &config.login.user);
            Ok(to)
        }
    }
}

