// use std::marker::PhantomData;

// use crate::core::default::CryptoManager;
// use crate::core::BaseCryptoCache;
// use crate::core::BaseCryptoManager;
// use crate::core::BaseCryptoStore;

// pub struct AppBuilder<CB, C, S>
// where
//     CB: BaseCryptoBuilder<C, S>,
//     C: BaseCryptoCache,
//     S: BaseCryptoStore,
// {
//     pub crypto_builder: CB,
// }

// impl<CB, C, S> AppBuilder<CB, C, S>
// where
//     CB: BaseCryptoBuilder<C, S>,
//     C: BaseCryptoCache,
//     S: BaseCryptoStore,
// {
//     pub fn new(crypto_builder: CB) -> Self {
//         Self {
//             crypto_builder: crypto_builder,
//         }
//     }

//     pub fn build(self) {
//         let crypto_manager = self.crypto_builder.build();
//     }
// }

// pub trait BaseCryptoBuilder<C, S>
// where
//     C: BaseCryptoCache,
//     S: BaseCryptoStore,
// {
//     fn build(self) -> impl BaseCryptoManager;
// }

// impl<C, S> BaseCryptoBuilder<C, S> for CryptoBuilder<C, S>
// where
//     C: BaseCryptoCache,
//     S: BaseCryptoStore,
// {
//     fn build(self) -> impl BaseCryptoManager + Send + Sync {
//         let manager = CryptoManager::new(self.crypto_store, self.crypto_cache);
//         manager
//     }
// }

// pub struct CryptoBuilder<C, S>
// where
//     C: BaseCryptoCache,
//     S: BaseCryptoStore,
// {
//     crypto_cache: C,
//     crypto_store: S,
// }

// impl<C, S> CryptoBuilder<C, S>
// where
//     C: BaseCryptoCache,
//     S: BaseCryptoStore,
// {
//     pub fn new (crypto_store: S, crypto_cache: C) -> Self 
//     {
//         CryptoBuilder {
//             crypto_store,
//             crypto_cache
//         }
//     }
// }