//! Macro for generating adapter enum wrappers.
//!
//! This module provides a macro to reduce boilerplate when creating
//! enum-based adapter wrappers for different storage backends.
//!
//! # Why Enums Instead of Trait Objects?
//!
//! We use enum-based dispatch instead of `Box<dyn Trait>` for:
//! - Zero-cost abstractions (no vtable lookup)
//! - Clone-ability without Arc wrappers
//! - Compile-time exhaustiveness checking
//!
//! # Adding a New Adapter
//!
//! 1. Implement the trait for your new store (e.g., `RedisRoutesStore`)
//! 2. Add the variant to the enum using this macro
//! 3. The macro generates all trait method dispatching automatically
//!
//! # Example
//!
//! ```rust,ignore
//! impl_adapter_enum! {
//!     /// Routes storage adapter
//!     pub enum RoutesStoreType: RoutesStore {
//!         Dynamo(DynamoRoutesStore),
//!         Mongodb(MongodbRoutesStore),
//!         InMemory(InMemoryRoutesStore),
//!     }
//!     methods: {
//!         async fn get_route(&self, switch: &str, path: &str) -> Result<Option<Route>>;
//!     }
//! }
//! ```

/// Generates an enum that wraps multiple implementations of a trait,
/// with automatic method dispatch to the inner implementation.
///
/// This macro creates:
/// - The enum with Clone derive
/// - async_trait implementation for the trait
/// - Match-based dispatch for each method
#[macro_export]
macro_rules! impl_adapter_enum {
    (
        $(#[$enum_meta:meta])*
        pub enum $enum_name:ident : $trait_name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident($inner_type:ty)
            ),+ $(,)?
        }
        methods: {
            $(
                async fn $method:ident(&self $(, $param:ident : $param_ty:ty)*) -> $ret:ty;
            )+
        }
    ) => {
        $(#[$enum_meta])*
        #[derive(Clone)]
        pub enum $enum_name {
            $(
                $(#[$variant_meta])*
                $variant($inner_type),
            )+
        }

        #[async_trait::async_trait]
        impl $trait_name for $enum_name {
            $(
                async fn $method(&self $(, $param : $param_ty)*) -> $ret {
                    match self {
                        $(
                            $enum_name::$variant(inner) => inner.$method($($param),*).await,
                        )+
                    }
                }
            )+
        }
    };
}

/// Generates a sync adapter enum (for non-async traits).
#[macro_export]
macro_rules! impl_sync_adapter_enum {
    (
        $(#[$enum_meta:meta])*
        pub enum $enum_name:ident : $trait_name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident($inner_type:ty)
            ),+ $(,)?
        }
        methods: {
            $(
                fn $method:ident(&self $(, $param:ident : $param_ty:ty)*) -> $ret:ty;
            )+
        }
    ) => {
        $(#[$enum_meta])*
        #[derive(Clone)]
        pub enum $enum_name {
            $(
                $(#[$variant_meta])*
                $variant($inner_type),
            )+
        }

        #[async_trait::async_trait]
        impl $trait_name for $enum_name {
            $(
                fn $method(&self $(, $param : $param_ty)*) -> $ret {
                    match self {
                        $(
                            $enum_name::$variant(inner) => inner.$method($($param),*),
                        )+
                    }
                }
            )+
        }
    };
}

pub use impl_adapter_enum;
pub use impl_sync_adapter_enum;
