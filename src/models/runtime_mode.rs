/*
 * Sonarr
 *
 * Sonarr API docs - The v3 API docs apply to both v3 and v4 versions of Sonarr. Some functionality may only be available in v4 of the Sonarr application.
 *
 * The version of the OpenAPI document: v4.0.13.2932
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RuntimeMode {
    #[serde(rename = "console")]
    Console,
    #[serde(rename = "service")]
    Service,
    #[serde(rename = "tray")]
    Tray,

}

impl std::fmt::Display for RuntimeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Console => write!(f, "console"),
            Self::Service => write!(f, "service"),
            Self::Tray => write!(f, "tray"),
        }
    }
}

impl Default for RuntimeMode {
    fn default() -> RuntimeMode {
        Self::Console
    }
}

