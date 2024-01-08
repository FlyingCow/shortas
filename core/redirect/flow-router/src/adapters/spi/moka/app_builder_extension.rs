//use crate::core::app_builder::{ AppBuilder, BaseCryptoBuilder };
// use crate::adapters::spi::moka::crypto_cache::CryptoCache;

// impl<M> AppBuilder<M> where M: BaseCryptoBuilder {
//     pub fn with_moka(&mut self, max_capacity: u64, time_to_live_minutes: u64, time_to_idle_minutes: u64) -> &Self {

//         &self.crypto_builder.with_crypto_cache(CryptoCache::new(max_capacity, time_to_live_minutes, time_to_idle_minutes));
//         self
//     }
// }