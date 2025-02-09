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
pub struct SeriesEditorResource {
    #[serde(rename = "seriesIds", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub series_ids: Option<Option<Vec<i32>>>,
    #[serde(rename = "monitored", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub monitored: Option<Option<bool>>,
    #[serde(rename = "monitorNewItems", skip_serializing_if = "Option::is_none")]
    pub monitor_new_items: Option<models::NewItemMonitorTypes>,
    #[serde(rename = "qualityProfileId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub quality_profile_id: Option<Option<i32>>,
    #[serde(rename = "seriesType", skip_serializing_if = "Option::is_none")]
    pub series_type: Option<models::SeriesTypes>,
    #[serde(rename = "seasonFolder", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub season_folder: Option<Option<bool>>,
    #[serde(rename = "rootFolderPath", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub root_folder_path: Option<Option<String>>,
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<Vec<i32>>>,
    #[serde(rename = "applyTags", skip_serializing_if = "Option::is_none")]
    pub apply_tags: Option<models::ApplyTags>,
    #[serde(rename = "moveFiles", skip_serializing_if = "Option::is_none")]
    pub move_files: Option<bool>,
    #[serde(rename = "deleteFiles", skip_serializing_if = "Option::is_none")]
    pub delete_files: Option<bool>,
    #[serde(rename = "addImportListExclusion", skip_serializing_if = "Option::is_none")]
    pub add_import_list_exclusion: Option<bool>,
}

impl SeriesEditorResource {
    pub fn new() -> SeriesEditorResource {
        SeriesEditorResource {
            series_ids: None,
            monitored: None,
            monitor_new_items: None,
            quality_profile_id: None,
            series_type: None,
            season_folder: None,
            root_folder_path: None,
            tags: None,
            apply_tags: None,
            move_files: None,
            delete_files: None,
            add_import_list_exclusion: None,
        }
    }
}

