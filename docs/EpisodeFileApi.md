# \EpisodeFileApi

All URIs are relative to *http://localhost:8989*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_episode_file**](EpisodeFileApi.md#delete_episode_file) | **DELETE** /api/v3/episodefile/{id} | 
[**delete_episode_file_bulk**](EpisodeFileApi.md#delete_episode_file_bulk) | **DELETE** /api/v3/episodefile/bulk | 
[**get_episode_file_by_id**](EpisodeFileApi.md#get_episode_file_by_id) | **GET** /api/v3/episodefile/{id} | 
[**list_episode_file**](EpisodeFileApi.md#list_episode_file) | **GET** /api/v3/episodefile | 
[**put_episode_file_bulk**](EpisodeFileApi.md#put_episode_file_bulk) | **PUT** /api/v3/episodefile/bulk | 
[**put_episode_file_editor**](EpisodeFileApi.md#put_episode_file_editor) | **PUT** /api/v3/episodefile/editor | 
[**update_episode_file**](EpisodeFileApi.md#update_episode_file) | **PUT** /api/v3/episodefile/{id} | 



## delete_episode_file

> delete_episode_file(id)


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


## delete_episode_file_bulk

> delete_episode_file_bulk(episode_file_list_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**episode_file_list_resource** | Option<[**EpisodeFileListResource**](EpisodeFileListResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_episode_file_by_id

> models::EpisodeFileResource get_episode_file_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::EpisodeFileResource**](EpisodeFileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_episode_file

> Vec<models::EpisodeFileResource> list_episode_file(series_id, episode_file_ids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_id** | Option<**i32**> |  |  |
**episode_file_ids** | Option<[**Vec<i32>**](i32.md)> |  |  |

### Return type

[**Vec<models::EpisodeFileResource>**](EpisodeFileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_episode_file_bulk

> put_episode_file_bulk(episode_file_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**episode_file_resource** | Option<[**Vec<models::EpisodeFileResource>**](EpisodeFileResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_episode_file_editor

> put_episode_file_editor(episode_file_list_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**episode_file_list_resource** | Option<[**EpisodeFileListResource**](EpisodeFileListResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_episode_file

> models::EpisodeFileResource update_episode_file(id, episode_file_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**episode_file_resource** | Option<[**EpisodeFileResource**](EpisodeFileResource.md)> |  |  |

### Return type

[**models::EpisodeFileResource**](EpisodeFileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

