/*
 * Sonarr
 *
 * Sonarr API docs - The v3 API docs apply to both v3 and v4 versions of Sonarr. Some functionality may only be available in v4 of the Sonarr application.
 *
 * The version of the OpenAPI document: v4.0.13.2932
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`delete_queue`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteQueueError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_queue_bulk`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteQueueBulkError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_queue`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetQueueError {
    UnknownValue(serde_json::Value),
}


pub async fn delete_queue(configuration: &configuration::Configuration, id: i32, remove_from_client: Option<bool>, blocklist: Option<bool>, skip_redownload: Option<bool>, change_category: Option<bool>) -> Result<(), Error<DeleteQueueError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;
    let p_remove_from_client = remove_from_client;
    let p_blocklist = blocklist;
    let p_skip_redownload = skip_redownload;
    let p_change_category = change_category;

    let uri_str = format!("{}/api/v3/queue/{id}", configuration.base_path, id=p_id);
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref param_value) = p_remove_from_client {
        req_builder = req_builder.query(&[("removeFromClient", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_blocklist {
        req_builder = req_builder.query(&[("blocklist", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_skip_redownload {
        req_builder = req_builder.query(&[("skipRedownload", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_change_category {
        req_builder = req_builder.query(&[("changeCategory", &param_value.to_string())]);
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
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteQueueError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_queue_bulk(configuration: &configuration::Configuration, remove_from_client: Option<bool>, blocklist: Option<bool>, skip_redownload: Option<bool>, change_category: Option<bool>, queue_bulk_resource: Option<models::QueueBulkResource>) -> Result<(), Error<DeleteQueueBulkError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_remove_from_client = remove_from_client;
    let p_blocklist = blocklist;
    let p_skip_redownload = skip_redownload;
    let p_change_category = change_category;
    let p_queue_bulk_resource = queue_bulk_resource;

    let uri_str = format!("{}/api/v3/queue/bulk", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref param_value) = p_remove_from_client {
        req_builder = req_builder.query(&[("removeFromClient", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_blocklist {
        req_builder = req_builder.query(&[("blocklist", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_skip_redownload {
        req_builder = req_builder.query(&[("skipRedownload", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_change_category {
        req_builder = req_builder.query(&[("changeCategory", &param_value.to_string())]);
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
    req_builder = req_builder.json(&p_queue_bulk_resource);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteQueueBulkError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_queue(configuration: &configuration::Configuration, page: Option<i32>, page_size: Option<i32>, sort_key: Option<&str>, sort_direction: Option<models::SortDirection>, include_unknown_series_items: Option<bool>, include_series: Option<bool>, include_episode: Option<bool>, series_ids: Option<Vec<i32>>, protocol: Option<models::DownloadProtocol>, languages: Option<Vec<i32>>, quality: Option<Vec<i32>>, status: Option<Vec<models::QueueStatus>>) -> Result<models::QueueResourcePagingResource, Error<GetQueueError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_page = page;
    let p_page_size = page_size;
    let p_sort_key = sort_key;
    let p_sort_direction = sort_direction;
    let p_include_unknown_series_items = include_unknown_series_items;
    let p_include_series = include_series;
    let p_include_episode = include_episode;
    let p_series_ids = series_ids;
    let p_protocol = protocol;
    let p_languages = languages;
    let p_quality = quality;
    let p_status = status;

    let uri_str = format!("{}/api/v3/queue", configuration.base_path);
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
    if let Some(ref param_value) = p_include_unknown_series_items {
        req_builder = req_builder.query(&[("includeUnknownSeriesItems", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_include_series {
        req_builder = req_builder.query(&[("includeSeries", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_include_episode {
        req_builder = req_builder.query(&[("includeEpisode", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_series_ids {
        req_builder = match "multi" {
            "multi" => req_builder.query(&param_value.into_iter().map(|p| ("seriesIds".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => req_builder.query(&[("seriesIds", &param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref param_value) = p_protocol {
        req_builder = req_builder.query(&[("protocol", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_languages {
        req_builder = match "multi" {
            "multi" => req_builder.query(&param_value.into_iter().map(|p| ("languages".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => req_builder.query(&[("languages", &param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref param_value) = p_quality {
        req_builder = match "multi" {
            "multi" => req_builder.query(&param_value.into_iter().map(|p| ("quality".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => req_builder.query(&[("quality", &param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref param_value) = p_status {
        req_builder = match "multi" {
            "multi" => req_builder.query(&param_value.into_iter().map(|p| ("status".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => req_builder.query(&[("status", &param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
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
        let entity: Option<GetQueueError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

