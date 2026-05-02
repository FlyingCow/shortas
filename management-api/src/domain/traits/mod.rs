//! Domain traits (ports) for dependency inversion.

pub mod certificate_repository;
pub mod domain_repository;
pub mod outbox_repository;
pub mod route_repository;
pub mod workspace_repository;

pub use certificate_repository::*;
pub use domain_repository::*;
pub use outbox_repository::*;
pub use route_repository::*;
pub use workspace_repository::*;
