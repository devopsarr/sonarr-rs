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
pub struct SeriesResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<String>>,
    #[serde(rename = "alternateTitles", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub alternate_titles: Option<Option<Vec<models::AlternateTitleResource>>>,
    #[serde(rename = "sortTitle", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sort_title: Option<Option<String>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::SeriesStatusType>,
    #[serde(rename = "ended", skip_serializing_if = "Option::is_none")]
    pub ended: Option<bool>,
    #[serde(rename = "profileName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<Option<String>>,
    #[serde(rename = "overview", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub overview: Option<Option<String>>,
    #[serde(rename = "nextAiring", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub next_airing: Option<Option<String>>,
    #[serde(rename = "previousAiring", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub previous_airing: Option<Option<String>>,
    #[serde(rename = "network", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub network: Option<Option<String>>,
    #[serde(rename = "airTime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub air_time: Option<Option<String>>,
    #[serde(rename = "images", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub images: Option<Option<Vec<models::MediaCover>>>,
    #[serde(rename = "originalLanguage", skip_serializing_if = "Option::is_none")]
    pub original_language: Option<Box<models::Language>>,
    #[serde(rename = "remotePoster", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub remote_poster: Option<Option<String>>,
    #[serde(rename = "seasons", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub seasons: Option<Option<Vec<models::SeasonResource>>>,
    #[serde(rename = "year", skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    #[serde(rename = "path", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub path: Option<Option<String>>,
    #[serde(rename = "qualityProfileId", skip_serializing_if = "Option::is_none")]
    pub quality_profile_id: Option<i32>,
    #[serde(rename = "seasonFolder", skip_serializing_if = "Option::is_none")]
    pub season_folder: Option<bool>,
    #[serde(rename = "monitored", skip_serializing_if = "Option::is_none")]
    pub monitored: Option<bool>,
    #[serde(rename = "monitorNewItems", skip_serializing_if = "Option::is_none")]
    pub monitor_new_items: Option<models::NewItemMonitorTypes>,
    #[serde(rename = "useSceneNumbering", skip_serializing_if = "Option::is_none")]
    pub use_scene_numbering: Option<bool>,
    #[serde(rename = "runtime", skip_serializing_if = "Option::is_none")]
    pub runtime: Option<i32>,
    #[serde(rename = "tvdbId", skip_serializing_if = "Option::is_none")]
    pub tvdb_id: Option<i32>,
    #[serde(rename = "tvRageId", skip_serializing_if = "Option::is_none")]
    pub tv_rage_id: Option<i32>,
    #[serde(rename = "tvMazeId", skip_serializing_if = "Option::is_none")]
    pub tv_maze_id: Option<i32>,
    #[serde(rename = "tmdbId", skip_serializing_if = "Option::is_none")]
    pub tmdb_id: Option<i32>,
    #[serde(rename = "firstAired", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub first_aired: Option<Option<String>>,
    #[serde(rename = "lastAired", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_aired: Option<Option<String>>,
    #[serde(rename = "seriesType", skip_serializing_if = "Option::is_none")]
    pub series_type: Option<models::SeriesTypes>,
    #[serde(rename = "cleanTitle", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub clean_title: Option<Option<String>>,
    #[serde(rename = "imdbId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub imdb_id: Option<Option<String>>,
    #[serde(rename = "titleSlug", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title_slug: Option<Option<String>>,
    #[serde(rename = "rootFolderPath", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub root_folder_path: Option<Option<String>>,
    #[serde(rename = "folder", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub folder: Option<Option<String>>,
    #[serde(rename = "certification", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub certification: Option<Option<String>>,
    #[serde(rename = "genres", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub genres: Option<Option<Vec<String>>>,
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<Vec<i32>>>,
    #[serde(rename = "added", skip_serializing_if = "Option::is_none")]
    pub added: Option<String>,
    #[serde(rename = "addOptions", skip_serializing_if = "Option::is_none")]
    pub add_options: Option<Box<models::AddSeriesOptions>>,
    #[serde(rename = "ratings", skip_serializing_if = "Option::is_none")]
    pub ratings: Option<Box<models::Ratings>>,
    #[serde(rename = "statistics", skip_serializing_if = "Option::is_none")]
    pub statistics: Option<Box<models::SeriesStatisticsResource>>,
    #[serde(rename = "episodesChanged", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub episodes_changed: Option<Option<bool>>,
    #[serde(rename = "languageProfileId", skip_serializing_if = "Option::is_none")]
    pub language_profile_id: Option<i32>,
}

impl SeriesResource {
    pub fn new() -> SeriesResource {
        SeriesResource {
            id: None,
            title: None,
            alternate_titles: None,
            sort_title: None,
            status: None,
            ended: None,
            profile_name: None,
            overview: None,
            next_airing: None,
            previous_airing: None,
            network: None,
            air_time: None,
            images: None,
            original_language: None,
            remote_poster: None,
            seasons: None,
            year: None,
            path: None,
            quality_profile_id: None,
            season_folder: None,
            monitored: None,
            monitor_new_items: None,
            use_scene_numbering: None,
            runtime: None,
            tvdb_id: None,
            tv_rage_id: None,
            tv_maze_id: None,
            tmdb_id: None,
            first_aired: None,
            last_aired: None,
            series_type: None,
            clean_title: None,
            imdb_id: None,
            title_slug: None,
            root_folder_path: None,
            folder: None,
            certification: None,
            genres: None,
            tags: None,
            added: None,
            add_options: None,
            ratings: None,
            statistics: None,
            episodes_changed: None,
            language_profile_id: None,
        }
    }
}

