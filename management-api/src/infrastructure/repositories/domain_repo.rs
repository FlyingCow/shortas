//! Domain repository implementation using SQLx.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::entities::{ApiError, DomainVerificationStatus, Result, RouteDomain};
use crate::domain::traits::DomainRepository;

/// PostgreSQL domain repository.
pub struct PgDomainRepository {
    pool: PgPool,
}

impl PgDomainRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn map_row(row: &sqlx::postgres::PgRow) -> Result<RouteDomain> {
        let id: Uuid = row.try_get("id").map_err(|e| ApiError::internal(e.to_string()))?;
        let name: String = row.try_get("name").unwrap_or_default();
        let owner_id: String = row.try_get("owner_id").unwrap_or_default();
        let is_shared: bool = row.try_get("is_shared").unwrap_or(false);
        let status_str: String = row
            .try_get("verification_status")
            .unwrap_or_else(|_| "Pending".to_string());
        let reason: String = row
            .try_get("verification_reason")
            .unwrap_or_else(|_| "not_checked".to_string());
        let last_check: Option<DateTime<Utc>> = row.try_get("last_verification_check").ok();
        let next_check: Option<DateTime<Utc>> = row.try_get("next_verification_check").ok();
        let custom_index: Option<String> = row.try_get("custom_index_url").ok();
        let custom_404: Option<String> = row.try_get("custom_not_found_url").ok();
        let created_at: Option<DateTime<Utc>> = row.try_get("created_at").ok();
        let updated_at: Option<DateTime<Utc>> = row.try_get("updated_at").ok();

        Ok(RouteDomain {
            id,
            name,
            owner_id,
            is_shared,
            verification_status: DomainVerificationStatus::from_str(&status_str),
            verification_reason: reason,
            last_verification_check: last_check,
            next_verification_check: next_check,
            custom_index_url: custom_index,
            custom_not_found_url: custom_404,
            dns_config: None,
            created_at,
            updated_at,
        })
    }
}

#[async_trait]
impl DomainRepository for PgDomainRepository {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<RouteDomain>> {
        let row = sqlx::query("SELECT * FROM route_domains WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| ApiError::internal(e.to_string()))?;

        match row {
            Some(r) => Ok(Some(Self::map_row(&r)?)),
            None => Ok(None),
        }
    }

    async fn get_by_name(&self, name: &str) -> Result<Option<RouteDomain>> {
        let row = sqlx::query("SELECT * FROM route_domains WHERE LOWER(name) = LOWER($1)")
            .bind(name)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| ApiError::internal(e.to_string()))?;

        match row {
            Some(r) => Ok(Some(Self::map_row(&r)?)),
            None => Ok(None),
        }
    }

    async fn list_by_owner(&self, owner_id: &str) -> Result<Vec<RouteDomain>> {
        let rows = sqlx::query("SELECT * FROM route_domains WHERE owner_id = $1 ORDER BY name")
            .bind(owner_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(rows.iter().filter_map(|r| Self::map_row(r).ok()).collect())
    }

    async fn list_shared(&self) -> Result<Vec<RouteDomain>> {
        let rows = sqlx::query("SELECT * FROM route_domains WHERE is_shared = true ORDER BY name")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(rows.iter().filter_map(|r| Self::map_row(r).ok()).collect())
    }

    async fn list_accessible(&self, user_id: &str) -> Result<Vec<RouteDomain>> {
        let rows = sqlx::query(
            "SELECT * FROM route_domains WHERE owner_id = $1 OR is_shared = true ORDER BY is_shared, name",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(rows.iter().filter_map(|r| Self::map_row(r).ok()).collect())
    }

    async fn create(&self, domain: &RouteDomain) -> Result<RouteDomain> {
        sqlx::query(
            r#"
            INSERT INTO route_domains (id, name, owner_id, is_shared, verification_status,
                verification_reason, custom_index_url, custom_not_found_url, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
        )
        .bind(domain.id)
        .bind(&domain.name)
        .bind(&domain.owner_id)
        .bind(domain.is_shared)
        .bind(domain.verification_status.as_str())
        .bind(&domain.verification_reason)
        .bind(&domain.custom_index_url)
        .bind(&domain.custom_not_found_url)
        .bind(Utc::now())
        .bind(Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        self.get_by_id(domain.id)
            .await?
            .ok_or_else(|| ApiError::internal("Failed to retrieve created domain"))
    }

    async fn update(&self, domain: &RouteDomain) -> Result<RouteDomain> {
        sqlx::query(
            r#"
            UPDATE route_domains
            SET is_shared = $2, verification_status = $3, verification_reason = $4,
                custom_index_url = $5, custom_not_found_url = $6, updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(domain.id)
        .bind(domain.is_shared)
        .bind(domain.verification_status.as_str())
        .bind(&domain.verification_reason)
        .bind(&domain.custom_index_url)
        .bind(&domain.custom_not_found_url)
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        self.get_by_id(domain.id)
            .await?
            .ok_or_else(|| ApiError::internal("Failed to retrieve updated domain"))
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM route_domains WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(())
    }

    async fn name_exists(&self, name: &str) -> Result<bool> {
        let row = sqlx::query(
            "SELECT EXISTS(SELECT 1 FROM route_domains WHERE LOWER(name) = LOWER($1)) as exists",
        )
        .bind(name)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(row.try_get::<bool, _>("exists").unwrap_or(false))
    }

    async fn get_pending_verification(&self, limit: i32) -> Result<Vec<RouteDomain>> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM route_domains
            WHERE verification_status = 'Pending'
              AND (next_verification_check IS NULL OR next_verification_check <= NOW())
            ORDER BY created_at
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(rows.iter().filter_map(|r| Self::map_row(r).ok()).collect())
    }

    async fn update_verification_status(&self, id: Uuid, status: &str, reason: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE route_domains
            SET verification_status = $2, verification_reason = $3,
                last_verification_check = NOW(), updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(status)
        .bind(reason)
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(())
    }
}
