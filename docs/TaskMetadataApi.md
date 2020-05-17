# \TaskMetadataApi

All URIs are relative to *http://localhost/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_task_metadatum**](TaskMetadataApi.md#create_task_metadatum) | **Post** /tasks/{task_id}/task_metadata | Creates a new task metadatum.
[**destroy_task_metadatum**](TaskMetadataApi.md#destroy_task_metadatum) | **Delete** /tasks/{task_id}/task_metadata/{task_metadatum_id} | Destroys a specific task metadatum.
[**index_task_metadata**](TaskMetadataApi.md#index_task_metadata) | **Get** /tasks/{task_id}/task_metadata | Lists all task metadata.
[**show_task_metadatum**](TaskMetadataApi.md#show_task_metadatum) | **Get** /tasks/{task_id}/task_metadata/{task_metadatum_id} | Displays a specific task metadatum.
[**update_task_metadatum**](TaskMetadataApi.md#update_task_metadatum) | **Put** /tasks/{task_id}/task_metadata/{task_metadatum_id} | Updates a specific task metadatum.



## create_task_metadatum

> crate::models::TaskMetadatum create_task_metadatum(task_id, task_metadatum_body)
Creates a new task metadatum.

**API Key Scope**: task_metadata / create

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**task_metadatum_body** | [**TaskMetadatum**](TaskMetadatum.md) |  | [required] |

### Return type

[**crate::models::TaskMetadatum**](task_metadatum.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_task_metadatum

> destroy_task_metadatum(task_id, task_metadatum_id)
Destroys a specific task metadatum.

**API Key Scope**: task_metadata / destroy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**task_metadatum_id** | **i32** | Numeric ID of task metadatum. | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_task_metadata

> crate::models::TaskMetadatumCollection index_task_metadata(task_id, limit, offset, sort_by, id, key, value)
Lists all task metadata.

**API Key Scope**: task_metadata / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Sort results by attribute.  Can sort on multiple attributes, separated by `|`. Order direction can be suffixing the attribute by either `:asc` (default) or `:desc`. |  |
**id** | Option<**String**> | Filter on id |  |
**key** | Option<**String**> | Filter on key |  |
**value** | Option<**String**> | Filter on value |  |

### Return type

[**crate::models::TaskMetadatumCollection**](task_metadatum_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_task_metadatum

> crate::models::TaskMetadatum show_task_metadatum(task_id, task_metadatum_id)
Displays a specific task metadatum.

**API Key Scope**: task_metadata / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**task_metadatum_id** | **i32** | Numeric ID of task metadatum. | [required] |

### Return type

[**crate::models::TaskMetadatum**](task_metadatum.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_task_metadatum

> crate::models::TaskMetadatum update_task_metadatum(task_id, task_metadatum_id, task_metadatum_body)
Updates a specific task metadatum.

**API Key Scope**: task_metadata / update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**task_metadatum_id** | **i32** | Numeric ID of task metadatum. | [required] |
**task_metadatum_body** | [**TaskMetadatum**](TaskMetadatum.md) |  | [required] |

### Return type

[**crate::models::TaskMetadatum**](task_metadatum.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

