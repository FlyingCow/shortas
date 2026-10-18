//! Domain entities for the management API.
//!
//! These are management API's own domain entities.
//! Use shortas-common types only when communicating with underlying APIs.

pub mod condition;
pub mod domain;
pub mod error;
pub mod outbox;
pub mod route;
pub mod user;

// Re-export condition types
pub use condition::{Condition, DefaultOperator, NumericCondition, StringCondition};

// Re-export domain types
pub use domain::{
    Certificate, DnsConfig, DomainVerificationStatus, RouteDomain, UserWorkspace, Workspace,
};

// Re-export error types
pub use error::{ApiError, ErrorCode, OptionExt, Result, ResultExt};

// Re-export route types
pub use route::{
    BlockedReason, ChallengeRouting, ConditionalRouting, DestinationFormat, FileRouting,
    QrSettings, Route, RouteProperties, RouteStatus, RoutingPolicy, RoutingTerminal,
};

// Re-export outbox and user types
pub use outbox::*;
pub use user::*;
