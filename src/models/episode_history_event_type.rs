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
pub enum EpisodeHistoryEventType {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "grabbed")]
    Grabbed,
    #[serde(rename = "seriesFolderImported")]
    SeriesFolderImported,
    #[serde(rename = "downloadFolderImported")]
    DownloadFolderImported,
    #[serde(rename = "downloadFailed")]
    DownloadFailed,
    #[serde(rename = "episodeFileDeleted")]
    EpisodeFileDeleted,
    #[serde(rename = "episodeFileRenamed")]
    EpisodeFileRenamed,
    #[serde(rename = "downloadIgnored")]
    DownloadIgnored,

}

impl std::fmt::Display for EpisodeHistoryEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "unknown"),
            Self::Grabbed => write!(f, "grabbed"),
            Self::SeriesFolderImported => write!(f, "seriesFolderImported"),
            Self::DownloadFolderImported => write!(f, "downloadFolderImported"),
            Self::DownloadFailed => write!(f, "downloadFailed"),
            Self::EpisodeFileDeleted => write!(f, "episodeFileDeleted"),
            Self::EpisodeFileRenamed => write!(f, "episodeFileRenamed"),
            Self::DownloadIgnored => write!(f, "downloadIgnored"),
        }
    }
}

impl Default for EpisodeHistoryEventType {
    fn default() -> EpisodeHistoryEventType {
        Self::Unknown
    }
}

