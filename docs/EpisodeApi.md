# \EpisodeApi

All URIs are relative to *http://localhost:8989*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_episode_by_id**](EpisodeApi.md#get_episode_by_id) | **GET** /api/v3/episode/{id} | 
[**list_episode**](EpisodeApi.md#list_episode) | **GET** /api/v3/episode | 
[**put_episode_monitor**](EpisodeApi.md#put_episode_monitor) | **PUT** /api/v3/episode/monitor | 
[**update_episode**](EpisodeApi.md#update_episode) | **PUT** /api/v3/episode/{id} | 



## get_episode_by_id

> models::EpisodeResource get_episode_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::EpisodeResource**](EpisodeResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_episode

> Vec<models::EpisodeResource> list_episode(series_id, season_number, episode_ids, episode_file_id, include_series, include_episode_file, include_images)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_id** | Option<**i32**> |  |  |
**season_number** | Option<**i32**> |  |  |
**episode_ids** | Option<[**Vec<i32>**](i32.md)> |  |  |
**episode_file_id** | Option<**i32**> |  |  |
**include_series** | Option<**bool**> |  |  |[default to false]
**include_episode_file** | Option<**bool**> |  |  |[default to false]
**include_images** | Option<**bool**> |  |  |[default to false]

### Return type

[**Vec<models::EpisodeResource>**](EpisodeResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_episode_monitor

> put_episode_monitor(include_images, episodes_monitored_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_images** | Option<**bool**> |  |  |[default to false]
**episodes_monitored_resource** | Option<[**EpisodesMonitoredResource**](EpisodesMonitoredResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_episode

> models::EpisodeResource update_episode(id, episode_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**episode_resource** | Option<[**EpisodeResource**](EpisodeResource.md)> |  |  |

### Return type

[**models::EpisodeResource**](EpisodeResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

