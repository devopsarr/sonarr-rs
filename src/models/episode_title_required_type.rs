/*
 * Sonarr
 *
 * Sonarr API docs - The v3 API docs apply to both v3 and v4 versions of Sonarr. Some functionality may only be available in v4 of the Sonarr application.
 *
 * The version of the OpenAPI document: v4.0.12.2823
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EpisodeTitleRequiredType {
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "bulkSeasonReleases")]
    BulkSeasonReleases,
    #[serde(rename = "never")]
    Never,

}

impl std::fmt::Display for EpisodeTitleRequiredType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Always => write!(f, "always"),
            Self::BulkSeasonReleases => write!(f, "bulkSeasonReleases"),
            Self::Never => write!(f, "never"),
        }
    }
}

impl Default for EpisodeTitleRequiredType {
    fn default() -> EpisodeTitleRequiredType {
        Self::Always
    }
}

