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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlocklistResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "seriesId", skip_serializing_if = "Option::is_none")]
    pub series_id: Option<i32>,
    #[serde(rename = "episodeIds", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub episode_ids: Option<Option<Vec<i32>>>,
    #[serde(rename = "sourceTitle", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub source_title: Option<Option<String>>,
    #[serde(rename = "languages", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub languages: Option<Option<Vec<models::Language>>>,
    #[serde(rename = "quality", skip_serializing_if = "Option::is_none")]
    pub quality: Option<Box<models::QualityModel>>,
    #[serde(rename = "customFormats", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub custom_formats: Option<Option<Vec<models::CustomFormatResource>>>,
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<models::DownloadProtocol>,
    #[serde(rename = "indexer", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub indexer: Option<Option<String>>,
    #[serde(rename = "message", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub message: Option<Option<String>>,
    #[serde(rename = "series", skip_serializing_if = "Option::is_none")]
    pub series: Option<Box<models::SeriesResource>>,
}

impl BlocklistResource {
    pub fn new() -> BlocklistResource {
        BlocklistResource {
            id: None,
            series_id: None,
            episode_ids: None,
            source_title: None,
            languages: None,
            quality: None,
            custom_formats: None,
            date: None,
            protocol: None,
            indexer: None,
            message: None,
            series: None,
        }
    }
}

