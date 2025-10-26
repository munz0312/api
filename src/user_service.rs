use crate::model::{User, UserInfo};
use sqlx::PgPool;
use sqlx::error::Error;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[derive(Clone)]
pub struct UserService {
    pool: PgPool,
}

impl UserService {
    pub async fn new() -> Result<Self, Error> {
        dotenv::from_path("conf/secrets.env").ok();
        let url = env::var("DATABASE_URL").unwrap();
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&url)
            .await?;
        Ok(Self { pool })
    }

    pub async fn list_users(&self) -> Result<Vec<User>, Error> {
        let users = sqlx::query_as::<_, User>("SELECT id, email, password_hash, name, occupation, created_at FROM users")
            .fetch_all(&self.pool)
            .await?;
        Ok(users)
    }

    pub async fn get_users_by_id(&self, id: i32) -> Result<User, Error> {
        let user =
            sqlx::query_as::<_, User>("SELECT id, email, password_hash, name, occupation, created_at FROM users WHERE id = $1")
                .bind(id)
                .fetch_one(&self.pool)
                .await?;
        Ok(user)
    }

    pub async fn update_user(&self, id: i32, user: UserInfo) -> Result<(), Error> {
        sqlx::query("UPDATE users SET name = $1, occupation = $2 WHERE id = $3")
            .bind(user.name)
            .bind(user.occupation)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn delete_user(&self, id: i32) -> Result<(), Error> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn create_user_with_auth(
        &self,
        email: String,
        password_hash: String,
        name: String,
        occupation: String,
    ) -> Result<User, Error> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (email, password_hash, name, occupation) VALUES ($1, $2, $3, $4) RETURNING id, email, password_hash, name, occupation, created_at"
        )
        .bind(email)
        .bind(password_hash)
        .bind(name)
        .bind(occupation)
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<User, Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, password_hash, name, occupation, created_at FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }
}
