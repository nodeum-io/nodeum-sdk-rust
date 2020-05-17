# \TaskDestinationsApi

All URIs are relative to *http://localhost/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_task_destination**](TaskDestinationsApi.md#create_task_destination) | **Post** /tasks/{task_id}/task_destinations | Creates a new task destination.
[**destroy_task_destination**](TaskDestinationsApi.md#destroy_task_destination) | **Delete** /tasks/{task_id}/task_destinations/{task_destination_id} | Destroys a specific task destination.
[**index_task_destinations**](TaskDestinationsApi.md#index_task_destinations) | **Get** /tasks/{task_id}/task_destinations | Lists all task destinations.
[**show_task_destination**](TaskDestinationsApi.md#show_task_destination) | **Get** /tasks/{task_id}/task_destinations/{task_destination_id} | Displays a specific task destination.
[**update_task_destination**](TaskDestinationsApi.md#update_task_destination) | **Put** /tasks/{task_id}/task_destinations/{task_destination_id} | Updates a specific task destination.



## create_task_destination

> crate::models::TaskDestinationDown create_task_destination(task_id, task_destination_body)
Creates a new task destination.

**API Key Scope**: task_destinations / create

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**task_destination_body** | [**TaskDestinationUp**](TaskDestinationUp.md) |  | [required] |

### Return type

[**crate::models::TaskDestinationDown**](task_destination_down.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_task_destination

> destroy_task_destination(task_id, task_destination_id)
Destroys a specific task destination.

**API Key Scope**: task_destinations / destroy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**task_destination_id** | **i32** | Numeric ID of task destination. | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_task_destinations

> crate::models::TaskDestinationCollection index_task_destinations(task_id, limit, offset, sort_by, id, folder_id, tape_id, pool_id)
Lists all task destinations.

**API Key Scope**: task_destinations / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Sort results by attribute.  Can sort on multiple attributes, separated by `|`. Order direction can be suffixing the attribute by either `:asc` (default) or `:desc`. |  |
**id** | Option<**String**> | Filter on id |  |
**folder_id** | Option<**String**> | Filter on folder id |  |
**tape_id** | Option<**String**> | Filter on tape id |  |
**pool_id** | Option<**String**> | Filter on a pool id |  |

### Return type

[**crate::models::TaskDestinationCollection**](task_destination_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_task_destination

> crate::models::TaskDestinationDown show_task_destination(task_id, task_destination_id)
Displays a specific task destination.

**API Key Scope**: task_destinations / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**task_destination_id** | **i32** | Numeric ID of task destination. | [required] |

### Return type

[**crate::models::TaskDestinationDown**](task_destination_down.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_task_destination

> crate::models::TaskDestinationDown update_task_destination(task_id, task_destination_id, task_destination_body)
Updates a specific task destination.

**API Key Scope**: task_destinations / update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**task_destination_id** | **i32** | Numeric ID of task destination. | [required] |
**task_destination_body** | [**TaskDestinationUp**](TaskDestinationUp.md) |  | [required] |

### Return type

[**crate::models::TaskDestinationDown**](task_destination_down.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

