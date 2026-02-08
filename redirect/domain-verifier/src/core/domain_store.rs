use anyhow::Result;
use chrono::{DateTime, Utc};
use dyn_clone::{clone_trait_object, DynClone};

use crate::model::Domain;

#[async_trait::async_trait()]
pub trait DomainStore: DynClone {
    async fn store_domain(&self, domain: &Domain) -> Result<()>;
    async fn update_domain(&self, domain: &Domain) -> Result<()>;
    async fn delete_domain(&self, id: &str) -> Result<()>;
    async fn get_domain(&self, id: &str) -> Result<Option<Domain>>;
    async fn get_domain_by_name(&self, name: &str, owner_id: &str) -> Result<Option<Domain>>;
    async fn list_domains(
        &self,
        owner_id: Option<&str>,
        page: u32,
        page_size: u32,
    ) -> Result<(Vec<Domain>, u64)>;
    async fn get_domains_for_verification(
        &self,
        before: DateTime<Utc>,
        limit: usize,
    ) -> Result<Vec<Domain>>;
}

clone_trait_object!(DomainStore);
