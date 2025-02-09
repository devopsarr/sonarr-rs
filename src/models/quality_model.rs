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
pub struct QualityModel {
    #[serde(rename = "quality", skip_serializing_if = "Option::is_none")]
    pub quality: Option<Box<models::Quality>>,
    #[serde(rename = "revision", skip_serializing_if = "Option::is_none")]
    pub revision: Option<Box<models::Revision>>,
}

impl QualityModel {
    pub fn new() -> QualityModel {
        QualityModel {
            quality: None,
            revision: None,
        }
    }
}

