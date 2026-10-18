//! Certificate repository implementation using SQLx.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::entities::{ApiError, Certificate, Result};
use crate::domain::traits::CertificateRepository;

/// PostgreSQL certificate repository.
pub struct PgCertificateRepository {
    pool: PgPool,
}

impl PgCertificateRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn map_row(row: &sqlx::postgres::PgRow) -> Result<Certificate> {
        let id: Uuid = row.try_get("id").map_err(|e| ApiError::internal(e.to_string()))?;
        let key: String = row.try_get("key").unwrap_or_default();
        let cert: String = row.try_get("cert").unwrap_or_default();
        let ocsp_resp: Option<String> = row.try_get("ocsp_resp").ok();
        let owner_id: String = row.try_get("owner_id").unwrap_or_default();
        let domain_id: Uuid = row.try_get("domain_id").unwrap_or_default();
        let expires_at: Option<DateTime<Utc>> = row.try_get("expires_at").ok();
        let created_at: Option<DateTime<Utc>> = row.try_get("created_at").ok();

        Ok(Certificate {
            id,
            key,
            cert,
            ocsp_resp,
            owner_id,
            domain_id,
            expires_at,
            created_at,
        })
    }
}

#[async_trait]
impl CertificateRepository for PgCertificateRepository {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Certificate>> {
        let row = sqlx::query("SELECT * FROM certificates WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| ApiError::internal(e.to_string()))?;

        match row {
            Some(r) => Ok(Some(Self::map_row(&r)?)),
            None => Ok(None),
        }
    }

    async fn get_by_domain(&self, domain_id: Uuid) -> Result<Option<Certificate>> {
        let row = sqlx::query("SELECT * FROM certificates WHERE domain_id = $1")
            .bind(domain_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| ApiError::internal(e.to_string()))?;

        match row {
            Some(r) => Ok(Some(Self::map_row(&r)?)),
            None => Ok(None),
        }
    }

    async fn list_by_owner(&self, owner_id: &str) -> Result<Vec<Certificate>> {
        let rows = sqlx::query("SELECT * FROM certificates WHERE owner_id = $1 ORDER BY created_at DESC")
            .bind(owner_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(rows.iter().filter_map(|r| Self::map_row(r).ok()).collect())
    }

    async fn create(&self, certificate: &Certificate) -> Result<Certificate> {
        sqlx::query(
            r#"
            INSERT INTO certificates (id, key, cert, ocsp_resp, owner_id, domain_id, expires_at, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(certificate.id)
        .bind(&certificate.key)
        .bind(&certificate.cert)
        .bind(&certificate.ocsp_resp)
        .bind(&certificate.owner_id)
        .bind(certificate.domain_id)
        .bind(certificate.expires_at)
        .bind(Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        self.get_by_id(certificate.id)
            .await?
            .ok_or_else(|| ApiError::internal("Failed to retrieve created certificate"))
    }

    async fn update(&self, certificate: &Certificate) -> Result<Certificate> {
        sqlx::query(
            r#"
            UPDATE certificates
            SET key = $2, cert = $3, ocsp_resp = $4, expires_at = $5
            WHERE id = $1
            "#,
        )
        .bind(certificate.id)
        .bind(&certificate.key)
        .bind(&certificate.cert)
        .bind(&certificate.ocsp_resp)
        .bind(certificate.expires_at)
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        self.get_by_id(certificate.id)
            .await?
            .ok_or_else(|| ApiError::internal("Failed to retrieve updated certificate"))
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM certificates WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(())
    }

    async fn delete_by_domain(&self, domain_id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM certificates WHERE domain_id = $1")
            .bind(domain_id)
            .execute(&self.pool)
            .await
            .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(())
    }

    async fn get_expiring(&self, days: i32) -> Result<Vec<Certificate>> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM certificates
            WHERE expires_at IS NOT NULL
              AND expires_at <= NOW() + ($1 || ' days')::interval
            ORDER BY expires_at
            "#,
        )
        .bind(days)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

        Ok(rows.iter().filter_map(|r| Self::map_row(r).ok()).collect())
    }
}
