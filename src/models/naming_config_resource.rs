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
pub struct NamingConfigResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "renameEpisodes", skip_serializing_if = "Option::is_none")]
    pub rename_episodes: Option<bool>,
    #[serde(rename = "replaceIllegalCharacters", skip_serializing_if = "Option::is_none")]
    pub replace_illegal_characters: Option<bool>,
    #[serde(rename = "colonReplacementFormat", skip_serializing_if = "Option::is_none")]
    pub colon_replacement_format: Option<i32>,
    #[serde(rename = "customColonReplacementFormat", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub custom_colon_replacement_format: Option<Option<String>>,
    #[serde(rename = "multiEpisodeStyle", skip_serializing_if = "Option::is_none")]
    pub multi_episode_style: Option<i32>,
    #[serde(rename = "standardEpisodeFormat", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub standard_episode_format: Option<Option<String>>,
    #[serde(rename = "dailyEpisodeFormat", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub daily_episode_format: Option<Option<String>>,
    #[serde(rename = "animeEpisodeFormat", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub anime_episode_format: Option<Option<String>>,
    #[serde(rename = "seriesFolderFormat", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub series_folder_format: Option<Option<String>>,
    #[serde(rename = "seasonFolderFormat", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub season_folder_format: Option<Option<String>>,
    #[serde(rename = "specialsFolderFormat", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub specials_folder_format: Option<Option<String>>,
}

impl NamingConfigResource {
    pub fn new() -> NamingConfigResource {
        NamingConfigResource {
            id: None,
            rename_episodes: None,
            replace_illegal_characters: None,
            colon_replacement_format: None,
            custom_colon_replacement_format: None,
            multi_episode_style: None,
            standard_episode_format: None,
            daily_episode_format: None,
            anime_episode_format: None,
            series_folder_format: None,
            season_folder_format: None,
            specials_folder_format: None,
        }
    }
}

