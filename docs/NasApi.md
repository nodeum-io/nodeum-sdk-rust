# \NasApi

All URIs are relative to *http://localhost/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_nas**](NasApi.md#create_nas) | **Post** /nas | Creates a new NAS.
[**destroy_nas**](NasApi.md#destroy_nas) | **Delete** /nas/{nas_id} | Destroys a specific NAS.
[**index_nas**](NasApi.md#index_nas) | **Get** /nas | Lists all NAS.
[**show_nas**](NasApi.md#show_nas) | **Get** /nas/{nas_id} | Displays a specific NAS.
[**update_nas**](NasApi.md#update_nas) | **Put** /nas/{nas_id} | Updates a specific NAS.



## create_nas

> crate::models::Nas create_nas(nas_body)
Creates a new NAS.

**API Key Scope**: nas / create

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nas_body** | [**Nas**](Nas.md) |  | [required] |

### Return type

[**crate::models::Nas**](nas.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_nas

> destroy_nas(nas_id)
Destroys a specific NAS.

**API Key Scope**: nas / destroy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nas_id** | **String** | Numeric ID or name of NAS. | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_nas

> crate::models::NasCollection index_nas(limit, offset, sort_by, id, name, comment, host, _type, price)
Lists all NAS.

**API Key Scope**: nas / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Sort results by attribute.  Can sort on multiple attributes, separated by `|`. Order direction can be suffixing the attribute by either `:asc` (default) or `:desc`. |  |
**id** | Option<**String**> | Filter on id |  |
**name** | Option<**String**> | Filter on name |  |
**comment** | Option<**String**> | Filter on comment |  |
**host** | Option<**String**> | Filter on host |  |
**_type** | Option<**String**> | Filter on type |  |
**price** | Option<**String**> | Filter on price |  |

### Return type

[**crate::models::NasCollection**](nas_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_nas

> crate::models::Nas show_nas(nas_id)
Displays a specific NAS.

**API Key Scope**: nas / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nas_id** | **String** | Numeric ID or name of NAS. | [required] |

### Return type

[**crate::models::Nas**](nas.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_nas

> crate::models::Nas update_nas(nas_id, nas_body)
Updates a specific NAS.

**API Key Scope**: nas / update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nas_id** | **String** | Numeric ID or name of NAS. | [required] |
**nas_body** | [**Nas**](Nas.md) |  | [required] |

### Return type

[**crate::models::Nas**](nas.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

