# \TaskSchedulesApi

All URIs are relative to *http://localhost/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_task_schedule**](TaskSchedulesApi.md#create_task_schedule) | **Post** /tasks/{task_id}/task_schedule | Creates a new task schedule. Only one should be created.
[**destroy_task_schedule**](TaskSchedulesApi.md#destroy_task_schedule) | **Delete** /tasks/{task_id}/task_schedule | Destroys the task schedule.
[**index_task_schedules**](TaskSchedulesApi.md#index_task_schedules) | **Get** /task_schedules | Lists all task schedules.
[**show_task_schedule**](TaskSchedulesApi.md#show_task_schedule) | **Get** /tasks/{task_id}/task_schedule | Displays the task schedule.
[**update_task_schedule**](TaskSchedulesApi.md#update_task_schedule) | **Put** /tasks/{task_id}/task_schedule | Updates the existing task schedule.



## create_task_schedule

> crate::models::TaskSchedule create_task_schedule(task_id, task_schedule_body)
Creates a new task schedule. Only one should be created.

**API Key Scope**: task_schedules / create

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**task_schedule_body** | [**TaskSchedule**](TaskSchedule.md) |  | [required] |

### Return type

[**crate::models::TaskSchedule**](task_schedule.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_task_schedule

> destroy_task_schedule(task_id)
Destroys the task schedule.

**API Key Scope**: task_schedules / destroy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_task_schedules

> crate::models::TaskScheduleCollection index_task_schedules(with_next, limit, offset, sort_by, id, rrule, done, task_id)
Lists all task schedules.

**API Key Scope**: task_schedules / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**with_next** | Option<**bool**> | Display the next scheduled date, and information about missing executions. |  |[default to true]
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Sort results by attribute.  Can sort on multiple attributes, separated by `|`. Order direction can be suffixing the attribute by either `:asc` (default) or `:desc`. |  |
**id** | Option<**String**> | Filter on id |  |
**rrule** | Option<**String**> | Filter on RRule of schedule |  |
**done** | Option<**String**> | Filter on done schedule |  |
**task_id** | Option<**String**> | Filter on task id |  |

### Return type

[**crate::models::TaskScheduleCollection**](task_schedule_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_task_schedule

> crate::models::TaskSchedule show_task_schedule(task_id)
Displays the task schedule.

**API Key Scope**: task_schedules / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |

### Return type

[**crate::models::TaskSchedule**](task_schedule.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_task_schedule

> crate::models::TaskSchedule update_task_schedule(task_id, task_schedule_body)
Updates the existing task schedule.

**API Key Scope**: task_schedules / update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**task_schedule_body** | [**TaskSchedule**](TaskSchedule.md) |  | [required] |

### Return type

[**crate::models::TaskSchedule**](task_schedule.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

