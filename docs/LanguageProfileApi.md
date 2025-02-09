# \LanguageProfileApi

All URIs are relative to *http://localhost:8989*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_language_profile**](LanguageProfileApi.md#create_language_profile) | **POST** /api/v3/languageprofile | 
[**delete_language_profile**](LanguageProfileApi.md#delete_language_profile) | **DELETE** /api/v3/languageprofile/{id} | 
[**get_language_profile_by_id**](LanguageProfileApi.md#get_language_profile_by_id) | **GET** /api/v3/languageprofile/{id} | 
[**list_language_profile**](LanguageProfileApi.md#list_language_profile) | **GET** /api/v3/languageprofile | 
[**update_language_profile**](LanguageProfileApi.md#update_language_profile) | **PUT** /api/v3/languageprofile/{id} | 



## create_language_profile

> models::LanguageProfileResource create_language_profile(language_profile_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language_profile_resource** | Option<[**LanguageProfileResource**](LanguageProfileResource.md)> |  |  |

### Return type

[**models::LanguageProfileResource**](LanguageProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_language_profile

> delete_language_profile(id)


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


## get_language_profile_by_id

> models::LanguageProfileResource get_language_profile_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::LanguageProfileResource**](LanguageProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_language_profile

> Vec<models::LanguageProfileResource> list_language_profile()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::LanguageProfileResource>**](LanguageProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_language_profile

> models::LanguageProfileResource update_language_profile(id, language_profile_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**language_profile_resource** | Option<[**LanguageProfileResource**](LanguageProfileResource.md)> |  |  |

### Return type

[**models::LanguageProfileResource**](LanguageProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

