use log;
use sqlx::{PgPool, Result};

#[derive(Debug)]
pub struct Redirection {
    pub from: String,
    to: Option<String>,
}

impl Redirection {
    pub fn new(from: String) -> Self {
        Self { from, to: None }
    }

    pub async fn get_url(&mut self, pool: &PgPool) -> Result<Option<String>, sqlx::Error> {
        if let Some(url) = &self.to {
            return Ok(Some(url.clone()));
        }

        let row: (String,) = sqlx::query_as("SELECT url_to FROM Redirects WHERE url_from = $1")
            .bind(&self.from)
            .fetch_one(pool)
            .await?;
        log::debug!("Running a query to get url.");

        let result = if row.0.is_empty() {
            None
        } else {
            Some(row.0)
        };
        self.to = result.clone();
        Ok(result)
    }

    pub async fn set_outcoming_url(&mut self, url: &str, user: &str, pool: &PgPool) -> Result<(), sqlx::Error> {
        let user_id: Result<(String, ), _> = sqlx::query_as("SELECT user_id FROM Users WHERE user_id = $1 LIMIT 1")
            .bind(&user)
            .fetch_one(pool)
            .await;

        if let Err(e) = user_id {
            log::error!("User {} does not exist! Error: {}", user, e);
            return Err(e);
        }

        if self.from.is_empty() {
            log::error!("No \"from\" url set!");
            return Ok(());
        }

        let redirect_id: (String,) = sqlx::query_as("INSERT INTO Redirects (url_from, url_to) VALUES ($1, $2) RETURNING redirect_id")
            .bind(&self.from)
            .bind(&self.to)
            .fetch_one(pool)
            .await?;
        log::info!("Inserted redirect with id {:?}", redirect_id.0);

        sqlx::query("INSERT INTO Redirects_Users (redirect_id, user_id) VALUES ($1, $2)")
            .bind(&redirect_id.0)
            .bind(&user)
            .execute(pool)
            .await?;
        log::info!("Inserted redirect for user {:?}", user);

        self.to = Some(url.to_string());
        Ok(())
    }
}
