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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueueResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "seriesId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub series_id: Option<Option<i32>>,
    #[serde(rename = "episodeId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub episode_id: Option<Option<i32>>,
    #[serde(rename = "seasonNumber", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub season_number: Option<Option<i32>>,
    #[serde(rename = "series", skip_serializing_if = "Option::is_none")]
    pub series: Option<Box<models::SeriesResource>>,
    #[serde(rename = "episode", skip_serializing_if = "Option::is_none")]
    pub episode: Option<Box<models::EpisodeResource>>,
    #[serde(rename = "languages", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub languages: Option<Option<Vec<models::Language>>>,
    #[serde(rename = "quality", skip_serializing_if = "Option::is_none")]
    pub quality: Option<Box<models::QualityModel>>,
    #[serde(rename = "customFormats", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub custom_formats: Option<Option<Vec<models::CustomFormatResource>>>,
    #[serde(rename = "customFormatScore", skip_serializing_if = "Option::is_none")]
    pub custom_format_score: Option<i32>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<String>>,
    #[serde(rename = "estimatedCompletionTime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub estimated_completion_time: Option<Option<String>>,
    #[serde(rename = "added", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub added: Option<Option<String>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::QueueStatus>,
    #[serde(rename = "trackedDownloadStatus", skip_serializing_if = "Option::is_none")]
    pub tracked_download_status: Option<models::TrackedDownloadStatus>,
    #[serde(rename = "trackedDownloadState", skip_serializing_if = "Option::is_none")]
    pub tracked_download_state: Option<models::TrackedDownloadState>,
    #[serde(rename = "statusMessages", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub status_messages: Option<Option<Vec<models::TrackedDownloadStatusMessage>>>,
    #[serde(rename = "errorMessage", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<Option<String>>,
    #[serde(rename = "downloadId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub download_id: Option<Option<String>>,
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<models::DownloadProtocol>,
    #[serde(rename = "downloadClient", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub download_client: Option<Option<String>>,
    #[serde(rename = "downloadClientHasPostImportCategory", skip_serializing_if = "Option::is_none")]
    pub download_client_has_post_import_category: Option<bool>,
    #[serde(rename = "indexer", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub indexer: Option<Option<String>>,
    #[serde(rename = "outputPath", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub output_path: Option<Option<String>>,
    #[serde(rename = "episodeHasFile", skip_serializing_if = "Option::is_none")]
    pub episode_has_file: Option<bool>,
    #[serde(rename = "sizeleft", skip_serializing_if = "Option::is_none")]
    pub sizeleft: Option<f64>,
    #[serde(rename = "timeleft", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub timeleft: Option<Option<String>>,
}

impl QueueResource {
    pub fn new() -> QueueResource {
        QueueResource {
            id: None,
            series_id: None,
            episode_id: None,
            season_number: None,
            series: None,
            episode: None,
            languages: None,
            quality: None,
            custom_formats: None,
            custom_format_score: None,
            size: None,
            title: None,
            estimated_completion_time: None,
            added: None,
            status: None,
            tracked_download_status: None,
            tracked_download_state: None,
            status_messages: None,
            error_message: None,
            download_id: None,
            protocol: None,
            download_client: None,
            download_client_has_post_import_category: None,
            indexer: None,
            output_path: None,
            episode_has_file: None,
            sizeleft: None,
            timeleft: None,
        }
    }
}

