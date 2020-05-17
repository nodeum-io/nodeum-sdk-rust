# \ContainersApi

All URIs are relative to *http://localhost/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_container**](ContainersApi.md#create_container) | **Post** /containers | Creates a new container.
[**create_container_privilege**](ContainersApi.md#create_container_privilege) | **Post** /containers/{container_id}/container_privileges | Creates a new privilege on the container.
[**destroy_container**](ContainersApi.md#destroy_container) | **Delete** /containers/{container_id} | Destroys a specific container.
[**destroy_container_privilege**](ContainersApi.md#destroy_container_privilege) | **Delete** /containers/{container_id}/container_privileges/{container_privilege_id} | Destroys a specific privilege.
[**index_container_privileges**](ContainersApi.md#index_container_privileges) | **Get** /containers/{container_id}/container_privileges | Lists all privilege on the container.
[**index_containers**](ContainersApi.md#index_containers) | **Get** /containers | Lists all containers.
[**show_container**](ContainersApi.md#show_container) | **Get** /containers/{container_id} | Displays a specific container.
[**show_container_privilege**](ContainersApi.md#show_container_privilege) | **Get** /containers/{container_id}/container_privileges/{container_privilege_id} | Displays a specific privilege.
[**update_container**](ContainersApi.md#update_container) | **Put** /containers/{container_id} | Updates a specific container.
[**update_container_privilege**](ContainersApi.md#update_container_privilege) | **Put** /containers/{container_id}/container_privileges/{container_privilege_id} | Updates a specific privilege.



## create_container

> crate::models::Container create_container(container_body)
Creates a new container.

It **does not** yet create the file structure and configure the samba connection. Use API v1 instead.  **API Key Scope**: containers / create

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_body** | [**Container**](Container.md) |  | [required] |

### Return type

[**crate::models::Container**](container.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_container_privilege

> crate::models::ContainerPrivilege create_container_privilege(container_id, container_privilege_body)
Creates a new privilege on the container.

**API Key Scope**: container_privileges / create

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** | Numeric ID or name of container. | [required] |
**container_privilege_body** | [**ContainerPrivilege**](ContainerPrivilege.md) |  | [required] |

### Return type

[**crate::models::ContainerPrivilege**](container_privilege.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_container

> destroy_container(container_id)
Destroys a specific container.

**API Key Scope**: containers / destroy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** | Numeric ID or name of container. | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_container_privilege

> destroy_container_privilege(container_id, container_privilege_id)
Destroys a specific privilege.

**API Key Scope**: container_privileges / destroy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** | Numeric ID or name of container. | [required] |
**container_privilege_id** | **i32** | Numeric ID of container privilege. | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_container_privileges

> crate::models::ContainerPrivilegeCollection index_container_privileges(container_id, limit, offset, sort_by, id, name, privilege, _type)
Lists all privilege on the container.

**API Key Scope**: container_privileges / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** | Numeric ID or name of container. | [required] |
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Sort results by attribute.  Can sort on multiple attributes, separated by `|`. Order direction can be suffixing the attribute by either `:asc` (default) or `:desc`. |  |
**id** | Option<**String**> | Filter on id |  |
**name** | Option<**String**> | Filter on name |  |
**privilege** | Option<**String**> | Filter on privilege |  |
**_type** | Option<**String**> | Filter on type |  |

### Return type

[**crate::models::ContainerPrivilegeCollection**](container_privilege_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_containers

> crate::models::ContainerCollection index_containers(limit, offset, sort_by, id, name, comment, quota_total_size, quota_on_cache, stat_total_files, stat_total_size, stat_size_on_cache, guest_right, last_update)
Lists all containers.

**API Key Scope**: containers / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Sort results by attribute.  Can sort on multiple attributes, separated by `|`. Order direction can be suffixing the attribute by either `:asc` (default) or `:desc`. |  |
**id** | Option<**String**> | Filter on id |  |
**name** | Option<**String**> | Filter on name |  |
**comment** | Option<**String**> | Filter on comment |  |
**quota_total_size** | Option<**String**> | Filter on quota total size |  |
**quota_on_cache** | Option<**String**> | Filter on quota on cache |  |
**stat_total_files** | Option<**String**> | Filter on stat total files |  |
**stat_total_size** | Option<**String**> | Filter on stat total size |  |
**stat_size_on_cache** | Option<**String**> | Filter on stat size on cache |  |
**guest_right** | Option<**String**> | Filter on guest right |  |
**last_update** | Option<**String**> | Filter on last update |  |

### Return type

[**crate::models::ContainerCollection**](container_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_container

> crate::models::Container show_container(container_id)
Displays a specific container.

**API Key Scope**: containers / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** | Numeric ID or name of container. | [required] |

### Return type

[**crate::models::Container**](container.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_container_privilege

> crate::models::ContainerPrivilege show_container_privilege(container_id, container_privilege_id)
Displays a specific privilege.

**API Key Scope**: container_privileges / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** | Numeric ID or name of container. | [required] |
**container_privilege_id** | **i32** | Numeric ID of container privilege. | [required] |

### Return type

[**crate::models::ContainerPrivilege**](container_privilege.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_container

> crate::models::Container update_container(container_id, container_body)
Updates a specific container.

It **does not** yet create the file structure and configure the samba connection. Use API v1 instead.  **API Key Scope**: containers / update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** | Numeric ID or name of container. | [required] |
**container_body** | [**Container**](Container.md) |  | [required] |

### Return type

[**crate::models::Container**](container.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_container_privilege

> crate::models::ContainerPrivilege update_container_privilege(container_id, container_privilege_id, container_privilege_body)
Updates a specific privilege.

**API Key Scope**: container_privileges / update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **String** | Numeric ID or name of container. | [required] |
**container_privilege_id** | **i32** | Numeric ID of container privilege. | [required] |
**container_privilege_body** | [**ContainerPrivilege**](ContainerPrivilege.md) |  | [required] |

### Return type

[**crate::models::ContainerPrivilege**](container_privilege.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

