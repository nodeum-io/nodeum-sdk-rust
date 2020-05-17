# \FilesApi

All URIs are relative to *http://localhost/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**files_children**](FilesApi.md#files_children) | **Get** /files/{file_parent_id}/children | Lists files under a specific folder.
[**files_children_by_container**](FilesApi.md#files_children_by_container) | **Get** /containers/{container_id}/files/{file_parent_id}/children | Lists files under a specific folder.
[**files_children_by_pool**](FilesApi.md#files_children_by_pool) | **Get** /pools/{pool_id}/files/{file_parent_id}/children | Lists files under a specific folder.
[**files_children_by_task**](FilesApi.md#files_children_by_task) | **Get** /tasks/{task_id}/files/{file_parent_id}/children | Lists files under a specific folder.
[**files_children_by_task_execution**](FilesApi.md#files_children_by_task_execution) | **Get** /task_executions/{task_execution_id}/files/{file_parent_id}/children | Lists files under a specific folder.
[**files_children_by_task_execution_by_task**](FilesApi.md#files_children_by_task_execution_by_task) | **Get** /tasks/{task_id}/task_executions/{task_execution_id}/files/{file_parent_id}/children | Lists files under a specific folder.
[**import_files_children_by_pool**](FilesApi.md#import_files_children_by_pool) | **Get** /pools/{pool_id}/import_files/{file_parent_id}/children | Lists files under a specific folder on tape of pools, specific for Data Exchange.
[**index_files**](FilesApi.md#index_files) | **Get** /files | Lists files on root.
[**index_files_by_container**](FilesApi.md#index_files_by_container) | **Get** /containers/{container_id}/files | Lists files on root.
[**index_files_by_pool**](FilesApi.md#index_files_by_pool) | **Get** /pools/{pool_id}/files | Lists files on root.
[**index_files_by_task**](FilesApi.md#index_files_by_task) | **Get** /tasks/{task_id}/files | Lists files on root.
[**index_files_by_task_execution**](FilesApi.md#index_files_by_task_execution) | **Get** /task_executions/{task_execution_id}/files | Lists files on root.
[**index_files_by_task_execution_by_task**](FilesApi.md#index_files_by_task_execution_by_task) | **Get** /tasks/{task_id}/task_executions/{task_execution_id}/files | Lists files on root.
[**index_import_files_by_pool**](FilesApi.md#index_import_files_by_pool) | **Get** /pools/{pool_id}/import_files | Lists files on root of tape of pools, specific for Data Exchange.
[**index_on_tapes_files_by_pool**](FilesApi.md#index_on_tapes_files_by_pool) | **Get** /pools/{pool_id}/on_tapes_files | Lists files on root of tape of pools, specific for Active and Offline.
[**index_tapes_by_file_by_pool**](FilesApi.md#index_tapes_by_file_by_pool) | **Get** /pools/{pool_id}/files/{file_id}/tapes | Displays tapes containing specific file, related to the specific pool.
[**index_tapes_by_file_by_task**](FilesApi.md#index_tapes_by_file_by_task) | **Get** /tasks/{task_id}/files/{file_id}/tapes | Displays tapes containing specific file, related to the specific task.
[**index_tapes_by_file_by_task_execution**](FilesApi.md#index_tapes_by_file_by_task_execution) | **Get** /task_executions/{task_execution_id}/files/{file_id}/tapes | Displays tapes containing specific file, related to the specific task.
[**index_tapes_by_file_by_task_execution_by_task**](FilesApi.md#index_tapes_by_file_by_task_execution_by_task) | **Get** /tasks/{task_id}/task_executions/{task_execution_id}/files/{file_id}/tapes | Displays tapes containing specific file, related to the specific task.
[**on_tapes_files_children_by_pool**](FilesApi.md#on_tapes_files_children_by_pool) | **Get** /pools/{pool_id}/on_tapes_files/{file_parent_id}/children | Lists files under a specific folder on tape of pools, specific for Active and Offline.
[**show_file**](FilesApi.md#show_file) | **Get** /files/{file_id} | Displays a specific file.
[**show_file_by_container**](FilesApi.md#show_file_by_container) | **Get** /containers/{container_id}/files/{file_id} | Displays a specific file.
[**show_file_by_pool**](FilesApi.md#show_file_by_pool) | **Get** /pools/{pool_id}/files/{file_id} | Displays a specific file.
[**show_file_by_task**](FilesApi.md#show_file_by_task) | **Get** /tasks/{task_id}/files/{file_id} | Displays a specific file.
[**show_file_by_task_execution**](FilesApi.md#show_file_by_task_execution) | **Get** /task_executions/{task_execution_id}/files/{file_id} | Displays a specific file.
[**show_file_by_task_execution_by_task**](FilesApi.md#show_file_by_task_execution_by_task) | **Get** /tasks/{task_id}/task_executions/{task_execution_id}/files/{file_id} | Displays a specific file.
[**show_import_file_by_pool**](FilesApi.md#show_import_file_by_pool) | **Get** /pools/{pool_id}/import_files/{file_id} | Displays a specific file on tape of pools, specific for Data Exchange.
[**show_on_tape_file_by_pool**](FilesApi.md#show_on_tape_file_by_pool) | **Get** /pools/{pool_id}/on_tapes_files/{file_id} | Displays a specific file on tape of pools, specific for Active and Offline.



## files_children

> crate::models::NodeumFileCollection files_children(file_parent_id, limit, offset, file_id, name, _type, permission, size, change_date, modification_date, access_date, gid, uid)
Lists files under a specific folder.

**API Key Scope**: files / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_parent_id** | **i32** | Numeric ID of parent folder. | [required] |
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**file_id** | Option<**String**> | Filter on file id |  |
**name** | Option<**String**> | Filter on name |  |
**_type** | Option<**String**> | Filter on type |  |
**permission** | Option<**String**> | Filter on permission |  |
**size** | Option<**String**> | Filter on size |  |
**change_date** | Option<**String**> | Filter on change date |  |
**modification_date** | Option<**String**> | Filter on modification date |  |
**access_date** | Option<**String**> | Filter on access date |  |
**gid** | Option<**String**> | Filter on gid |  |
**uid** | Option<**String**> | Filter on uid |  |

### Return type

[**crate::models::NodeumFileCollection**](nodeum_file_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_children_by_container

> crate::models::NodeumFileCollection files_children_by_container(container_id, file_parent_id, limit, offset, file_id, name, _type, permission, size, change_date, modification_date, access_date, gid, uid)
Lists files under a specific folder.

**API Key Scope**: files / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** | Numeric ID or name of container. | [required] |
**file_parent_id** | **i32** | Numeric ID of parent folder. | [required] |
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**file_id** | Option<**String**> | Filter on file id |  |
**name** | Option<**String**> | Filter on name |  |
**_type** | Option<**String**> | Filter on type |  |
**permission** | Option<**String**> | Filter on permission |  |
**size** | Option<**String**> | Filter on size |  |
**change_date** | Option<**String**> | Filter on change date |  |
**modification_date** | Option<**String**> | Filter on modification date |  |
**access_date** | Option<**String**> | Filter on access date |  |
**gid** | Option<**String**> | Filter on gid |  |
**uid** | Option<**String**> | Filter on uid |  |

### Return type

[**crate::models::NodeumFileCollection**](nodeum_file_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_children_by_pool

> crate::models::NodeumFileCollection files_children_by_pool(pool_id, file_parent_id, limit, offset, file_id, name, _type, permission, size, change_date, modification_date, access_date, gid, uid)
Lists files under a specific folder.

**API Key Scope**: files / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |
**file_parent_id** | **i32** | Numeric ID of parent folder. | [required] |
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**file_id** | Option<**String**> | Filter on file id |  |
**name** | Option<**String**> | Filter on name |  |
**_type** | Option<**String**> | Filter on type |  |
**permission** | Option<**String**> | Filter on permission |  |
**size** | Option<**String**> | Filter on size |  |
**change_date** | Option<**String**> | Filter on change date |  |
**modification_date** | Option<**String**> | Filter on modification date |  |
**access_date** | Option<**String**> | Filter on access date |  |
**gid** | Option<**String**> | Filter on gid |  |
**uid** | Option<**String**> | Filter on uid |  |

### Return type

[**crate::models::NodeumFileCollection**](nodeum_file_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_children_by_task

> crate::models::NodeumFileCollection files_children_by_task(task_id, file_parent_id, limit, offset, file_id, name, _type, permission, size, change_date, modification_date, access_date, gid, uid)
Lists files under a specific folder.

**API Key Scope**: files / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**file_parent_id** | **i32** | Numeric ID of parent folder. | [required] |
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**file_id** | Option<**String**> | Filter on file id |  |
**name** | Option<**String**> | Filter on name |  |
**_type** | Option<**String**> | Filter on type |  |
**permission** | Option<**String**> | Filter on permission |  |
**size** | Option<**String**> | Filter on size |  |
**change_date** | Option<**String**> | Filter on change date |  |
**modification_date** | Option<**String**> | Filter on modification date |  |
**access_date** | Option<**String**> | Filter on access date |  |
**gid** | Option<**String**> | Filter on gid |  |
**uid** | Option<**String**> | Filter on uid |  |

### Return type

[**crate::models::NodeumFileCollection**](nodeum_file_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_children_by_task_execution

> crate::models::NodeumFileCollection files_children_by_task_execution(task_execution_id, file_parent_id, limit, offset, file_id, name, _type, permission, size, change_date, modification_date, access_date, gid, uid)
Lists files under a specific folder.

**API Key Scope**: files / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_execution_id** | **String** | Numeric ID of task execution. | [required] |
**file_parent_id** | **i32** | Numeric ID of parent folder. | [required] |
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**file_id** | Option<**String**> | Filter on file id |  |
**name** | Option<**String**> | Filter on name |  |
**_type** | Option<**String**> | Filter on type |  |
**permission** | Option<**String**> | Filter on permission |  |
**size** | Option<**String**> | Filter on size |  |
**change_date** | Option<**String**> | Filter on change date |  |
**modification_date** | Option<**String**> | Filter on modification date |  |
**access_date** | Option<**String**> | Filter on access date |  |
**gid** | Option<**String**> | Filter on gid |  |
**uid** | Option<**String**> | Filter on uid |  |

### Return type

[**crate::models::NodeumFileCollection**](nodeum_file_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_children_by_task_execution_by_task

> crate::models::NodeumFileCollection files_children_by_task_execution_by_task(task_id, task_execution_id, file_parent_id, limit, offset, file_id, name, _type, permission, size, change_date, modification_date, access_date, gid, uid)
Lists files under a specific folder.

**API Key Scope**: files / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**task_execution_id** | **String** | Numeric ID of task execution. | [required] |
**file_parent_id** | **i32** | Numeric ID of parent folder. | [required] |
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**file_id** | Option<**String**> | Filter on file id |  |
**name** | Option<**String**> | Filter on name |  |
**_type** | Option<**String**> | Filter on type |  |
**permission** | Option<**String**> | Filter on permission |  |
**size** | Option<**String**> | Filter on size |  |
**change_date** | Option<**String**> | Filter on change date |  |
**modification_date** | Option<**String**> | Filter on modification date |  |
**access_date** | Option<**String**> | Filter on access date |  |
**gid** | Option<**String**> | Filter on gid |  |
**uid** | Option<**String**> | Filter on uid |  |

### Return type

[**crate::models::NodeumFileCollection**](nodeum_file_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_files_children_by_pool

> crate::models::ImportFileCollection import_files_children_by_pool(pool_id, file_parent_id, limit, offset, file_id, name, _type, permission, size, change_date, modification_date, access_date, gid, uid)
Lists files under a specific folder on tape of pools, specific for Data Exchange.

**API Key Scope**: import_files / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |
**file_parent_id** | **i32** | Numeric ID of parent folder. | [required] |
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**file_id** | Option<**String**> | Filter on file id |  |
**name** | Option<**String**> | Filter on name |  |
**_type** | Option<**String**> | Filter on type |  |
**permission** | Option<**String**> | Filter on permission |  |
**size** | Option<**String**> | Filter on size |  |
**change_date** | Option<**String**> | Filter on change date |  |
**modification_date** | Option<**String**> | Filter on modification date |  |
**access_date** | Option<**String**> | Filter on access date |  |
**gid** | Option<**String**> | Filter on gid |  |
**uid** | Option<**String**> | Filter on uid |  |

### Return type

[**crate::models::ImportFileCollection**](import_file_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_files

> crate::models::NodeumFileCollection index_files(limit, offset, file_id, name, _type, permission, size, change_date, modification_date, access_date, gid, uid)
Lists files on root.

**API Key Scope**: files / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**file_id** | Option<**String**> | Filter on file id |  |
**name** | Option<**String**> | Filter on name |  |
**_type** | Option<**String**> | Filter on type |  |
**permission** | Option<**String**> | Filter on permission |  |
**size** | Option<**String**> | Filter on size |  |
**change_date** | Option<**String**> | Filter on change date |  |
**modification_date** | Option<**String**> | Filter on modification date |  |
**access_date** | Option<**String**> | Filter on access date |  |
**gid** | Option<**String**> | Filter on gid |  |
**uid** | Option<**String**> | Filter on uid |  |

### Return type

[**crate::models::NodeumFileCollection**](nodeum_file_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_files_by_container

> crate::models::NodeumFileCollection index_files_by_container(container_id, limit, offset, file_id, name, _type, permission, size, change_date, modification_date, access_date, gid, uid)
Lists files on root.

**API Key Scope**: files / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** | Numeric ID or name of container. | [required] |
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**file_id** | Option<**String**> | Filter on file id |  |
**name** | Option<**String**> | Filter on name |  |
**_type** | Option<**String**> | Filter on type |  |
**permission** | Option<**String**> | Filter on permission |  |
**size** | Option<**String**> | Filter on size |  |
**change_date** | Option<**String**> | Filter on change date |  |
**modification_date** | Option<**String**> | Filter on modification date |  |
**access_date** | Option<**String**> | Filter on access date |  |
**gid** | Option<**String**> | Filter on gid |  |
**uid** | Option<**String**> | Filter on uid |  |

### Return type

[**crate::models::NodeumFileCollection**](nodeum_file_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_files_by_pool

> crate::models::NodeumFileCollection index_files_by_pool(pool_id, limit, offset, file_id, name, _type, permission, size, change_date, modification_date, access_date, gid, uid)
Lists files on root.

**API Key Scope**: files / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**file_id** | Option<**String**> | Filter on file id |  |
**name** | Option<**String**> | Filter on name |  |
**_type** | Option<**String**> | Filter on type |  |
**permission** | Option<**String**> | Filter on permission |  |
**size** | Option<**String**> | Filter on size |  |
**change_date** | Option<**String**> | Filter on change date |  |
**modification_date** | Option<**String**> | Filter on modification date |  |
**access_date** | Option<**String**> | Filter on access date |  |
**gid** | Option<**String**> | Filter on gid |  |
**uid** | Option<**String**> | Filter on uid |  |

### Return type

[**crate::models::NodeumFileCollection**](nodeum_file_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_files_by_task

> crate::models::NodeumFileCollection index_files_by_task(task_id, limit, offset, file_id, name, _type, permission, size, change_date, modification_date, access_date, gid, uid)
Lists files on root.

**API Key Scope**: files / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**file_id** | Option<**String**> | Filter on file id |  |
**name** | Option<**String**> | Filter on name |  |
**_type** | Option<**String**> | Filter on type |  |
**permission** | Option<**String**> | Filter on permission |  |
**size** | Option<**String**> | Filter on size |  |
**change_date** | Option<**String**> | Filter on change date |  |
**modification_date** | Option<**String**> | Filter on modification date |  |
**access_date** | Option<**String**> | Filter on access date |  |
**gid** | Option<**String**> | Filter on gid |  |
**uid** | Option<**String**> | Filter on uid |  |

### Return type

[**crate::models::NodeumFileCollection**](nodeum_file_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_files_by_task_execution

> crate::models::NodeumFileCollection index_files_by_task_execution(task_execution_id, limit, offset, file_id, name, _type, permission, size, change_date, modification_date, access_date, gid, uid)
Lists files on root.

**API Key Scope**: files / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_execution_id** | **String** | Numeric ID of task execution. | [required] |
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**file_id** | Option<**String**> | Filter on file id |  |
**name** | Option<**String**> | Filter on name |  |
**_type** | Option<**String**> | Filter on type |  |
**permission** | Option<**String**> | Filter on permission |  |
**size** | Option<**String**> | Filter on size |  |
**change_date** | Option<**String**> | Filter on change date |  |
**modification_date** | Option<**String**> | Filter on modification date |  |
**access_date** | Option<**String**> | Filter on access date |  |
**gid** | Option<**String**> | Filter on gid |  |
**uid** | Option<**String**> | Filter on uid |  |

### Return type

[**crate::models::NodeumFileCollection**](nodeum_file_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_files_by_task_execution_by_task

> crate::models::NodeumFileCollection index_files_by_task_execution_by_task(task_id, task_execution_id, limit, offset, file_id, name, _type, permission, size, change_date, modification_date, access_date, gid, uid)
Lists files on root.

**API Key Scope**: files / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**task_execution_id** | **String** | Numeric ID of task execution. | [required] |
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**file_id** | Option<**String**> | Filter on file id |  |
**name** | Option<**String**> | Filter on name |  |
**_type** | Option<**String**> | Filter on type |  |
**permission** | Option<**String**> | Filter on permission |  |
**size** | Option<**String**> | Filter on size |  |
**change_date** | Option<**String**> | Filter on change date |  |
**modification_date** | Option<**String**> | Filter on modification date |  |
**access_date** | Option<**String**> | Filter on access date |  |
**gid** | Option<**String**> | Filter on gid |  |
**uid** | Option<**String**> | Filter on uid |  |

### Return type

[**crate::models::NodeumFileCollection**](nodeum_file_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_import_files_by_pool

> crate::models::ImportFileCollection index_import_files_by_pool(pool_id, limit, offset, file_id, name, _type, permission, size, change_date, modification_date, access_date, gid, uid)
Lists files on root of tape of pools, specific for Data Exchange.

**API Key Scope**: import_files / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**file_id** | Option<**String**> | Filter on file id |  |
**name** | Option<**String**> | Filter on name |  |
**_type** | Option<**String**> | Filter on type |  |
**permission** | Option<**String**> | Filter on permission |  |
**size** | Option<**String**> | Filter on size |  |
**change_date** | Option<**String**> | Filter on change date |  |
**modification_date** | Option<**String**> | Filter on modification date |  |
**access_date** | Option<**String**> | Filter on access date |  |
**gid** | Option<**String**> | Filter on gid |  |
**uid** | Option<**String**> | Filter on uid |  |

### Return type

[**crate::models::ImportFileCollection**](import_file_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_on_tapes_files_by_pool

> crate::models::OnTapesFileCollection index_on_tapes_files_by_pool(pool_id, limit, offset, name, _type, size)
Lists files on root of tape of pools, specific for Active and Offline.

**API Key Scope**: on_tapes_files / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**name** | Option<**String**> | Filter on name |  |
**_type** | Option<**String**> | Filter on type |  |
**size** | Option<**String**> | Filter on size |  |

### Return type

[**crate::models::OnTapesFileCollection**](on_tapes_file_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_tapes_by_file_by_pool

> crate::models::TapeCollection index_tapes_by_file_by_pool(pool_id, file_id)
Displays tapes containing specific file, related to the specific pool.

**API Key Scope**: files / tapes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |
**file_id** | **i32** | Numeric ID of file. | [required] |

### Return type

[**crate::models::TapeCollection**](tape_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_tapes_by_file_by_task

> crate::models::TapeCollection index_tapes_by_file_by_task(task_id, file_id)
Displays tapes containing specific file, related to the specific task.

**API Key Scope**: files / tapes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**file_id** | **i32** | Numeric ID of file. | [required] |

### Return type

[**crate::models::TapeCollection**](tape_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_tapes_by_file_by_task_execution

> crate::models::TapeCollection index_tapes_by_file_by_task_execution(task_execution_id, file_id)
Displays tapes containing specific file, related to the specific task.

**API Key Scope**: files / tapes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_execution_id** | **String** | Numeric ID of task execution. | [required] |
**file_id** | **i32** | Numeric ID of file. | [required] |

### Return type

[**crate::models::TapeCollection**](tape_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_tapes_by_file_by_task_execution_by_task

> crate::models::TapeCollection index_tapes_by_file_by_task_execution_by_task(task_id, task_execution_id, file_id)
Displays tapes containing specific file, related to the specific task.

**API Key Scope**: files / tapes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**task_execution_id** | **String** | Numeric ID of task execution. | [required] |
**file_id** | **i32** | Numeric ID of file. | [required] |

### Return type

[**crate::models::TapeCollection**](tape_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## on_tapes_files_children_by_pool

> crate::models::OnTapesFileCollection on_tapes_files_children_by_pool(pool_id, file_parent_id, limit, offset, name, _type, size)
Lists files under a specific folder on tape of pools, specific for Active and Offline.

**API Key Scope**: on_tapes_files / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |
**file_parent_id** | **i32** | Numeric ID of parent folder. | [required] |
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**name** | Option<**String**> | Filter on name |  |
**_type** | Option<**String**> | Filter on type |  |
**size** | Option<**String**> | Filter on size |  |

### Return type

[**crate::models::OnTapesFileCollection**](on_tapes_file_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_file

> crate::models::NodeumFileWithPath show_file(file_id)
Displays a specific file.

**API Key Scope**: files / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_id** | **i32** | Numeric ID of file. | [required] |

### Return type

[**crate::models::NodeumFileWithPath**](nodeum_file_with_path.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_file_by_container

> crate::models::NodeumFileWithPath show_file_by_container(container_id, file_id)
Displays a specific file.

**API Key Scope**: files / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** | Numeric ID or name of container. | [required] |
**file_id** | **i32** | Numeric ID of file. | [required] |

### Return type

[**crate::models::NodeumFileWithPath**](nodeum_file_with_path.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_file_by_pool

> crate::models::NodeumFileWithPath show_file_by_pool(pool_id, file_id)
Displays a specific file.

**API Key Scope**: files / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |
**file_id** | **i32** | Numeric ID of file. | [required] |

### Return type

[**crate::models::NodeumFileWithPath**](nodeum_file_with_path.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_file_by_task

> crate::models::NodeumFileWithPath show_file_by_task(task_id, file_id)
Displays a specific file.

**API Key Scope**: files / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**file_id** | **i32** | Numeric ID of file. | [required] |

### Return type

[**crate::models::NodeumFileWithPath**](nodeum_file_with_path.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_file_by_task_execution

> crate::models::NodeumFileWithPath show_file_by_task_execution(task_execution_id, file_id)
Displays a specific file.

**API Key Scope**: files / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_execution_id** | **String** | Numeric ID of task execution. | [required] |
**file_id** | **i32** | Numeric ID of file. | [required] |

### Return type

[**crate::models::NodeumFileWithPath**](nodeum_file_with_path.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_file_by_task_execution_by_task

> crate::models::NodeumFileWithPath show_file_by_task_execution_by_task(task_id, task_execution_id, file_id)
Displays a specific file.

**API Key Scope**: files / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Numeric ID or name of task. Task names are not unique, it's recommanded to use numeric ID. | [required] |
**task_execution_id** | **String** | Numeric ID of task execution. | [required] |
**file_id** | **i32** | Numeric ID of file. | [required] |

### Return type

[**crate::models::NodeumFileWithPath**](nodeum_file_with_path.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_import_file_by_pool

> crate::models::ImportFileWithPath show_import_file_by_pool(pool_id, file_id)
Displays a specific file on tape of pools, specific for Data Exchange.

**API Key Scope**: import_files / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |
**file_id** | **i32** | Numeric ID of file. | [required] |

### Return type

[**crate::models::ImportFileWithPath**](import_file_with_path.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_on_tape_file_by_pool

> crate::models::OnTapesFile show_on_tape_file_by_pool(pool_id, file_id)
Displays a specific file on tape of pools, specific for Active and Offline.

**API Key Scope**: on_tapes_files / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |
**file_id** | **i32** | Numeric ID of file. | [required] |

### Return type

[**crate::models::OnTapesFile**](on_tapes_file.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

