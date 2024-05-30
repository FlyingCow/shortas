use dyn_clone::{clone_trait_object, DynClone};
use serde::{Deserialize, Serialize};

/// Describes the `Family` as well as the `Major`, `Minor`, `Patch`, and
/// `PatchMinor` versions of an `OS`
#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct OS {
    pub family: String,
    pub major: Option<String>,
    pub minor: Option<String>,
    pub patch: Option<String>,
    pub patch_minor: Option<String>,
}

impl Default for OS {
    fn default() -> Self {
        Self {
            family: String::from("Other"),
            major: None,
            minor: None,
            patch: None,
            patch_minor: None,
        }
    }
}

/// Describes the `Family` as well as the `Major`, `Minor`, and `Patch` versions
/// of a `UserAgent` client
#[derive(Clone, Debug, Deserialize, Serialize, Eq, Hash, PartialEq)]
pub struct UserAgent {
    pub family: String,
    pub major: Option<String>,
    pub minor: Option<String>,
    pub patch: Option<String>,
}

impl Default for UserAgent {
    fn default() -> Self {
        Self {
            family: String::from("Other"),
            major: None,
            minor: None,
            patch: None,
        }
    }
}

/// Describes the `Family`, `Brand` and `Model` of a `Device`
#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Device {
    pub family: String,
    pub brand: Option<String>,
    pub model: Option<String>,
}

impl Default for Device {
    fn default() -> Self {
        Self {
            family: String::from("Other"),
            brand: None,
            model: None,
        }
    }
}

/// Houses the `Device`, `OS`, and `UserAgent` structs, which each get parsed
/// out from a user agent string by a `UserAgentParser`.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Client {
    pub device: Device,
    pub os: OS,
    pub user_agent: UserAgent,
}

pub trait BaseUserAgentDetector: DynClone {
    fn parse_device(&self, user_agent: &str) -> Device;
    fn parse_os(&self, user_agent: &str) -> OS;
    fn parse_user_agent(&self, user_agent: &str) -> UserAgent;
}

clone_trait_object!(BaseUserAgentDetector);
