use sqlx::{types::Uuid, PgPool};

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct User {
    user_id: Option<Uuid>,
    authenticated: bool,
    pub name: String,
    pub private_key: String,
}

impl User {
    pub fn new(name: String, private_key: String) -> Self {
        Self {
            user_id: None,
            // NOTE: Add authentication
            authenticated: true,
            name: name.to_string(),
            private_key: private_key.to_string(),
        }
    }

    pub async fn save(&self, pool: PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "
INSERT INTO Users (name, private_key)
values ($1, $2)
",
        )
        .bind(&self.name)
        .bind(&self.private_key)
        .execute(&pool).await?;
        Ok(())
    }
}
