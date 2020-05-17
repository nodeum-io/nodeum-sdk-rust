# \TaskOptionsApi

All URIs are relative to *http://localhost/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_task_option**](TaskOptionsApi.md#create_task_option) | **Post** /tasks/{task_id}/task_options | Creates a new task option.
[**destroy_task_option**](TaskOptionsApi.md#destroy_task_option) | **Delete** /tasks/{task_id}/task_options/{task_option_id} | Destroys a specific task option.
[**index_task_options**](TaskOptionsApi.md#index_task_options) | **Get** /tasks/{task_id}/task_options | Lists all task options.
[**show_task_option**](TaskOptionsApi.md#show_task_option) | **Get** /tasks/{task_id}/task_options/{task_option_id} | Displays a specific task option.
[**update_task_option**](TaskOptionsApi.md#update_task_option) | **Put** /tasks/{task_id}/task_options/{task_option_id} | Updates a specific task option.



## create_task_option

> crate::models::TaskOption create_task_option(task_id, task_option_body)
Creates a new task option.

**API Key Scope**: task_options / create

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**task_option_body** | [**TaskOption**](TaskOption.md) |  | [required] |

### Return type

[**crate::models::TaskOption**](task_option.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_task_option

> destroy_task_option(task_id, task_option_id)
Destroys a specific task option.

**API Key Scope**: task_options / destroy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**task_option_id** | **i32** | Numeric ID of task option. | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_task_options

> crate::models::TaskOptionCollection index_task_options(task_id, limit, offset, sort_by, id, _type, value)
Lists all task options.

**API Key Scope**: task_options / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Sort results by attribute.  Can sort on multiple attributes, separated by `|`. Order direction can be suffixing the attribute by either `:asc` (default) or `:desc`. |  |
**id** | Option<**String**> | Filter on id |  |
**_type** | Option<**String**> | Filter on type |  |
**value** | Option<**String**> | Filter on value |  |

### Return type

[**crate::models::TaskOptionCollection**](task_option_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_task_option

> crate::models::TaskOption show_task_option(task_id, task_option_id)
Displays a specific task option.

**API Key Scope**: task_options / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**task_option_id** | **i32** | Numeric ID of task option. | [required] |

### Return type

[**crate::models::TaskOption**](task_option.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_task_option

> crate::models::TaskOption update_task_option(task_id, task_option_id, task_option_body)
Updates a specific task option.

**API Key Scope**: task_options / update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**task_option_id** | **i32** | Numeric ID of task option. | [required] |
**task_option_body** | [**TaskOption**](TaskOption.md) |  | [required] |

### Return type

[**crate::models::TaskOption**](task_option.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

