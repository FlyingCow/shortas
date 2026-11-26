use anyhow::Result;
use dyn_clone::{clone_trait_object, DynClone};

use crate::model::Conversion;

#[async_trait::async_trait()]
pub trait ConversionStore: DynClone {
    /// Stores the conversion unless a newer version is already stored.
    /// Returns whether it was stored.
    async fn put_conversion(&self, conversion: &Conversion) -> Result<bool>;
    async fn get_conversion(&self, public_key: &str) -> Result<Option<Conversion>>;
}
clone_trait_object!(ConversionStore);

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::model::ConversionStep;

    pub(crate) fn conversion(version: i64) -> Conversion {
        Conversion {
            public_key: "pk_test".into(),
            conversion_id: "c".into(),
            workspace_id: "w".into(),
            version,
            active: true,
            secret_hash: "hash".into(),
            currency: "EUR".into(),
            window_days: 30,
            counting: "Every".into(),
            allowed_origins: vec![],
            server_values_only: false,
            steps: vec![ConversionStep {
                key: "sale".into(),
                order: 0,
                is_final: true,
                default_value: Some("5.25".into()),
            }],
            route_filter: None,
        }
    }

    pub(crate) async fn older_versions_never_replace_newer(store: &dyn ConversionStore) {
        assert_eq!(store.get_conversion("pk_test").await.unwrap(), None);
        assert!(store.put_conversion(&conversion(2)).await.unwrap());
        assert!(!store.put_conversion(&conversion(1)).await.unwrap());
        assert!(!store.put_conversion(&conversion(2)).await.unwrap());
        assert_eq!(store.get_conversion("pk_test").await.unwrap(), Some(conversion(2)));
        assert!(store.put_conversion(&conversion(10)).await.unwrap());
        assert_eq!(store.get_conversion("pk_test").await.unwrap(), Some(conversion(10)));

        let tombstone = Conversion::tombstone("pk_test".into());
        assert!(store.put_conversion(&tombstone).await.unwrap());
        assert!(!store.put_conversion(&conversion(11)).await.unwrap());
        assert_eq!(store.get_conversion("pk_test").await.unwrap(), Some(tombstone));
    }

    /// Concurrent deliveries in any order end with the newest version.
    pub(crate) async fn concurrent_writes_keep_the_newest(store: &(dyn ConversionStore + Sync)) {
        let versions = [13, 18, 11, 20, 15, 12, 19, 14, 17, 16].map(conversion);
        let writes = versions.iter().map(|c| store.put_conversion(c));
        for result in futures::future::join_all(writes).await {
            result.unwrap();
        }
        assert_eq!(store.get_conversion("pk_test").await.unwrap(), Some(conversion(20)));
    }
}
