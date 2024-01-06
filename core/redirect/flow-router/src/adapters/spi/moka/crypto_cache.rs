// use std::time::Duration;

// use moka::future::Cache;

// use crate::domain::Keycert;
// use crate::core::base_crypto_cache::{ 
//     BaseCryptoCache, 
//     CryptoCacheError, 
//     Result
// };

// #[derive(Clone, Debug)]
// pub struct CryptoCache {
//     cache:  Cache
// }

// impl CryptoCache {
//     pub fn new() -> Self {
//         let cache = Cache::builder()
//             .max_capacity(2)
//             .time_to_live(Duration::from_secs(ttl))
//             .eviction_listener(|key, value, cause| {
//                 println!("Evicted ({key:?},{value:?}) because {cause:?}")
//             })
//             .build();
    
//         Self {}
//     }
// }

// impl BaseCryptoCache for CryptoCache {
//     async fn get_certificate(&self, server_name: &str) -> Result<Option<Keycert>> {
//         Ok(Some(
//             Keycert::new()
//                 .cert_from_path("./certs/cert.pem")
//                 .unwrap()
//                 .key_from_path("/certs/key.pem")
//                 .unwrap(),
//         ))
//     }

//     async fn add_certificate(&self, server_name: &str, keycert: Option<Keycert>) -> Result<()> {

//     }
// }