# \ImportListExclusionApi

All URIs are relative to *http://localhost:8989*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_import_list_exclusion**](ImportListExclusionApi.md#create_import_list_exclusion) | **POST** /api/v3/importlistexclusion | 
[**delete_import_list_exclusion**](ImportListExclusionApi.md#delete_import_list_exclusion) | **DELETE** /api/v3/importlistexclusion/{id} | 
[**delete_import_list_exclusion_bulk**](ImportListExclusionApi.md#delete_import_list_exclusion_bulk) | **DELETE** /api/v3/importlistexclusion/bulk | 
[**get_import_list_exclusion_by_id**](ImportListExclusionApi.md#get_import_list_exclusion_by_id) | **GET** /api/v3/importlistexclusion/{id} | 
[**get_import_list_exclusion_paged**](ImportListExclusionApi.md#get_import_list_exclusion_paged) | **GET** /api/v3/importlistexclusion/paged | 
[**list_import_list_exclusion**](ImportListExclusionApi.md#list_import_list_exclusion) | **GET** /api/v3/importlistexclusion | 
[**update_import_list_exclusion**](ImportListExclusionApi.md#update_import_list_exclusion) | **PUT** /api/v3/importlistexclusion/{id} | 



## create_import_list_exclusion

> models::ImportListExclusionResource create_import_list_exclusion(import_list_exclusion_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**import_list_exclusion_resource** | Option<[**ImportListExclusionResource**](ImportListExclusionResource.md)> |  |  |

### Return type

[**models::ImportListExclusionResource**](ImportListExclusionResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_import_list_exclusion

> delete_import_list_exclusion(id)


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


## delete_import_list_exclusion_bulk

> delete_import_list_exclusion_bulk(import_list_exclusion_bulk_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**import_list_exclusion_bulk_resource** | Option<[**ImportListExclusionBulkResource**](ImportListExclusionBulkResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_import_list_exclusion_by_id

> models::ImportListExclusionResource get_import_list_exclusion_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::ImportListExclusionResource**](ImportListExclusionResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_import_list_exclusion_paged

> models::ImportListExclusionResourcePagingResource get_import_list_exclusion_paged(page, page_size, sort_key, sort_direction)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**page_size** | Option<**i32**> |  |  |[default to 10]
**sort_key** | Option<**String**> |  |  |
**sort_direction** | Option<[**SortDirection**](.md)> |  |  |

### Return type

[**models::ImportListExclusionResourcePagingResource**](ImportListExclusionResourcePagingResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_import_list_exclusion

> Vec<models::ImportListExclusionResource> list_import_list_exclusion()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ImportListExclusionResource>**](ImportListExclusionResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_import_list_exclusion

> models::ImportListExclusionResource update_import_list_exclusion(id, import_list_exclusion_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**import_list_exclusion_resource** | Option<[**ImportListExclusionResource**](ImportListExclusionResource.md)> |  |  |

### Return type

[**models::ImportListExclusionResource**](ImportListExclusionResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

