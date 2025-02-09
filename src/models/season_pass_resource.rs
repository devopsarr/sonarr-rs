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
pub struct SeasonPassResource {
    #[serde(rename = "series", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub series: Option<Option<Vec<models::SeasonPassSeriesResource>>>,
    #[serde(rename = "monitoringOptions", skip_serializing_if = "Option::is_none")]
    pub monitoring_options: Option<Box<models::MonitoringOptions>>,
}

impl SeasonPassResource {
    pub fn new() -> SeasonPassResource {
        SeasonPassResource {
            series: None,
            monitoring_options: None,
        }
    }
}

