//! The address a request came from, believing only configured proxies.

use std::net::{IpAddr, Ipv6Addr};

use http::HeaderMap;
use ipnet::IpNet;

const FORWARDED_FOR: &str = "x-forwarded-for";

#[derive(Debug, Clone, Default)]
pub struct TrustedProxies(Vec<IpNet>);

impl TrustedProxies {
    /// Networks such as `10.0.0.0/8`, or single addresses.
    pub fn parse(networks: &[String]) -> anyhow::Result<Self> {
        networks
            .iter()
            .map(|n| {
                n.parse::<IpNet>()
                    .or_else(|_| n.parse::<IpAddr>().map(IpNet::from))
                    .map_err(|_| anyhow::anyhow!("invalid trusted proxy: {n}"))
            })
            .collect::<anyhow::Result<_>>()
            .map(Self)
    }

    fn contains(&self, address: IpAddr) -> bool {
        self.0.iter().any(|network| network.contains(&address))
    }

    /// The last forwarded address not added by a trusted proxy. Clients can
    /// prepend anything, so earlier ones mean nothing.
    pub fn client_address(&self, peer: IpAddr, headers: &HeaderMap) -> IpAddr {
        let mut address = peer.to_canonical();
        let values: Vec<&str> = headers
            .get_all(FORWARDED_FOR)
            .iter()
            .map(|v| v.to_str().unwrap_or_default())
            .collect();
        for value in values.iter().rev().flat_map(|v| v.rsplit(',')) {
            if !self.contains(address) {
                break;
            }
            match value.trim().parse::<IpAddr>() {
                Ok(forwarded) => address = forwarded.to_canonical(),
                Err(_) => break,
            }
        }
        address
    }
}

/// One subscriber usually has a whole IPv6 /64.
pub fn limit_key(address: IpAddr) -> IpAddr {
    match address {
        IpAddr::V4(_) => address,
        IpAddr::V6(v6) => Ipv6Addr::from_bits(v6.to_bits() & (!0 << 64)).into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn client(trusted: &[&str], peer: &str, forwarded: &[&str]) -> String {
        let trusted: Vec<_> = trusted.iter().map(|t| t.to_string()).collect();
        let mut headers = HeaderMap::new();
        for value in forwarded {
            headers.append(FORWARDED_FOR, value.parse().unwrap());
        }
        TrustedProxies::parse(&trusted)
            .unwrap()
            .client_address(peer.parse().unwrap(), &headers)
            .to_string()
    }

    #[test]
    fn only_trusted_proxies_are_believed() {
        assert_eq!(client(&[], "1.1.1.1", &["9.9.9.9"]), "1.1.1.1");
        assert_eq!(client(&["10.0.0.0/8"], "1.1.1.1", &["9.9.9.9"]), "1.1.1.1");
        assert_eq!(client(&["10.0.0.0/8"], "10.0.0.1", &["9.9.9.9"]), "9.9.9.9");
        assert_eq!(client(&["10.0.0.0/8"], "::ffff:10.0.0.1", &["9.9.9.9"]), "9.9.9.9");
        assert_eq!(client(&["::1"], "::1", &["9.9.9.9"]), "9.9.9.9");
    }

    #[test]
    fn forged_entries_before_the_real_one_are_ignored() {
        assert_eq!(client(&["10.0.0.0/8"], "10.0.0.1", &["6.6.6.6, 9.9.9.9"]), "9.9.9.9");
        assert_eq!(client(&["10.0.0.0/8"], "10.0.0.1", &["6.6.6.6", "9.9.9.9, 10.0.0.2"]), "9.9.9.9");
        assert_eq!(client(&["10.0.0.0/8"], "10.0.0.1", &["9.9.9.9, junk"]), "10.0.0.1");
    }

    #[test]
    fn rejects_bad_settings_and_groups_ipv6() {
        assert!(TrustedProxies::parse(&["nope".into()]).is_err());
        let key = |a: &str| limit_key(a.parse().unwrap());
        assert_eq!(key("2001:db8::1"), key("2001:db8::ffff:2"));
        assert_ne!(key("2001:db8::1"), key("2001:db8:0:1::1"));
        assert_ne!(key("1.1.1.1"), key("1.1.1.2"));
    }
}
