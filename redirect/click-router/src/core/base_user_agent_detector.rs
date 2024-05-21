use std::borrow::Cow;

use dyn_clone::{clone_trait_object, DynClone};
use serde::{Deserialize, Serialize};

/// Describes the `Family` as well as the `Major`, `Minor`, `Patch`, and
/// `PatchMinor` versions of an `OS`
#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct OS<'a> {
    pub family: Cow<'a, str>,
    pub major: Option<Cow<'a, str>>,
    pub minor: Option<Cow<'a, str>>,
    pub patch: Option<Cow<'a, str>>,
    pub patch_minor: Option<Cow<'a, str>>,
}

impl<'a> Default for OS<'a> {
    fn default() -> Self {
        Self {
            family: Cow::Borrowed("Other"),
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
pub struct UserAgent<'a> {
    pub family: Cow<'a, str>,
    pub major: Option<Cow<'a, str>>,
    pub minor: Option<Cow<'a, str>>,
    pub patch: Option<Cow<'a, str>>,
}

impl<'a> Default for UserAgent<'a> {
    fn default() -> Self {
        Self {
            family: Cow::Borrowed("Other"),
            major: None,
            minor: None,
            patch: None,
        }
    }
}

/// Describes the `Family`, `Brand` and `Model` of a `Device`
#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Device<'a> {
    pub family: Cow<'a, str>,
    pub brand: Option<Cow<'a, str>>,
    pub model: Option<Cow<'a, str>>,
}

impl<'a> Default for Device<'a> {
    fn default() -> Self {
        Self {
            family: Cow::Borrowed("Other"),
            brand: None,
            model: None,
        }
    }
}

/// Houses the `Device`, `OS`, and `UserAgent` structs, which each get parsed
/// out from a user agent string by a `UserAgentParser`.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Client<'a> {
    pub device: Device<'a>,
    pub os: OS<'a>,
    pub user_agent: UserAgent<'a>,
}

pub trait BaseUserAgentDetector: DynClone {
    fn parse_device<'a>(&self, user_agent: &'a str) -> Device<'a>;
    fn parse_os<'a>(&self, user_agent: &'a str) -> OS<'a>;
    fn parse_user_agent<'a>(&self, user_agent: &'a str) -> UserAgent<'a>;
}

clone_trait_object!(BaseUserAgentDetector);