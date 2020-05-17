# \TaskCallbacksApi

All URIs are relative to *http://localhost/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_task_callback**](TaskCallbacksApi.md#create_task_callback) | **Post** /tasks/{task_id}/task_callbacks | Creates a new task callback.
[**destroy_task_callback**](TaskCallbacksApi.md#destroy_task_callback) | **Delete** /tasks/{task_id}/task_callbacks/{task_callback_id} | Destroys a specific task callback.
[**index_task_callbacks**](TaskCallbacksApi.md#index_task_callbacks) | **Get** /tasks/{task_id}/task_callbacks | Lists all task callbacks.
[**show_task_callback**](TaskCallbacksApi.md#show_task_callback) | **Get** /tasks/{task_id}/task_callbacks/{task_callback_id} | Displays a specific task callback.
[**update_task_callback**](TaskCallbacksApi.md#update_task_callback) | **Put** /tasks/{task_id}/task_callbacks/{task_callback_id} | Updates a specific task callback.



## create_task_callback

> crate::models::TaskCallback create_task_callback(task_id, task_callback_body)
Creates a new task callback.

**API Key Scope**: task_callbacks / create

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**task_callback_body** | [**TaskCallback**](TaskCallback.md) |  | [required] |

### Return type

[**crate::models::TaskCallback**](task_callback.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_task_callback

> destroy_task_callback(task_id, task_callback_id)
Destroys a specific task callback.

**API Key Scope**: task_callbacks / destroy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**task_callback_id** | **i32** | Numeric ID of task callback. | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_task_callbacks

> crate::models::TaskCallbackCollection index_task_callbacks(task_id, limit, offset, sort_by, id, _type, script)
Lists all task callbacks.

**API Key Scope**: task_callbacks / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Sort results by attribute.  Can sort on multiple attributes, separated by `|`. Order direction can be suffixing the attribute by either `:asc` (default) or `:desc`. |  |
**id** | Option<**String**> | Filter on id |  |
**_type** | Option<**String**> | Filter on type |  |
**script** | Option<**String**> | Filter on task callback script |  |

### Return type

[**crate::models::TaskCallbackCollection**](task_callback_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_task_callback

> crate::models::TaskCallback show_task_callback(task_id, task_callback_id)
Displays a specific task callback.

**API Key Scope**: task_callbacks / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**task_callback_id** | **i32** | Numeric ID of task callback. | [required] |

### Return type

[**crate::models::TaskCallback**](task_callback.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_task_callback

> crate::models::TaskCallback update_task_callback(task_id, task_callback_id, task_callback_body)
Updates a specific task callback.

**API Key Scope**: task_callbacks / update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**task_callback_id** | **i32** | Numeric ID of task callback. | [required] |
**task_callback_body** | [**TaskCallback**](TaskCallback.md) |  | [required] |

### Return type

[**crate::models::TaskCallback**](task_callback.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

