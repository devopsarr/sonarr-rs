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
pub enum CommandResult {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "successful")]
    Successful,
    #[serde(rename = "unsuccessful")]
    Unsuccessful,

}

impl std::fmt::Display for CommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "unknown"),
            Self::Successful => write!(f, "successful"),
            Self::Unsuccessful => write!(f, "unsuccessful"),
        }
    }
}

impl Default for CommandResult {
    fn default() -> CommandResult {
        Self::Unknown
    }
}

