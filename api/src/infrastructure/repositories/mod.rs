//! Repository implementations using SQLx.

pub mod certificate_repo;
pub mod domain_repo;
pub mod invitation_repo;
pub mod notification_repo;
pub mod outbox_repo;
pub mod route_repo;
pub mod user_repo;
pub mod workspace_repo;

pub use certificate_repo::*;
pub use domain_repo::*;
pub use invitation_repo::*;
pub use notification_repo::*;
pub use outbox_repo::*;
pub use route_repo::*;
pub use user_repo::*;
pub use workspace_repo::*;
