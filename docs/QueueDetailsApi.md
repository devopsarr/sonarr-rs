# \QueueDetailsApi

All URIs are relative to *http://localhost:8989*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_queue_details**](QueueDetailsApi.md#list_queue_details) | **GET** /api/v3/queue/details | 



## list_queue_details

> Vec<models::QueueResource> list_queue_details(series_id, episode_ids, include_series, include_episode)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_id** | Option<**i32**> |  |  |
**episode_ids** | Option<[**Vec<i32>**](i32.md)> |  |  |
**include_series** | Option<**bool**> |  |  |[default to false]
**include_episode** | Option<**bool**> |  |  |[default to false]

### Return type

[**Vec<models::QueueResource>**](QueueResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

