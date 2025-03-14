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
pub struct ImportListConfigResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "listSyncLevel", skip_serializing_if = "Option::is_none")]
    pub list_sync_level: Option<models::ListSyncLevelType>,
    #[serde(rename = "listSyncTag", skip_serializing_if = "Option::is_none")]
    pub list_sync_tag: Option<i32>,
}

impl ImportListConfigResource {
    pub fn new() -> ImportListConfigResource {
        ImportListConfigResource {
            id: None,
            list_sync_level: None,
            list_sync_tag: None,
        }
    }
}

