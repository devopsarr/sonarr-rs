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
pub struct MediaCover {
    #[serde(rename = "coverType", skip_serializing_if = "Option::is_none")]
    pub cover_type: Option<models::MediaCoverTypes>,
    #[serde(rename = "url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub url: Option<Option<String>>,
    #[serde(rename = "remoteUrl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub remote_url: Option<Option<String>>,
}

impl MediaCover {
    pub fn new() -> MediaCover {
        MediaCover {
            cover_type: None,
            url: None,
            remote_url: None,
        }
    }
}

