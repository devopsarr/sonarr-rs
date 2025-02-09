# \MissingApi

All URIs are relative to *http://localhost:8989*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_wanted_missing**](MissingApi.md#get_wanted_missing) | **GET** /api/v3/wanted/missing | 
[**get_wanted_missing_by_id**](MissingApi.md#get_wanted_missing_by_id) | **GET** /api/v3/wanted/missing/{id} | 



## get_wanted_missing

> models::EpisodeResourcePagingResource get_wanted_missing(page, page_size, sort_key, sort_direction, include_series, include_images, monitored)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**page_size** | Option<**i32**> |  |  |[default to 10]
**sort_key** | Option<**String**> |  |  |
**sort_direction** | Option<[**SortDirection**](.md)> |  |  |
**include_series** | Option<**bool**> |  |  |[default to false]
**include_images** | Option<**bool**> |  |  |[default to false]
**monitored** | Option<**bool**> |  |  |[default to true]

### Return type

[**models::EpisodeResourcePagingResource**](EpisodeResourcePagingResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_wanted_missing_by_id

> models::EpisodeResource get_wanted_missing_by_id(id)


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

