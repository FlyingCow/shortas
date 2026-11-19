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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{VerificationReason, VerificationStatus};
    use std::collections::HashMap;
    use std::sync::Mutex;

    /// In-memory mock implementation of DomainStore for testing.
    #[derive(Clone, Default)]
    struct InMemoryDomainStore {
        domains: std::sync::Arc<Mutex<HashMap<String, Domain>>>,
    }

    #[async_trait::async_trait()]
    impl DomainStore for InMemoryDomainStore {
        async fn store_domain(&self, domain: &Domain) -> Result<()> {
            let mut map = self.domains.lock().unwrap();
            if map.contains_key(&domain.id) {
                anyhow::bail!("already exists");
            }
            map.insert(domain.id.clone(), domain.clone());
            Ok(())
        }

        async fn update_domain(&self, domain: &Domain) -> Result<()> {
            let mut map = self.domains.lock().unwrap();
            if !map.contains_key(&domain.id) {
                anyhow::bail!("not found");
            }
            map.insert(domain.id.clone(), domain.clone());
            Ok(())
        }

        async fn delete_domain(&self, id: &str) -> Result<()> {
            let mut map = self.domains.lock().unwrap();
            map.remove(id);
            Ok(())
        }

        async fn get_domain(&self, id: &str) -> Result<Option<Domain>> {
            let map = self.domains.lock().unwrap();
            Ok(map.get(id).cloned())
        }

        async fn get_domain_by_name(&self, name: &str, owner_id: &str) -> Result<Option<Domain>> {
            let map = self.domains.lock().unwrap();
            Ok(map
                .values()
                .find(|d| d.name == name && d.owner_id == owner_id)
                .cloned())
        }

        async fn list_domains(
            &self,
            owner_id: Option<&str>,
            page: u32,
            page_size: u32,
        ) -> Result<(Vec<Domain>, u64)> {
            let map = self.domains.lock().unwrap();
            let mut domains: Vec<Domain> = match owner_id {
                Some(oid) => map.values().filter(|d| d.owner_id == oid).cloned().collect(),
                None => map.values().cloned().collect(),
            };
            domains.sort_by(|a, b| a.name.cmp(&b.name));
            let total = domains.len() as u64;
            let skip = ((page - 1) * page_size) as usize;
            let page_domains = domains.into_iter().skip(skip).take(page_size as usize).collect();
            Ok((page_domains, total))
        }

        async fn get_domains_for_verification(
            &self,
            before: DateTime<Utc>,
            limit: usize,
        ) -> Result<Vec<Domain>> {
            let map = self.domains.lock().unwrap();
            let mut domains: Vec<Domain> = map
                .values()
                .filter(|d| match d.next_check_at {
                    Some(t) => t <= before,
                    None => true,
                })
                .cloned()
                .collect();
            domains.truncate(limit);
            Ok(domains)
        }
    }

    #[tokio::test]
    async fn test_store_and_get_domain() {
        let store = InMemoryDomainStore::default();
        let domain = Domain::new("d1".into(), "example.com".into(), "o1".into());

        store.store_domain(&domain).await.unwrap();

        let retrieved = store.get_domain("d1").await.unwrap();
        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.id, "d1");
        assert_eq!(retrieved.name, "example.com");
    }

    #[tokio::test]
    async fn test_get_nonexistent_domain_returns_none() {
        let store = InMemoryDomainStore::default();
        let result = store.get_domain("nonexistent").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_store_duplicate_fails() {
        let store = InMemoryDomainStore::default();
        let domain = Domain::new("d1".into(), "example.com".into(), "o1".into());

        store.store_domain(&domain).await.unwrap();
        let result = store.store_domain(&domain).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_domain() {
        let store = InMemoryDomainStore::default();
        let mut domain = Domain::new("d1".into(), "example.com".into(), "o1".into());
        store.store_domain(&domain).await.unwrap();

        domain.status = VerificationStatus::Verified;
        domain.verification_reason = VerificationReason::TxtRecordValid;
        store.update_domain(&domain).await.unwrap();

        let updated = store.get_domain("d1").await.unwrap().unwrap();
        assert_eq!(updated.status, VerificationStatus::Verified);
        assert_eq!(updated.verification_reason, VerificationReason::TxtRecordValid);
    }

    #[tokio::test]
    async fn test_update_nonexistent_fails() {
        let store = InMemoryDomainStore::default();
        let domain = Domain::new("d1".into(), "example.com".into(), "o1".into());
        let result = store.update_domain(&domain).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_domain() {
        let store = InMemoryDomainStore::default();
        let domain = Domain::new("d1".into(), "example.com".into(), "o1".into());
        store.store_domain(&domain).await.unwrap();

        store.delete_domain("d1").await.unwrap();

        let result = store.get_domain("d1").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_get_domain_by_name() {
        let store = InMemoryDomainStore::default();
        let domain = Domain::new("d1".into(), "example.com".into(), "o1".into());
        store.store_domain(&domain).await.unwrap();

        let found = store.get_domain_by_name("example.com", "o1").await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, "d1");

        // Different owner
        let not_found = store.get_domain_by_name("example.com", "other").await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    async fn test_list_domains_all() {
        let store = InMemoryDomainStore::default();
        store.store_domain(&Domain::new("d1".into(), "aaa.com".into(), "o1".into())).await.unwrap();
        store.store_domain(&Domain::new("d2".into(), "bbb.com".into(), "o1".into())).await.unwrap();
        store.store_domain(&Domain::new("d3".into(), "ccc.com".into(), "o2".into())).await.unwrap();

        let (domains, total) = store.list_domains(None, 1, 10).await.unwrap();
        assert_eq!(total, 3);
        assert_eq!(domains.len(), 3);
    }

    #[tokio::test]
    async fn test_list_domains_by_owner() {
        let store = InMemoryDomainStore::default();
        store.store_domain(&Domain::new("d1".into(), "aaa.com".into(), "o1".into())).await.unwrap();
        store.store_domain(&Domain::new("d2".into(), "bbb.com".into(), "o2".into())).await.unwrap();

        let (domains, total) = store.list_domains(Some("o1"), 1, 10).await.unwrap();
        assert_eq!(total, 1);
        assert_eq!(domains[0].owner_id, "o1");
    }

    #[tokio::test]
    async fn test_list_domains_pagination() {
        let store = InMemoryDomainStore::default();
        for i in 0..5 {
            store
                .store_domain(&Domain::new(
                    format!("d{}", i),
                    format!("{:03}.com", i),
                    "o1".into(),
                ))
                .await
                .unwrap();
        }

        let (page1, total) = store.list_domains(None, 1, 2).await.unwrap();
        assert_eq!(total, 5);
        assert_eq!(page1.len(), 2);

        let (page2, _) = store.list_domains(None, 2, 2).await.unwrap();
        assert_eq!(page2.len(), 2);

        let (page3, _) = store.list_domains(None, 3, 2).await.unwrap();
        assert_eq!(page3.len(), 1);
    }

    #[tokio::test]
    async fn test_get_domains_for_verification() {
        let store = InMemoryDomainStore::default();
        let now = Utc::now();

        // Domain due for verification (next_check_at in the past)
        let mut d1 = Domain::new("d1".into(), "due.com".into(), "o1".into());
        d1.next_check_at = Some(now - chrono::Duration::minutes(5));
        store.store_domain(&d1).await.unwrap();

        // Domain not yet due (next_check_at in the future)
        let mut d2 = Domain::new("d2".into(), "notdue.com".into(), "o1".into());
        d2.next_check_at = Some(now + chrono::Duration::minutes(30));
        store.store_domain(&d2).await.unwrap();

        // Domain with no next_check_at (should be included)
        let mut d3 = Domain::new("d3".into(), "nonext.com".into(), "o1".into());
        d3.next_check_at = None;
        store.store_domain(&d3).await.unwrap();

        let due = store.get_domains_for_verification(now, 100).await.unwrap();
        let due_ids: Vec<&str> = due.iter().map(|d| d.id.as_str()).collect();

        assert!(due_ids.contains(&"d1"));
        assert!(!due_ids.contains(&"d2"));
        assert!(due_ids.contains(&"d3"));
    }

    #[tokio::test]
    async fn test_get_domains_for_verification_respects_limit() {
        let store = InMemoryDomainStore::default();
        let now = Utc::now();

        for i in 0..10 {
            let mut d = Domain::new(format!("d{}", i), format!("{}.com", i), "o1".into());
            d.next_check_at = Some(now - chrono::Duration::minutes(1));
            store.store_domain(&d).await.unwrap();
        }

        let due = store.get_domains_for_verification(now, 3).await.unwrap();
        assert_eq!(due.len(), 3);
    }

    #[test]
    fn test_domain_store_clone() {
        let store = InMemoryDomainStore::default();
        let boxed: Box<dyn DomainStore> = Box::new(store);
        let _cloned = boxed.clone();
    }
}
