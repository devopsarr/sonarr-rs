/*
 * Sonarr
 *
 * Sonarr API docs - The v3 API docs apply to both v3 and v4 versions of Sonarr. Some functionality may only be available in v4 of the Sonarr application.
 *
 * The version of the OpenAPI document: v4.0.12.2823
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`get_wanted_missing`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWantedMissingError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_wanted_missing_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWantedMissingByIdError {
    UnknownValue(serde_json::Value),
}


pub async fn get_wanted_missing(configuration: &configuration::Configuration, page: Option<i32>, page_size: Option<i32>, sort_key: Option<&str>, sort_direction: Option<models::SortDirection>, include_series: Option<bool>, include_images: Option<bool>, monitored: Option<bool>) -> Result<models::EpisodeResourcePagingResource, Error<GetWantedMissingError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_page = page;
    let p_page_size = page_size;
    let p_sort_key = sort_key;
    let p_sort_direction = sort_direction;
    let p_include_series = include_series;
    let p_include_images = include_images;
    let p_monitored = monitored;

    let uri_str = format!("{}/api/v3/wanted/missing", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page_size {
        req_builder = req_builder.query(&[("pageSize", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_sort_key {
        req_builder = req_builder.query(&[("sortKey", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_sort_direction {
        req_builder = req_builder.query(&[("sortDirection", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_include_series {
        req_builder = req_builder.query(&[("includeSeries", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_include_images {
        req_builder = req_builder.query(&[("includeImages", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_monitored {
        req_builder = req_builder.query(&[("monitored", &param_value.to_string())]);
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.query(&[("apikey", value)]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-Api-Key", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<GetWantedMissingError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_wanted_missing_by_id(configuration: &configuration::Configuration, id: i32) -> Result<models::EpisodeResource, Error<GetWantedMissingByIdError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;

    let uri_str = format!("{}/api/v3/wanted/missing/{id}", configuration.base_path, id=p_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.query(&[("apikey", value)]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-Api-Key", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<GetWantedMissingByIdError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

