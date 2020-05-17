# \TasksApi

All URIs are relative to *http://localhost/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_task**](TasksApi.md#create_task) | **Post** /tasks | Creates a new task.
[**destroy_task**](TasksApi.md#destroy_task) | **Delete** /tasks/{task_id} | Destroys a specific task.
[**index_tasks**](TasksApi.md#index_tasks) | **Get** /tasks | Lists all tasks.
[**pause_task**](TasksApi.md#pause_task) | **Put** /tasks/{task_id}/action/pause | Pause a task.
[**pause_task_result**](TasksApi.md#pause_task_result) | **Get** /tasks/{task_id}/action/pause | Check result of a task pause request.
[**resume_task**](TasksApi.md#resume_task) | **Put** /tasks/{task_id}/action/resume | Resume a task.
[**resume_task_result**](TasksApi.md#resume_task_result) | **Get** /tasks/{task_id}/action/resume | Check result of a task resume request.
[**run_task**](TasksApi.md#run_task) | **Put** /tasks/{task_id}/action/run | Run a task.
[**run_task_result**](TasksApi.md#run_task_result) | **Get** /tasks/{task_id}/action/run | Check result of a task run request.
[**show_task**](TasksApi.md#show_task) | **Get** /tasks/{task_id} | Displays a specific task.
[**stop_task**](TasksApi.md#stop_task) | **Put** /tasks/{task_id}/action/stop | Stop a task.
[**stop_task_result**](TasksApi.md#stop_task_result) | **Get** /tasks/{task_id}/action/stop | Check result of a task stop request.
[**update_task**](TasksApi.md#update_task) | **Put** /tasks/{task_id} | Updates a specific task.



## create_task

> crate::models::Task create_task(task_body)
Creates a new task.

**API Key Scope**: tasks / create 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_body** | [**Task**](Task.md) |  | [required] |

### Return type

[**crate::models::Task**](task.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_task

> destroy_task(task_id)
Destroys a specific task.

**API Key Scope**: tasks / destroy

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


## index_tasks

> crate::models::TaskCollection index_tasks(limit, offset, sort_by, id, name, comment, workflow_type, workflow_action, source_type, destination_type, priority, conflict_resolution, action, activate, creation_date, creation_username, modification_date, modification_username, job_started, job_finished, status, processed_size, to_process_size, source_pool_id, source_pool_name, source_tape_id, source_tape_barcode, destination_pool_id, destination_pool_name, destination_tape_id, destination_tape_barcode)
Lists all tasks.

**API Key Scope**: tasks / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Sort results by attribute.  Can sort on multiple attributes, separated by `|`. Order direction can be suffixing the attribute by either `:asc` (default) or `:desc`. |  |
**id** | Option<**String**> | Filter on id |  |
**name** | Option<**String**> | Filter on name |  |
**comment** | Option<**String**> | Filter on comment |  |
**workflow_type** | Option<**String**> | Filter on task workflow type |  |
**workflow_action** | Option<**String**> | Filter on task workflow action |  |
**source_type** | Option<**String**> | Filter on task source type |  |
**destination_type** | Option<**String**> | Filter on task destination type |  |
**priority** | Option<**String**> | Filter on priority |  |
**conflict_resolution** | Option<**String**> | Filter on conflict resolution |  |
**action** | Option<**String**> | Filter on action |  |
**activate** | Option<**String**> | Filter on activate |  |
**creation_date** | Option<**String**> | Filter on creation date |  |
**creation_username** | Option<**String**> | Filter on creation username |  |
**modification_date** | Option<**String**> | Filter on modification date |  |
**modification_username** | Option<**String**> | Filter on modification username |  |
**job_started** | Option<**String**> | Filter on job started |  |
**job_finished** | Option<**String**> | Filter on job finished |  |
**status** | Option<**String**> | Filter on status |  |
**processed_size** | Option<**String**> | Filter on processed size |  |
**to_process_size** | Option<**String**> | Filter on to process size |  |
**source_pool_id** | Option<**String**> | Filter on task source pool id |  |
**source_pool_name** | Option<**String**> | Filter on task source pool name |  |
**source_tape_id** | Option<**String**> | Filter on task source tape id |  |
**source_tape_barcode** | Option<**String**> | Filter on task source tape barcode |  |
**destination_pool_id** | Option<**String**> | Filter on task destination pool id |  |
**destination_pool_name** | Option<**String**> | Filter on task destination pool name |  |
**destination_tape_id** | Option<**String**> | Filter on task destination tape id |  |
**destination_tape_barcode** | Option<**String**> | Filter on task destination tape barcode |  |

### Return type

[**crate::models::TaskCollection**](task_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pause_task

> crate::models::ActiveJobStatus pause_task(task_id)
Pause a task.

**API Key Scope**: tasks / action

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |

### Return type

[**crate::models::ActiveJobStatus**](active_job_status.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, queued, working, failed

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pause_task_result

> crate::models::ActiveJobStatus pause_task_result(task_id, job_id)
Check result of a task pause request.

**API Key Scope**: tasks / action

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**job_id** | **String** | ID of active job | [required] |

### Return type

[**crate::models::ActiveJobStatus**](active_job_status.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, queued, working, failed

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resume_task

> crate::models::ActiveJobStatus resume_task(task_id)
Resume a task.

**API Key Scope**: tasks / action

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |

### Return type

[**crate::models::ActiveJobStatus**](active_job_status.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, queued, working, failed

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resume_task_result

> crate::models::ActiveJobStatus resume_task_result(task_id, job_id)
Check result of a task resume request.

**API Key Scope**: tasks / action

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**job_id** | **String** | ID of active job | [required] |

### Return type

[**crate::models::ActiveJobStatus**](active_job_status.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, queued, working, failed

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## run_task

> crate::models::ActiveJobStatus run_task(task_id)
Run a task.

**API Key Scope**: tasks / action

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |

### Return type

[**crate::models::ActiveJobStatus**](active_job_status.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, queued, working, failed

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## run_task_result

> crate::models::ActiveJobStatus run_task_result(task_id, job_id)
Check result of a task run request.

**API Key Scope**: tasks / action

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**job_id** | **String** | ID of active job | [required] |

### Return type

[**crate::models::ActiveJobStatus**](active_job_status.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, queued, working, failed

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_task

> crate::models::Task show_task(task_id)
Displays a specific task.

**API Key Scope**: tasks / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |

### Return type

[**crate::models::Task**](task.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_task

> crate::models::ActiveJobStatus stop_task(task_id)
Stop a task.

**API Key Scope**: tasks / action

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |

### Return type

[**crate::models::ActiveJobStatus**](active_job_status.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, queued, working, failed

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_task_result

> crate::models::ActiveJobStatus stop_task_result(task_id, job_id)
Check result of a task stop request.

**API Key Scope**: tasks / action

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**job_id** | **String** | ID of active job | [required] |

### Return type

[**crate::models::ActiveJobStatus**](active_job_status.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, queued, working, failed

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_task

> crate::models::Task update_task(task_id, task_body)
Updates a specific task.

**API Key Scope**: tasks / update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**task_body** | [**Task**](Task.md) |  | [required] |

### Return type

[**crate::models::Task**](task.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

