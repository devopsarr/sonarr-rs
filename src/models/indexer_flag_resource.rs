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
pub struct IndexerFlagResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "nameLower", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name_lower: Option<Option<String>>,
}

impl IndexerFlagResource {
    pub fn new() -> IndexerFlagResource {
        IndexerFlagResource {
            id: None,
            name: None,
            name_lower: None,
        }
    }
}

