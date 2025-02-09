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
pub struct NotificationResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "fields", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Option<Vec<models::Field>>>,
    #[serde(rename = "implementationName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub implementation_name: Option<Option<String>>,
    #[serde(rename = "implementation", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub implementation: Option<Option<String>>,
    #[serde(rename = "configContract", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub config_contract: Option<Option<String>>,
    #[serde(rename = "infoLink", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub info_link: Option<Option<String>>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<models::ProviderMessage>>,
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<Vec<i32>>>,
    #[serde(rename = "presets", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub presets: Option<Option<Vec<models::NotificationResource>>>,
    #[serde(rename = "link", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub link: Option<Option<String>>,
    #[serde(rename = "onGrab", skip_serializing_if = "Option::is_none")]
    pub on_grab: Option<bool>,
    #[serde(rename = "onDownload", skip_serializing_if = "Option::is_none")]
    pub on_download: Option<bool>,
    #[serde(rename = "onUpgrade", skip_serializing_if = "Option::is_none")]
    pub on_upgrade: Option<bool>,
    #[serde(rename = "onImportComplete", skip_serializing_if = "Option::is_none")]
    pub on_import_complete: Option<bool>,
    #[serde(rename = "onRename", skip_serializing_if = "Option::is_none")]
    pub on_rename: Option<bool>,
    #[serde(rename = "onSeriesAdd", skip_serializing_if = "Option::is_none")]
    pub on_series_add: Option<bool>,
    #[serde(rename = "onSeriesDelete", skip_serializing_if = "Option::is_none")]
    pub on_series_delete: Option<bool>,
    #[serde(rename = "onEpisodeFileDelete", skip_serializing_if = "Option::is_none")]
    pub on_episode_file_delete: Option<bool>,
    #[serde(rename = "onEpisodeFileDeleteForUpgrade", skip_serializing_if = "Option::is_none")]
    pub on_episode_file_delete_for_upgrade: Option<bool>,
    #[serde(rename = "onHealthIssue", skip_serializing_if = "Option::is_none")]
    pub on_health_issue: Option<bool>,
    #[serde(rename = "includeHealthWarnings", skip_serializing_if = "Option::is_none")]
    pub include_health_warnings: Option<bool>,
    #[serde(rename = "onHealthRestored", skip_serializing_if = "Option::is_none")]
    pub on_health_restored: Option<bool>,
    #[serde(rename = "onApplicationUpdate", skip_serializing_if = "Option::is_none")]
    pub on_application_update: Option<bool>,
    #[serde(rename = "onManualInteractionRequired", skip_serializing_if = "Option::is_none")]
    pub on_manual_interaction_required: Option<bool>,
    #[serde(rename = "supportsOnGrab", skip_serializing_if = "Option::is_none")]
    pub supports_on_grab: Option<bool>,
    #[serde(rename = "supportsOnDownload", skip_serializing_if = "Option::is_none")]
    pub supports_on_download: Option<bool>,
    #[serde(rename = "supportsOnUpgrade", skip_serializing_if = "Option::is_none")]
    pub supports_on_upgrade: Option<bool>,
    #[serde(rename = "supportsOnImportComplete", skip_serializing_if = "Option::is_none")]
    pub supports_on_import_complete: Option<bool>,
    #[serde(rename = "supportsOnRename", skip_serializing_if = "Option::is_none")]
    pub supports_on_rename: Option<bool>,
    #[serde(rename = "supportsOnSeriesAdd", skip_serializing_if = "Option::is_none")]
    pub supports_on_series_add: Option<bool>,
    #[serde(rename = "supportsOnSeriesDelete", skip_serializing_if = "Option::is_none")]
    pub supports_on_series_delete: Option<bool>,
    #[serde(rename = "supportsOnEpisodeFileDelete", skip_serializing_if = "Option::is_none")]
    pub supports_on_episode_file_delete: Option<bool>,
    #[serde(rename = "supportsOnEpisodeFileDeleteForUpgrade", skip_serializing_if = "Option::is_none")]
    pub supports_on_episode_file_delete_for_upgrade: Option<bool>,
    #[serde(rename = "supportsOnHealthIssue", skip_serializing_if = "Option::is_none")]
    pub supports_on_health_issue: Option<bool>,
    #[serde(rename = "supportsOnHealthRestored", skip_serializing_if = "Option::is_none")]
    pub supports_on_health_restored: Option<bool>,
    #[serde(rename = "supportsOnApplicationUpdate", skip_serializing_if = "Option::is_none")]
    pub supports_on_application_update: Option<bool>,
    #[serde(rename = "supportsOnManualInteractionRequired", skip_serializing_if = "Option::is_none")]
    pub supports_on_manual_interaction_required: Option<bool>,
    #[serde(rename = "testCommand", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub test_command: Option<Option<String>>,
}

impl NotificationResource {
    pub fn new() -> NotificationResource {
        NotificationResource {
            id: None,
            name: None,
            fields: None,
            implementation_name: None,
            implementation: None,
            config_contract: None,
            info_link: None,
            message: None,
            tags: None,
            presets: None,
            link: None,
            on_grab: None,
            on_download: None,
            on_upgrade: None,
            on_import_complete: None,
            on_rename: None,
            on_series_add: None,
            on_series_delete: None,
            on_episode_file_delete: None,
            on_episode_file_delete_for_upgrade: None,
            on_health_issue: None,
            include_health_warnings: None,
            on_health_restored: None,
            on_application_update: None,
            on_manual_interaction_required: None,
            supports_on_grab: None,
            supports_on_download: None,
            supports_on_upgrade: None,
            supports_on_import_complete: None,
            supports_on_rename: None,
            supports_on_series_add: None,
            supports_on_series_delete: None,
            supports_on_episode_file_delete: None,
            supports_on_episode_file_delete_for_upgrade: None,
            supports_on_health_issue: None,
            supports_on_health_restored: None,
            supports_on_application_update: None,
            supports_on_manual_interaction_required: None,
            test_command: None,
        }
    }
}

