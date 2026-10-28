//! Local user directory repository implementation using SQLx.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};

use crate::domain::entities::{ApiError, Result, User};
use crate::domain::traits::UserRepository;

/// PostgreSQL user directory repository.
pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn map_row(row: &sqlx::postgres::PgRow) -> Result<User> {
        let id: String = row.try_get("id").map_err(|e| ApiError::internal(e.to_string()))?;
        let email: String = row.try_get("email").unwrap_or_default();
        let name: Option<String> = row.try_get("name").ok();
        let created_at: DateTime<Utc> = row.try_get("created_at").unwrap_or_else(|_| Utc::now());
        let updated_at: DateTime<Utc> = row.try_get("updated_at").unwrap_or_else(|_| Utc::now());

        Ok(User {
            id,
            email,
            name,
            created_at,
            updated_at,
        })
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn upsert<'a>(&self, id: &str, email: &str, name: Option<&'a str>) -> Result<User> {
        sqlx::query(
            r#"
            INSERT INTO users (id, email, name, created_at, updated_at)
            VALUES ($1, $2, $3, NOW(), NOW())
            ON CONFLICT (id) DO UPDATE
                SET email = EXCLUDED.email,
                    name = EXCLUDED.name,
                    updated_at = NOW()
            "#,
        )
        .bind(id)
        .bind(email)
        .bind(name)
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        self.get_by_id(id)
            .await?
            .ok_or_else(|| ApiError::internal("Failed to retrieve upserted user"))
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<User>> {
        let row = sqlx::query("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| ApiError::internal(e.to_string()))?;

        match row {
            Some(r) => Ok(Some(Self::map_row(&r)?)),
            None => Ok(None),
        }
    }

    async fn get_by_email(&self, email: &str) -> Result<Option<User>> {
        let row = sqlx::query("SELECT * FROM users WHERE LOWER(email) = LOWER($1)")
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| ApiError::internal(e.to_string()))?;

        match row {
            Some(r) => Ok(Some(Self::map_row(&r)?)),
            None => Ok(None),
        }
    }

    async fn search_by_email(&self, query: &str, limit: i32) -> Result<Vec<User>> {
        let pattern = format!("%{}%", query.replace('%', "\\%").replace('_', "\\_"));

        let rows = sqlx::query(
            r#"
            SELECT * FROM users
            WHERE email ILIKE $1
            ORDER BY email
            LIMIT $2
            "#,
        )
        .bind(pattern)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(rows.iter().filter_map(|r| Self::map_row(r).ok()).collect())
    }
}
