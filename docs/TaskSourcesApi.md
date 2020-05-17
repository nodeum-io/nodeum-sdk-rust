# \TaskSourcesApi

All URIs are relative to *http://localhost/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_task_source**](TaskSourcesApi.md#create_task_source) | **Post** /tasks/{task_id}/task_sources | Creates a new task source.
[**destroy_task_source**](TaskSourcesApi.md#destroy_task_source) | **Delete** /tasks/{task_id}/task_sources/{task_source_id} | Destroys a specific task source.
[**index_task_sources**](TaskSourcesApi.md#index_task_sources) | **Get** /tasks/{task_id}/task_sources | Lists all task sources.
[**show_task_source**](TaskSourcesApi.md#show_task_source) | **Get** /tasks/{task_id}/task_sources/{task_source_id} | Displays a specific task source.
[**update_task_source**](TaskSourcesApi.md#update_task_source) | **Put** /tasks/{task_id}/task_sources/{task_source_id} | Updates a specific task source.



## create_task_source

> crate::models::TaskSourceDown create_task_source(task_id, task_source_body)
Creates a new task source.

**API Key Scope**: task_sources / create

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**task_source_body** | [**TaskSourceUp**](TaskSourceUp.md) |  | [required] |

### Return type

[**crate::models::TaskSourceDown**](task_source_down.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_task_source

> destroy_task_source(task_id, task_source_id)
Destroys a specific task source.

**API Key Scope**: task_sources / destroy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**task_source_id** | **i32** | Numeric ID of task source. | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_task_sources

> crate::models::TaskSourceCollection index_task_sources(task_id, limit, offset, sort_by, id, file_id, import_file_id, tape_id, pool_id)
Lists all task sources.

**API Key Scope**: task_sources / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Sort results by attribute.  Can sort on multiple attributes, separated by `|`. Order direction can be suffixing the attribute by either `:asc` (default) or `:desc`. |  |
**id** | Option<**String**> | Filter on id |  |
**file_id** | Option<**String**> | Filter on file id |  |
**import_file_id** | Option<**String**> | Filter on import file id |  |
**tape_id** | Option<**String**> | Filter on tape id |  |
**pool_id** | Option<**String**> | Filter on a pool id |  |

### Return type

[**crate::models::TaskSourceCollection**](task_source_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_task_source

> crate::models::TaskSourceDown show_task_source(task_id, task_source_id)
Displays a specific task source.

**API Key Scope**: task_sources / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**task_source_id** | **i32** | Numeric ID of task source. | [required] |

### Return type

[**crate::models::TaskSourceDown**](task_source_down.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_task_source

> crate::models::TaskSourceDown update_task_source(task_id, task_source_id, task_source_body)
Updates a specific task source.

**API Key Scope**: task_sources / update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**task_source_id** | **i32** | Numeric ID of task source. | [required] |
**task_source_body** | [**TaskSourceUp**](TaskSourceUp.md) |  | [required] |

### Return type

[**crate::models::TaskSourceDown**](task_source_down.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

