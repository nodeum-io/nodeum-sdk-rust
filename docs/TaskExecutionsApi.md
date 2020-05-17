# \TaskExecutionsApi

All URIs are relative to *http://localhost/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**index_task_executions**](TaskExecutionsApi.md#index_task_executions) | **Get** /task_executions | Lists all task executions.
[**index_task_executions_by_task**](TaskExecutionsApi.md#index_task_executions_by_task) | **Get** /tasks/{task_id}/task_executions | Lists all task executions.
[**show_task_execution**](TaskExecutionsApi.md#show_task_execution) | **Get** /task_executions/{task_execution_id} | Displays a specific task execution.
[**show_task_execution_by_task**](TaskExecutionsApi.md#show_task_execution_by_task) | **Get** /tasks/{task_id}/task_executions/{task_execution_id} | Displays a specific task execution.



## index_task_executions

> crate::models::TaskExecutionCollection index_task_executions(limit, offset, sort_by, complete_list, id, task_id, name, workflow_type, workflow_action, source_type, destination_type, status, log_time, job_started, job_finished, creation_date, modification_date, to_process_size, processed_size, to_process_files, processed_files, finalized_files, estimation_time, bandwidth)
Lists all task executions.

**API Key Scope**: task_executions / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Sort results by attribute.  Can sort on multiple attributes, separated by `|`. Order direction can be suffixing the attribute by either `:asc` (default) or `:desc`. |  |
**complete_list** | Option<**bool**> | If `false`, only includes the last correct execution of a task. |  |[default to true]
**id** | Option<**String**> | Filter on id |  |
**task_id** | Option<**String**> | Filter on task id |  |
**name** | Option<**String**> | Filter on name |  |
**workflow_type** | Option<**String**> | Filter on task workflow type |  |
**workflow_action** | Option<**String**> | Filter on task workflow action |  |
**source_type** | Option<**String**> | Filter on task source type |  |
**destination_type** | Option<**String**> | Filter on task destination type |  |
**status** | Option<**String**> | Filter on status |  |
**log_time** | Option<**String**> | Filter on log time |  |
**job_started** | Option<**String**> | Filter on job started |  |
**job_finished** | Option<**String**> | Filter on job finished |  |
**creation_date** | Option<**String**> | Filter on creation date |  |
**modification_date** | Option<**String**> | Filter on modification date |  |
**to_process_size** | Option<**String**> | Filter on to process size |  |
**processed_size** | Option<**String**> | Filter on processed size |  |
**to_process_files** | Option<**String**> | Filter on to process files |  |
**processed_files** | Option<**String**> | Filter on processed files |  |
**finalized_files** | Option<**String**> | Filter on finalized files |  |
**estimation_time** | Option<**String**> | Filter on estimation time |  |
**bandwidth** | Option<**String**> | Filter on bandwidth |  |

### Return type

[**crate::models::TaskExecutionCollection**](task_execution_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_task_executions_by_task

> crate::models::TaskExecutionCollection index_task_executions_by_task(task_id, limit, offset, sort_by, id, name, workflow_type, workflow_action, source_type, destination_type, status, log_time, job_started, job_finished, to_process_size, processed_size, to_process_files, processed_files, finalized_files, estimation_time, bandwidth)
Lists all task executions.

**API Key Scope**: task_executions / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | Option<**String**> | Filter on task id |  |
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Sort results by attribute.  Can sort on multiple attributes, separated by `|`. Order direction can be suffixing the attribute by either `:asc` (default) or `:desc`. |  |
**id** | Option<**String**> | Filter on id |  |
**name** | Option<**String**> | Filter on name |  |
**workflow_type** | Option<**String**> | Filter on task workflow type |  |
**workflow_action** | Option<**String**> | Filter on task workflow action |  |
**source_type** | Option<**String**> | Filter on task source type |  |
**destination_type** | Option<**String**> | Filter on task destination type |  |
**status** | Option<**String**> | Filter on status |  |
**log_time** | Option<**String**> | Filter on log time |  |
**job_started** | Option<**String**> | Filter on job started |  |
**job_finished** | Option<**String**> | Filter on job finished |  |
**to_process_size** | Option<**String**> | Filter on to process size |  |
**processed_size** | Option<**String**> | Filter on processed size |  |
**to_process_files** | Option<**String**> | Filter on to process files |  |
**processed_files** | Option<**String**> | Filter on processed files |  |
**finalized_files** | Option<**String**> | Filter on finalized files |  |
**estimation_time** | Option<**String**> | Filter on estimation time |  |
**bandwidth** | Option<**String**> | Filter on bandwidth |  |

### Return type

[**crate::models::TaskExecutionCollection**](task_execution_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_task_execution

> crate::models::TaskExecution show_task_execution(task_execution_id)
Displays a specific task execution.

**API Key Scope**: task_executions / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_execution_id** | **String** | Numeric ID of task execution. | [required] |

### Return type

[**crate::models::TaskExecution**](task_execution.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_task_execution_by_task

> crate::models::TaskExecution show_task_execution_by_task(task_id, task_execution_id)
Displays a specific task execution.

**API Key Scope**: task_executions / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**task_execution_id** | **String** | Numeric ID of task execution. | [required] |

### Return type

[**crate::models::TaskExecution**](task_execution.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

