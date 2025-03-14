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
pub struct DelayProfileResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "enableUsenet", skip_serializing_if = "Option::is_none")]
    pub enable_usenet: Option<bool>,
    #[serde(rename = "enableTorrent", skip_serializing_if = "Option::is_none")]
    pub enable_torrent: Option<bool>,
    #[serde(rename = "preferredProtocol", skip_serializing_if = "Option::is_none")]
    pub preferred_protocol: Option<models::DownloadProtocol>,
    #[serde(rename = "usenetDelay", skip_serializing_if = "Option::is_none")]
    pub usenet_delay: Option<i32>,
    #[serde(rename = "torrentDelay", skip_serializing_if = "Option::is_none")]
    pub torrent_delay: Option<i32>,
    #[serde(rename = "bypassIfHighestQuality", skip_serializing_if = "Option::is_none")]
    pub bypass_if_highest_quality: Option<bool>,
    #[serde(rename = "bypassIfAboveCustomFormatScore", skip_serializing_if = "Option::is_none")]
    pub bypass_if_above_custom_format_score: Option<bool>,
    #[serde(rename = "minimumCustomFormatScore", skip_serializing_if = "Option::is_none")]
    pub minimum_custom_format_score: Option<i32>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<Vec<i32>>>,
}

impl DelayProfileResource {
    pub fn new() -> DelayProfileResource {
        DelayProfileResource {
            id: None,
            enable_usenet: None,
            enable_torrent: None,
            preferred_protocol: None,
            usenet_delay: None,
            torrent_delay: None,
            bypass_if_highest_quality: None,
            bypass_if_above_custom_format_score: None,
            minimum_custom_format_score: None,
            order: None,
            tags: None,
        }
    }
}

