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

    async fn check_txt_record(&self, domain_name: &str, expected_value: &str) -> Result<(), VerificationReason> {
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
            Err(e) => {
                match e.kind() {
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
                }
            }
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
            Err(e) => {
                match e.kind() {
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
                }
            }
        }
    }

    async fn check_aaaa_records(&self, domain_name: &str) -> Result<(), VerificationReason> {
        // AAAA records are optional - only check if we have allowed IPv6 addresses configured
        if self.allowed_ipv6.is_empty() {
            debug!("No IPv6 addresses configured, skipping AAAA check for: {}", domain_name);
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
