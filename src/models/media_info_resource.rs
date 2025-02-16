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
pub struct MediaInfoResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "audioBitrate", skip_serializing_if = "Option::is_none")]
    pub audio_bitrate: Option<i64>,
    #[serde(rename = "audioChannels", skip_serializing_if = "Option::is_none")]
    pub audio_channels: Option<f64>,
    #[serde(rename = "audioCodec", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub audio_codec: Option<Option<String>>,
    #[serde(rename = "audioLanguages", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub audio_languages: Option<Option<String>>,
    #[serde(rename = "audioStreamCount", skip_serializing_if = "Option::is_none")]
    pub audio_stream_count: Option<i32>,
    #[serde(rename = "videoBitDepth", skip_serializing_if = "Option::is_none")]
    pub video_bit_depth: Option<i32>,
    #[serde(rename = "videoBitrate", skip_serializing_if = "Option::is_none")]
    pub video_bitrate: Option<i64>,
    #[serde(rename = "videoCodec", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub video_codec: Option<Option<String>>,
    #[serde(rename = "videoFps", skip_serializing_if = "Option::is_none")]
    pub video_fps: Option<f64>,
    #[serde(rename = "videoDynamicRange", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub video_dynamic_range: Option<Option<String>>,
    #[serde(rename = "videoDynamicRangeType", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub video_dynamic_range_type: Option<Option<String>>,
    #[serde(rename = "resolution", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub resolution: Option<Option<String>>,
    #[serde(rename = "runTime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub run_time: Option<Option<String>>,
    #[serde(rename = "scanType", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<Option<String>>,
    #[serde(rename = "subtitles", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub subtitles: Option<Option<String>>,
}

impl MediaInfoResource {
    pub fn new() -> MediaInfoResource {
        MediaInfoResource {
            id: None,
            audio_bitrate: None,
            audio_channels: None,
            audio_codec: None,
            audio_languages: None,
            audio_stream_count: None,
            video_bit_depth: None,
            video_bitrate: None,
            video_codec: None,
            video_fps: None,
            video_dynamic_range: None,
            video_dynamic_range_type: None,
            resolution: None,
            run_time: None,
            scan_type: None,
            subtitles: None,
        }
    }
}

