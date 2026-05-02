//! Domain entities for the management API.
//!
//! Re-exports from shortas-common and management-specific entities.

pub mod outbox;
pub mod user;

// Re-export common types
pub use shortas_common::{
    ApiError, BlockedReason, Certificate, Condition, ConditionalRouting, DestinationFormat,
    DomainVerificationStatus, ErrorCode, OptionExt, QrSettings, Result, ResultExt, Route,
    RouteDomain, RouteProperties, RouteStatus, RoutingPolicy, RoutingTerminal, UserWorkspace,
    Workspace,
};

pub use outbox::*;
pub use user::*;
