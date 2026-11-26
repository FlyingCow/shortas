//! Shared domains named in the settings.

use tracing::{debug, info, warn};

use crate::domain::entities::{DomainVerificationStatus, OutboxMessage, Result, RouteDomain};
use crate::domain::traits::{DomainRepository, OutboxRepository};

/// Owner of the shared domains created here.
const SYSTEM_OWNER_ID: &str = "__system__";

/// Create the configured shared domains that don't exist yet, and have
/// domain-verifier report each of them as verified, which is what makes
/// Cert Bot issue their certificates. That message is retried until
/// domain-verifier is available, and not queued twice.
///
/// A name already used by a domain that isn't shared is left alone.
pub async fn seed_shared_domains(
    domains: &dyn DomainRepository,
    outbox: &dyn OutboxRepository,
    names: &[String],
) -> Result<()> {
    if names.is_empty() {
        info!("No shared domains configured");
        return Ok(());
    }
    info!("Seeding shared domains: {:?}", names);

    for name in names {
        let domain = match domains.get_by_name(name).await? {
            Some(existing) if !existing.is_shared => {
                warn!("Domain '{}' exists but is not marked as shared. Skipping.", existing.name);
                continue;
            }
            Some(existing) => {
                debug!("Shared domain '{}' already exists", existing.name);
                existing
            }
            None => {
                let created = domains
                    .create(&RouteDomain {
                        is_shared: true,
                        verification_status: DomainVerificationStatus::Verified,
                        verification_reason: "shared_domain".into(),
                        ..RouteDomain::new(name.clone(), SYSTEM_OWNER_ID.into())
                    })
                    .await?;
                info!("Created shared domain: {} (ID: {})", created.name, created.id);
                created
            }
        };
        outbox
            .create_unless_pending(&OutboxMessage::verify_shared_domain(domain.id))
            .await?;
    }

    info!("Shared domain seeding completed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::OutboxMessageType;
    use crate::domain::traits::{MockDomainRepository, MockOutboxRepository};
    use std::sync::{Arc, Mutex};
    use uuid::Uuid;

    fn shared(name: &str) -> RouteDomain {
        RouteDomain { is_shared: true, ..RouteDomain::new(name.into(), SYSTEM_OWNER_ID.into()) }
    }

    /// Runs the seeding over `existing` domains; returns the created domains
    /// and the domain IDs that verification was queued for.
    async fn seed(existing: Vec<RouteDomain>, names: &[&str]) -> (Vec<RouteDomain>, Vec<Uuid>) {
        let created = Arc::new(Mutex::new(Vec::new()));
        let queued = Arc::new(Mutex::new(Vec::new()));

        let mut domains = MockDomainRepository::new();
        domains.expect_get_by_name().returning(move |name| {
            Ok(existing.iter().find(|d| d.name.eq_ignore_ascii_case(name)).cloned())
        });
        let c = created.clone();
        domains.expect_create().returning(move |d| {
            c.lock().unwrap().push(d.clone());
            Ok(d.clone())
        });
        let mut outbox = MockOutboxRepository::new();
        let q = queued.clone();
        outbox.expect_create_unless_pending().returning(move |m| {
            assert_eq!(m.message_type, OutboxMessageType::VerifyDomain);
            assert_eq!(m.max_retries, i32::MAX);
            let id = m.payload["domain_id"].as_str().unwrap().parse().unwrap();
            q.lock().unwrap().push(id);
            Ok(true)
        });

        let names: Vec<String> = names.iter().map(|n| n.to_string()).collect();
        seed_shared_domains(&domains, &outbox, &names).await.unwrap();
        let created = created.lock().unwrap().clone();
        let queued = queued.lock().unwrap().clone();
        (created, queued)
    }

    #[tokio::test]
    async fn creates_missing_domains_verified_and_queues_them() {
        let (created, queued) = seed(vec![], &["Shortas.Space"]).await;
        assert_eq!(created.len(), 1);
        let d = &created[0];
        assert_eq!((d.name.as_str(), d.owner_id.as_str(), d.is_shared), ("shortas.space", SYSTEM_OWNER_ID, true));
        assert_eq!(d.verification_status, DomainVerificationStatus::Verified);
        assert_eq!(d.verification_reason, "shared_domain");
        assert_eq!(queued, vec![d.id]);
    }

    #[tokio::test]
    async fn queues_existing_shared_domains_without_creating() {
        let existing = shared("shortas.space");
        let (created, queued) = seed(vec![existing.clone()], &["SHORTAS.space"]).await;
        assert!(created.is_empty());
        assert_eq!(queued, vec![existing.id]);
    }

    #[tokio::test]
    async fn leaves_names_taken_by_unshared_domains_alone() {
        let taken = RouteDomain::new("shortas.space".into(), "someone".into());
        let (created, queued) = seed(vec![taken], &["shortas.space"]).await;
        assert!(created.is_empty());
        assert!(queued.is_empty());
    }

    #[tokio::test]
    async fn does_nothing_without_names() {
        let (created, queued) = seed(vec![], &[]).await;
        assert!(created.is_empty() && queued.is_empty());
    }
}
