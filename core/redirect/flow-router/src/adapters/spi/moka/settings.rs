use serde_derive::Deserialize;


#[derive(Debug, Deserialize)]
#[allow(unused)]
struct CryptoCache {
    max_capacity: u64, 
    time_to_live_minutes: u64, 
    time_to_idle_minutes: u64
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Moka {
    crypto_cache: CryptoCache
}