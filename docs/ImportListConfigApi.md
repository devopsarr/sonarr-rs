# \ImportListConfigApi

All URIs are relative to *http://localhost:8989*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_import_list_config**](ImportListConfigApi.md#get_import_list_config) | **GET** /api/v3/config/importlist | 
[**get_import_list_config_by_id**](ImportListConfigApi.md#get_import_list_config_by_id) | **GET** /api/v3/config/importlist/{id} | 
[**update_import_list_config**](ImportListConfigApi.md#update_import_list_config) | **PUT** /api/v3/config/importlist/{id} | 



## get_import_list_config

> models::ImportListConfigResource get_import_list_config()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ImportListConfigResource**](ImportListConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_import_list_config_by_id

> models::ImportListConfigResource get_import_list_config_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::ImportListConfigResource**](ImportListConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_import_list_config

> models::ImportListConfigResource update_import_list_config(id, import_list_config_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**import_list_config_resource** | Option<[**ImportListConfigResource**](ImportListConfigResource.md)> |  |  |

### Return type

[**models::ImportListConfigResource**](ImportListConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

