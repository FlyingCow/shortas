use anyhow::Result;
use hickory_resolver::{
    config::{ResolverConfig, ResolverOpts},
    error::ResolveErrorKind,
    TokioAsyncResolver,
};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::time::Duration;
use tracing::{debug, info, warn};

use crate::model::{Domain, VerificationReason, VerificationStatus};
use crate::settings::DnsSettings;

const DEFAULT_DOMAIN: &str = "shortas.space";

#[derive(Clone)]
pub struct DnsVerifier {
    resolver: TokioAsyncResolver,
    txt_record_name: String,
    allowed_ipv4: Vec<Ipv4Addr>,
    allowed_ipv6: Vec<Ipv6Addr>,
}

#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub status: VerificationStatus,
    pub reason: VerificationReason,
}

impl DnsVerifier {
    pub fn new(settings: &DnsSettings) -> Result<Self> {
        let mut opts = ResolverOpts::default();
        opts.timeout = Duration::from_secs(5);
        opts.attempts = 2;

        let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), opts);

        let allowed_ipv4: Vec<Ipv4Addr> = settings
            .allowed_ipv4
            .iter()
            .filter_map(|s| s.parse().ok())
            .collect();

        let allowed_ipv6: Vec<Ipv6Addr> = settings
            .allowed_ipv6
            .iter()
            .filter_map(|s| s.parse().ok())
            .collect();

        info!(
            "DNS verifier initialized with {} allowed IPv4 and {} allowed IPv6 addresses",
            allowed_ipv4.len(),
            allowed_ipv6.len()
        );

        Ok(Self {
            resolver,
            txt_record_name: settings.txt_record_name.clone(),
            allowed_ipv4,
            allowed_ipv6,
        })
    }

    pub async fn verify(&self, domain: &Domain) -> VerificationResult {
        info!("Verifying domain: {} (id: {})", domain.name, domain.id);

        if DEFAULT_DOMAIN.eq_ignore_ascii_case(domain.name.as_str()) {
            return VerificationResult {
                status: VerificationStatus::Verified,
                reason: VerificationReason::TxtRecordValid,
            };
        }

        // Step 1: Check TXT record
        let txt_result = self.check_txt_record(&domain.name, &domain.id).await;
        if let Err(reason) = txt_result {
            return VerificationResult {
                status: VerificationStatus::Failed,
                reason,
            };
        }

        // Step 2: Check A records (required)
        let a_result = self.check_a_records(&domain.name).await;
        if let Err(reason) = a_result {
            return VerificationResult {
                status: VerificationStatus::Failed,
                reason,
            };
        }

        // Step 3: Check AAAA records (optional - only fail if present and invalid)
        let aaaa_result = self.check_aaaa_records(&domain.name).await;
        if let Err(reason) = aaaa_result {
            return VerificationResult {
                status: VerificationStatus::Failed,
                reason,
            };
        }

        VerificationResult {
            status: VerificationStatus::Verified,
            reason: VerificationReason::TxtRecordValid,
        }
    }

    async fn check_txt_record(
        &self,
        domain_name: &str,
        expected_value: &str,
    ) -> Result<(), VerificationReason> {
        let txt_domain = format!("{}.{}", self.txt_record_name, domain_name);
        debug!("Checking TXT record: {}", txt_domain);

        match self.resolver.txt_lookup(&txt_domain).await {
            Ok(response) => {
                for record in response.iter() {
                    let txt_data: String = record
                        .txt_data()
                        .iter()
                        .map(|bytes| String::from_utf8_lossy(bytes).to_string())
                        .collect();

                    debug!("Found TXT record: {}", txt_data);

                    if txt_data.trim() == expected_value {
                        info!("TXT record valid for domain: {}", domain_name);
                        return Ok(());
                    }
                }
                warn!(
                    "TXT record mismatch for domain {}: expected {}",
                    domain_name, expected_value
                );
                Err(VerificationReason::TxtRecordMismatch)
            }
            Err(e) => match e.kind() {
                ResolveErrorKind::NoRecordsFound { .. } => {
                    warn!("TXT record missing for domain: {}", domain_name);
                    Err(VerificationReason::TxtRecordMissing)
                }
                ResolveErrorKind::Timeout => {
                    warn!("DNS timeout for TXT record: {}", domain_name);
                    Err(VerificationReason::DnsTimeout)
                }
                _ => {
                    warn!("DNS error for TXT record {}: {}", domain_name, e);
                    Err(VerificationReason::DnsError(e.to_string()))
                }
            },
        }
    }

    async fn check_a_records(&self, domain_name: &str) -> Result<(), VerificationReason> {
        debug!("Checking A records for: {}", domain_name);

        match self.resolver.ipv4_lookup(domain_name).await {
            Ok(response) => {
                let ips: Vec<Ipv4Addr> = response.iter().map(|a| a.0).collect();

                if ips.is_empty() {
                    warn!("No A records found for domain: {}", domain_name);
                    return Err(VerificationReason::ARecordMissing);
                }

                for ip in &ips {
                    if !self.allowed_ipv4.contains(ip) {
                        warn!(
                            "Invalid A record for domain {}: {} not in allowed list",
                            domain_name, ip
                        );
                        return Err(VerificationReason::ARecordInvalid);
                    }
                }

                info!("A records valid for domain: {} ({:?})", domain_name, ips);
                Ok(())
            }
            Err(e) => match e.kind() {
                ResolveErrorKind::NoRecordsFound { .. } => {
                    warn!("A record missing for domain: {}", domain_name);
                    Err(VerificationReason::ARecordMissing)
                }
                ResolveErrorKind::Timeout => {
                    warn!("DNS timeout for A record: {}", domain_name);
                    Err(VerificationReason::DnsTimeout)
                }
                _ => {
                    warn!("DNS error for A record {}: {}", domain_name, e);
                    Err(VerificationReason::DnsError(e.to_string()))
                }
            },
        }
    }

    async fn check_aaaa_records(&self, domain_name: &str) -> Result<(), VerificationReason> {
        // AAAA records are optional - only check if we have allowed IPv6 addresses configured
        if self.allowed_ipv6.is_empty() {
            debug!(
                "No IPv6 addresses configured, skipping AAAA check for: {}",
                domain_name
            );
            return Ok(());
        }

        debug!("Checking AAAA records for: {}", domain_name);

        match self.resolver.ipv6_lookup(domain_name).await {
            Ok(response) => {
                let ips: Vec<Ipv6Addr> = response.iter().map(|aaaa| aaaa.0).collect();

                // If no AAAA records, that's fine
                if ips.is_empty() {
                    debug!("No AAAA records for domain (OK): {}", domain_name);
                    return Ok(());
                }

                // If AAAA records exist, they must be in the allowed list
                for ip in &ips {
                    if !self.allowed_ipv6.contains(ip) {
                        warn!(
                            "Invalid AAAA record for domain {}: {} not in allowed list",
                            domain_name, ip
                        );
                        return Err(VerificationReason::AaaaRecordInvalid);
                    }
                }

                info!("AAAA records valid for domain: {} ({:?})", domain_name, ips);
                Ok(())
            }
            Err(e) => {
                // No AAAA records is fine
                match e.kind() {
                    ResolveErrorKind::NoRecordsFound { .. } => {
                        debug!("No AAAA records for domain (OK): {}", domain_name);
                        Ok(())
                    }
                    ResolveErrorKind::Timeout => {
                        warn!("DNS timeout for AAAA record: {}", domain_name);
                        Err(VerificationReason::DnsTimeout)
                    }
                    _ => {
                        warn!("DNS error for AAAA record {}: {}", domain_name, e);
                        Err(VerificationReason::DnsError(e.to_string()))
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dns_settings(ipv4: Vec<&str>, ipv6: Vec<&str>) -> DnsSettings {
        DnsSettings {
            txt_record_name: "_shortas-domain-challenge".into(),
            allowed_ipv4: ipv4.into_iter().map(String::from).collect(),
            allowed_ipv6: ipv6.into_iter().map(String::from).collect(),
        }
    }

    #[test]
    fn test_dns_verifier_new_parses_valid_ipv4() {
        let settings = dns_settings(vec!["203.0.113.10", "1.2.3.4"], vec![]);
        let verifier = DnsVerifier::new(&settings).unwrap();

        assert_eq!(verifier.allowed_ipv4.len(), 2);
        assert_eq!(verifier.allowed_ipv4[0], Ipv4Addr::new(203, 0, 113, 10));
        assert_eq!(verifier.allowed_ipv4[1], Ipv4Addr::new(1, 2, 3, 4));
    }

    #[test]
    fn test_dns_verifier_new_skips_invalid_ipv4() {
        let settings = dns_settings(vec!["203.0.113.10", "not-an-ip", "1.2.3.4"], vec![]);
        let verifier = DnsVerifier::new(&settings).unwrap();

        assert_eq!(verifier.allowed_ipv4.len(), 2);
    }

    #[test]
    fn test_dns_verifier_new_parses_valid_ipv6() {
        let settings = dns_settings(vec![], vec!["::1", "2001:db8::1"]);
        let verifier = DnsVerifier::new(&settings).unwrap();

        assert_eq!(verifier.allowed_ipv6.len(), 2);
        assert_eq!(verifier.allowed_ipv6[0], Ipv6Addr::LOCALHOST);
    }

    #[test]
    fn test_dns_verifier_new_skips_invalid_ipv6() {
        let settings = dns_settings(vec![], vec!["::1", "not-ipv6"]);
        let verifier = DnsVerifier::new(&settings).unwrap();

        assert_eq!(verifier.allowed_ipv6.len(), 1);
    }

    #[test]
    fn test_dns_verifier_new_stores_txt_record_name() {
        let settings = dns_settings(vec![], vec![]);
        let verifier = DnsVerifier::new(&settings).unwrap();

        assert_eq!(verifier.txt_record_name, "_shortas-domain-challenge");
    }

    #[test]
    fn test_dns_verifier_new_empty_lists() {
        let settings = dns_settings(vec![], vec![]);
        let verifier = DnsVerifier::new(&settings).unwrap();

        assert!(verifier.allowed_ipv4.is_empty());
        assert!(verifier.allowed_ipv6.is_empty());
    }

    #[test]
    fn test_verification_result_fields() {
        let result = VerificationResult {
            status: VerificationStatus::Verified,
            reason: VerificationReason::TxtRecordValid,
        };

        assert_eq!(result.status, VerificationStatus::Verified);
        assert_eq!(result.reason, VerificationReason::TxtRecordValid);
    }

    #[test]
    fn test_verification_result_clone() {
        let result = VerificationResult {
            status: VerificationStatus::Failed,
            reason: VerificationReason::ARecordMissing,
        };

        let cloned = result.clone();
        assert_eq!(cloned.status, VerificationStatus::Failed);
        assert_eq!(cloned.reason, VerificationReason::ARecordMissing);
    }

    #[tokio::test]
    async fn test_verify_nonexistent_domain_fails() {
        let settings = dns_settings(vec!["203.0.113.10"], vec![]);
        let verifier = DnsVerifier::new(&settings).unwrap();

        let domain = Domain::new(
            "d1".into(),
            "this-domain-definitely-does-not-exist-xyz123.example".into(),
            "o1".into(),
        );

        let result = verifier.verify(&domain).await;
        assert_eq!(result.status, VerificationStatus::Failed);
    }
}
