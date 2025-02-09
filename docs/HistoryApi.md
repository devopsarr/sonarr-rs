# \HistoryApi

All URIs are relative to *http://localhost:8989*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_history_failed_by_id**](HistoryApi.md#create_history_failed_by_id) | **POST** /api/v3/history/failed/{id} | 
[**get_history**](HistoryApi.md#get_history) | **GET** /api/v3/history | 
[**list_history_series**](HistoryApi.md#list_history_series) | **GET** /api/v3/history/series | 
[**list_history_since**](HistoryApi.md#list_history_since) | **GET** /api/v3/history/since | 



## create_history_failed_by_id

> create_history_failed_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_history

> models::HistoryResourcePagingResource get_history(page, page_size, sort_key, sort_direction, include_series, include_episode, event_type, episode_id, download_id, series_ids, languages, quality)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**page_size** | Option<**i32**> |  |  |[default to 10]
**sort_key** | Option<**String**> |  |  |
**sort_direction** | Option<[**SortDirection**](.md)> |  |  |
**include_series** | Option<**bool**> |  |  |
**include_episode** | Option<**bool**> |  |  |
**event_type** | Option<[**Vec<i32>**](i32.md)> |  |  |
**episode_id** | Option<**i32**> |  |  |
**download_id** | Option<**String**> |  |  |
**series_ids** | Option<[**Vec<i32>**](i32.md)> |  |  |
**languages** | Option<[**Vec<i32>**](i32.md)> |  |  |
**quality** | Option<[**Vec<i32>**](i32.md)> |  |  |

### Return type

[**models::HistoryResourcePagingResource**](HistoryResourcePagingResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_history_series

> Vec<models::HistoryResource> list_history_series(series_id, season_number, event_type, include_series, include_episode)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_id** | Option<**i32**> |  |  |
**season_number** | Option<**i32**> |  |  |
**event_type** | Option<[**EpisodeHistoryEventType**](.md)> |  |  |
**include_series** | Option<**bool**> |  |  |[default to false]
**include_episode** | Option<**bool**> |  |  |[default to false]

### Return type

[**Vec<models::HistoryResource>**](HistoryResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_history_since

> Vec<models::HistoryResource> list_history_since(date, event_type, include_series, include_episode)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date** | Option<**String**> |  |  |
**event_type** | Option<[**EpisodeHistoryEventType**](.md)> |  |  |
**include_series** | Option<**bool**> |  |  |[default to false]
**include_episode** | Option<**bool**> |  |  |[default to false]

### Return type

[**Vec<models::HistoryResource>**](HistoryResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

