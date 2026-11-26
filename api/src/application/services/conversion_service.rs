//! Conversion definitions: access rules, validation and lifecycle.

use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

use super::CoverageSignal;
use crate::domain::entities::{
    ApiError, Conversion, ConversionDefinition, ConversionStatus, ErrorCode, OptionExt, Result,
    RouteDomain,
};
use crate::domain::traits::{
    CertificateLookup, ConversionRepository, DomainRepository, WorkspaceRepository,
};

pub struct ConversionService {
    conversions: Arc<dyn ConversionRepository>,
    workspaces: Arc<dyn WorkspaceRepository>,
    domains: Arc<dyn DomainRepository>,
    certificates: Arc<dyn CertificateLookup>,
    coverage: CoverageSignal,
}

impl ConversionService {
    pub fn new(
        conversions: Arc<dyn ConversionRepository>,
        workspaces: Arc<dyn WorkspaceRepository>,
        domains: Arc<dyn DomainRepository>,
        certificates: Arc<dyn CertificateLookup>,
        coverage: CoverageSignal,
    ) -> Self {
        Self {
            conversions,
            workspaces,
            domains,
            certificates,
            coverage,
        }
    }

    pub async fn list(&self, workspace_id: Uuid, user_id: &str) -> Result<Vec<Conversion>> {
        self.require_member(workspace_id, user_id).await?;
        self.conversions.list(workspace_id).await
    }

    pub async fn get(&self, workspace_id: Uuid, id: Uuid, user_id: &str) -> Result<Conversion> {
        self.require_member(workspace_id, user_id).await?;
        self.find(workspace_id, id).await
    }

    /// Domains to choose from, for Admins, and those in use, for Members.
    pub async fn domains(&self, workspace_id: Uuid, user_id: &str) -> Result<Vec<RouteDomain>> {
        let is_admin = self.require_member(workspace_id, user_id).await?;
        let used: Vec<Uuid> = self
            .conversions
            .list(workspace_id)
            .await?
            .iter()
            .filter_map(|c| c.definition.domain_id)
            .collect();
        let mut domains = self.domains.get_by_ids(&used).await?;
        if is_admin {
            if let Some(owner) = self.workspaces.get_owner_user_id(workspace_id).await? {
                domains.extend(self.domains.list_by_owner(&owner).await?);
            }
            domains.extend(self.domains.list_shared().await?);
        }
        domains.sort_by(|a, b| (&a.name, a.id).cmp(&(&b.name, b.id)));
        domains.dedup_by_key(|d| d.id);
        Ok(domains)
    }

    /// Activating and active conversions that removing the domain would stop.
    pub async fn list_on_domain(&self, domain_id: Uuid, user_id: &str) -> Result<Vec<Conversion>> {
        let domain = self
            .domains
            .get_by_id(domain_id)
            .await?
            .filter(|d| d.owner_id == user_id)
            .ok_or_not_found("Domain", &domain_id.to_string())?;
        self.conversions.list_active_by_domain(domain.id).await
    }

    /// Creates a draft and returns it with its secret key.
    pub async fn create(
        &self,
        workspace_id: Uuid,
        definition: ConversionDefinition,
        user_id: &str,
    ) -> Result<(Conversion, String)> {
        self.require_admin(workspace_id, user_id).await?;
        let definition = self.check(workspace_id, definition, None).await?;
        let (conversion, secret) = Conversion::new(workspace_id, definition, user_id.to_string());
        self.conversions.create(&conversion).await?;
        Ok((conversion, secret))
    }

    /// Replaces the definition. `version` is the one the caller edited.
    pub async fn update(
        &self,
        workspace_id: Uuid,
        id: Uuid,
        definition: ConversionDefinition,
        version: i64,
        user_id: &str,
    ) -> Result<Conversion> {
        self.require_admin(workspace_id, user_id).await?;
        let mut conversion = self.find(workspace_id, id).await?;
        require_not_archived(&conversion)?;
        let definition = self.check(workspace_id, definition, Some(&conversion)).await?;

        let domain_changed = definition.domain_id != conversion.definition.domain_id;
        conversion.redefine(definition);
        if conversion.status.covers_routes() && domain_changed {
            self.require_eligible_domain(&conversion).await?;
        }
        conversion.version = version;
        self.save_covering(&conversion).await
    }

    pub async fn activate(&self, workspace_id: Uuid, id: Uuid, user_id: &str) -> Result<Conversion> {
        self.require_admin(workspace_id, user_id).await?;
        let mut conversion = self.find(workspace_id, id).await?;
        conversion.transition(ConversionStatus::Activating)?;
        self.require_eligible_domain(&conversion).await?;
        self.save_covering(&conversion).await
    }

    pub async fn pause(&self, workspace_id: Uuid, id: Uuid, user_id: &str) -> Result<Conversion> {
        self.change_status(workspace_id, id, ConversionStatus::Paused, user_id).await
    }

    pub async fn archive(&self, workspace_id: Uuid, id: Uuid, user_id: &str) -> Result<Conversion> {
        self.change_status(workspace_id, id, ConversionStatus::Archived, user_id).await
    }

    /// Replaces the secret key and returns the new one.
    pub async fn rotate_secret(
        &self,
        workspace_id: Uuid,
        id: Uuid,
        user_id: &str,
    ) -> Result<(Conversion, String)> {
        self.require_admin(workspace_id, user_id).await?;
        let mut conversion = self.find(workspace_id, id).await?;
        require_not_archived(&conversion)?;
        let secret = conversion.rotate_secret();
        Ok((self.conversions.save(&conversion).await?, secret))
    }

    /// Saves a change that may alter which routes are covered, and has the
    /// coverage sent.
    async fn save_covering(&self, conversion: &Conversion) -> Result<Conversion> {
        let saved = self.conversions.save(conversion).await?;
        self.coverage.notify();
        Ok(saved)
    }

    async fn change_status(
        &self,
        workspace_id: Uuid,
        id: Uuid,
        status: ConversionStatus,
        user_id: &str,
    ) -> Result<Conversion> {
        self.require_admin(workspace_id, user_id).await?;
        let mut conversion = self.find(workspace_id, id).await?;
        conversion.transition(status)?;
        self.save_covering(&conversion).await
    }

    async fn find(&self, workspace_id: Uuid, id: Uuid) -> Result<Conversion> {
        self.conversions
            .get(workspace_id, id)
            .await?
            .ok_or_not_found("Conversion", &id.to_string())
    }

    /// Outsiders get "not found", so they can't tell what exists.
    async fn require_member(&self, workspace_id: Uuid, user_id: &str) -> Result<bool> {
        let membership = self
            .workspaces
            .get_membership(workspace_id, user_id)
            .await?
            .ok_or_not_found("Workspace", &workspace_id.to_string())?;
        Ok(membership.is_admin())
    }

    async fn require_admin(&self, workspace_id: Uuid, user_id: &str) -> Result<()> {
        if self.require_member(workspace_id, user_id).await? {
            Ok(())
        } else {
            Err(ApiError::forbidden())
        }
    }

    /// Validates a definition, including what it refers to. Domains and routes
    /// it already had are accepted, so an unrelated edit still works after one
    /// of them was moved or lost.
    async fn check(
        &self,
        workspace_id: Uuid,
        mut definition: ConversionDefinition,
        existing: Option<&Conversion>,
    ) -> Result<ConversionDefinition> {
        if let Some(routes) = &mut definition.route_filter {
            routes.sort();
            routes.dedup();
        }
        definition.validate()?;
        let existing = existing.map(|c| &c.definition);

        if let Some(domain_id) = definition.domain_id {
            if existing.and_then(|d| d.domain_id) != Some(domain_id)
                && !self.owner_can_use(workspace_id, domain_id).await?
            {
                return Err(invalid("domainId"));
            }
        }

        if let Some(routes) = &definition.route_filter {
            let known = existing.and_then(|d| d.route_filter.as_deref()).unwrap_or(&[]);
            let added: Vec<Uuid> = routes.iter().filter(|r| !known.contains(r)).copied().collect();
            if !added.is_empty()
                && self.conversions.count_routes(workspace_id, &added).await? != added.len() as i64
            {
                return Err(invalid("routeIds"));
            }
        }

        Ok(definition)
    }

    async fn owner_can_use(&self, workspace_id: Uuid, domain_id: Uuid) -> Result<bool> {
        let Some(domain) = self.domains.get_by_id(domain_id).await? else {
            return Ok(false);
        };
        let owner = self.workspaces.get_owner_user_id(workspace_id).await?;
        Ok(domain.is_shared || owner.is_some_and(|owner| domain.can_use(&owner)))
    }

    async fn require_eligible_domain(&self, conversion: &Conversion) -> Result<()> {
        let refuse = |condition: &str, message: &str| {
            Err(ApiError::with_details(
                ErrorCode::UnprocessableEntity,
                message,
                condition,
            ))
        };

        let Some(domain_id) = conversion.definition.domain_id else {
            return refuse("domain_missing", "Choose a short domain before activating");
        };
        let Some(domain) = self.domains.get_by_id(domain_id).await? else {
            return refuse("domain_missing", "The short domain no longer exists");
        };
        if !self.owner_can_use(conversion.workspace_id, domain_id).await? {
            return refuse(
                "domain_not_owned",
                "The short domain must be shared or belong to the workspace owner",
            );
        }
        if !domain.is_verified() {
            return refuse("domain_unverified", "The short domain is not verified");
        }
        let expiry = self
            .certificates
            .certificate_expiry(&domain.name)
            .await
            .map_err(|e| {
                tracing::error!("Looking up the certificate of {} failed: {e:#}", domain.name);
                ApiError::external_service("The short domain's certificate could not be checked. Try again")
            })?;
        if expiry.is_none_or(|expiry| expiry <= Utc::now()) {
            return refuse(
                "certificate_missing",
                "The short domain has no valid certificate",
            );
        }
        Ok(())
    }
}

fn invalid(field: &str) -> ApiError {
    ApiError::with_details(
        ErrorCode::ValidationError,
        format!("Invalid fields: {field}"),
        field,
    )
}

fn require_not_archived(conversion: &Conversion) -> Result<()> {
    if conversion.status == ConversionStatus::Archived {
        return Err(ApiError::conflict("An archived conversion cannot be changed"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::conversion::tests::definition;
    use crate::domain::entities::{DomainVerificationStatus, UserWorkspace};
    use crate::domain::traits::{
        MockCertificateLookup, MockConversionRepository, MockDomainRepository,
        MockWorkspaceRepository,
    };
    use chrono::Duration;

    /// Users are named after their role; anyone else is not a member.
    struct Setup {
        existing: Option<Conversion>,
        domain: Option<RouteDomain>,
        certificate_days: Option<i64>,
        known_routes: i64,
    }

    fn eligible_domain() -> RouteDomain {
        RouteDomain {
            verification_status: DomainVerificationStatus::Verified,
            ..RouteDomain::new("go.example".into(), "Owner".into())
        }
    }

    fn draft() -> Conversion {
        let d = ConversionDefinition { domain_id: Some(Uuid::new_v4()), ..definition() };
        Conversion::new(Uuid::new_v4(), d, "Owner".into()).0
    }

    impl Default for Setup {
        fn default() -> Self {
            Self {
                existing: Some(draft()),
                domain: Some(eligible_domain()),
                certificate_days: Some(30),
                known_routes: 0,
            }
        }
    }

    impl Setup {
        fn build(self) -> ConversionService {
            let mut workspaces = MockWorkspaceRepository::new();
            workspaces.expect_get_membership().returning(|w, user| {
                Ok(["Owner", "Admin", "Member"]
                    .contains(&user)
                    .then(|| UserWorkspace::new(user.into(), w, user.into())))
            });
            workspaces.expect_get_owner_user_id().returning(|_| Ok(Some("Owner".into())));
            let mut conversions = MockConversionRepository::new();
            let existing = self.existing;
            conversions.expect_get().returning(move |_, _| Ok(existing.clone()));
            conversions.expect_list().returning(|_| Ok(vec![]));
            conversions.expect_create().returning(|_| Ok(()));
            conversions.expect_save().returning(|c| Ok(c.clone()));
            let known = self.known_routes;
            conversions.expect_count_routes().returning(move |_, _| Ok(known));
            let mut domains = MockDomainRepository::new();
            let domain = self.domain;
            domains.expect_get_by_id().returning(move |_| Ok(domain.clone()));
            let own = RouteDomain::new("own.example".into(), "Owner".into());
            domains.expect_list_by_owner().returning(move |owner| {
                assert_eq!(owner, "Owner");
                Ok(vec![own.clone()])
            });
            domains.expect_list_shared().returning(|| Ok(vec![eligible_domain()]));
            domains.expect_get_by_ids().returning(|_| Ok(vec![]));
            conversions.expect_list_active_by_domain().returning(|_| Ok(vec![draft()]));
            let mut certificates = MockCertificateLookup::new();
            let days = self.certificate_days;
            certificates
                .expect_certificate_expiry()
                .returning(move |_| Ok(days.map(|d| Utc::now() + Duration::days(d))));
            ConversionService::new(
                Arc::new(conversions),
                Arc::new(workspaces),
                Arc::new(domains),
                Arc::new(certificates),
                CoverageSignal::default(),
            )
        }
    }

    #[tokio::test]
    async fn roles_decide_who_may_read_and_change() {
        let (w, id) = (Uuid::new_v4(), Uuid::new_v4());
        for (user, read, write) in [
            ("Owner", None, None),
            ("Admin", None, None),
            ("Member", None, Some(ErrorCode::Forbidden)),
            ("Outsider", Some(ErrorCode::NotFound), Some(ErrorCode::NotFound)),
        ] {
            let s = Setup::default().build();
            let reads = [
                s.list(w, user).await.err(),
                s.get(w, id, user).await.err(),
            ];
            for e in reads {
                assert_eq!(e.map(|e| e.code), read, "{user}");
            }
            let writes = [
                s.create(w, definition(), user).await.err(),
                s.update(w, id, definition(), 1, user).await.err(),
                s.activate(w, id, user).await.err(),
                s.archive(w, id, user).await.err(),
                s.rotate_secret(w, id, user).await.err(),
            ];
            for e in writes {
                assert_eq!(e.map(|e| e.code), write, "{user}");
            }
            let mut active = draft();
            active.status = ConversionStatus::Active;
            let s = Setup { existing: Some(active), ..Default::default() }.build();
            assert_eq!(s.pause(w, id, user).await.err().map(|e| e.code), write, "{user}");
        }
    }

    #[tokio::test]
    async fn only_admins_see_domains_that_are_not_in_use() {
        let w = Uuid::new_v4();
        for user in ["Owner", "Admin"] {
            assert_eq!(Setup::default().build().domains(w, user).await.unwrap().len(), 2);
        }
        assert_eq!(Setup::default().build().domains(w, "Member").await.unwrap().len(), 0);
        let e = Setup::default().build().domains(w, "Outsider").await.unwrap_err();
        assert_eq!(e.code, ErrorCode::NotFound);
    }

    #[tokio::test]
    async fn only_the_domain_owner_sees_its_conversions() {
        let id = Uuid::new_v4();
        let s = Setup::default().build();
        assert_eq!(s.list_on_domain(id, "Owner").await.unwrap().len(), 1);
        let e = s.list_on_domain(id, "Admin").await.unwrap_err();
        assert_eq!(e.code, ErrorCode::NotFound);
    }

    #[tokio::test]
    async fn activation_states_the_missing_condition() {
        let cases: Vec<(&str, Setup)> = vec![
            ("domain_missing", Setup {
                existing: Some(Conversion::new(Uuid::new_v4(), definition(), "Owner".into()).0),
                ..Default::default()
            }),
            ("domain_missing", Setup { domain: None, ..Default::default() }),
            ("domain_not_owned", Setup {
                domain: Some(RouteDomain { owner_id: "Admin".into(), ..eligible_domain() }),
                ..Default::default()
            }),
            ("domain_unverified", Setup {
                domain: Some(RouteDomain {
                    verification_status: DomainVerificationStatus::Pending,
                    ..eligible_domain()
                }),
                ..Default::default()
            }),
            ("certificate_missing", Setup { certificate_days: None, ..Default::default() }),
            ("certificate_missing", Setup { certificate_days: Some(-1), ..Default::default() }),
        ];
        for (condition, setup) in cases {
            let e = setup.build().activate(Uuid::new_v4(), Uuid::new_v4(), "Admin").await.unwrap_err();
            assert_eq!(e.code, ErrorCode::UnprocessableEntity);
            assert_eq!(e.details.as_deref(), Some(condition));
        }

        let shared = RouteDomain { is_shared: true, owner_id: "__system__".into(), ..eligible_domain() };
        let s = Setup { domain: Some(shared), ..Default::default() }.build();
        let c = s.activate(Uuid::new_v4(), Uuid::new_v4(), "Admin").await.unwrap();
        assert_eq!(c.status, ConversionStatus::Activating);
    }

    #[tokio::test]
    async fn definitions_cannot_refer_to_foreign_domains_or_routes() {
        let d = ConversionDefinition { domain_id: Some(Uuid::new_v4()), ..definition() };
        let foreign = RouteDomain { owner_id: "stranger".into(), ..eligible_domain() };
        let s = Setup { domain: Some(foreign), ..Default::default() }.build();
        let e = s.create(Uuid::new_v4(), d, "Owner").await.unwrap_err();
        assert_eq!(e.details.as_deref(), Some("domainId"));

        let id = Uuid::new_v4();
        let d = ConversionDefinition { route_filter: Some(vec![id, id]), ..definition() };
        let e = Setup::default().build().create(Uuid::new_v4(), d.clone(), "Owner").await.unwrap_err();
        assert_eq!(e.details.as_deref(), Some("routeIds"));
        let s = Setup { known_routes: 1, ..Default::default() }.build();
        let (c, _) = s.create(Uuid::new_v4(), d, "Owner").await.unwrap();
        assert_eq!(c.definition.route_filter, Some(vec![id]));
    }

    #[tokio::test]
    async fn running_conversion_cannot_move_to_an_ineligible_domain() {
        let mut active = draft();
        active.status = ConversionStatus::Active;
        let d = ConversionDefinition { domain_id: Some(Uuid::new_v4()), ..definition() };
        let s = Setup { existing: Some(active), certificate_days: None, ..Default::default() }.build();
        let e = s.update(Uuid::new_v4(), Uuid::new_v4(), d, 1, "Owner").await.unwrap_err();
        assert_eq!(e.details.as_deref(), Some("certificate_missing"));
    }

    #[tokio::test]
    async fn archived_conversion_cannot_be_changed() {
        let mut archived = draft();
        archived.status = ConversionStatus::Archived;
        let s = Setup { existing: Some(archived), ..Default::default() }.build();
        let (w, id) = (Uuid::new_v4(), Uuid::new_v4());
        for e in [
            s.update(w, id, definition(), 1, "Owner").await.unwrap_err(),
            s.rotate_secret(w, id, "Owner").await.unwrap_err(),
            s.activate(w, id, "Owner").await.unwrap_err(),
        ] {
            assert_eq!(e.code, ErrorCode::Conflict);
        }
    }
}
