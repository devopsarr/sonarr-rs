# \LocalizationApi

All URIs are relative to *http://localhost:8989*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_localization**](LocalizationApi.md#get_localization) | **GET** /api/v3/localization | 
[**get_localization_by_id**](LocalizationApi.md#get_localization_by_id) | **GET** /api/v3/localization/{id} | 
[**get_localization_language**](LocalizationApi.md#get_localization_language) | **GET** /api/v3/localization/language | 



## get_localization

> models::LocalizationResource get_localization()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::LocalizationResource**](LocalizationResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_localization_by_id

> models::LocalizationResource get_localization_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::LocalizationResource**](LocalizationResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_localization_language

> models::LocalizationLanguageResource get_localization_language()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::LocalizationLanguageResource**](LocalizationLanguageResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

