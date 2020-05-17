# \PoolsApi

All URIs are relative to *http://localhost/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_pool**](PoolsApi.md#create_pool) | **Post** /pools | Creates a new pool.
[**create_primary_scan**](PoolsApi.md#create_primary_scan) | **Post** /pools/{pool_id}/primary_scan | Set a new primary pool scan option.
[**destroy_pool**](PoolsApi.md#destroy_pool) | **Delete** /pools/{pool_id} | Destroys a specific tape pool.
[**destroy_primary_scan**](PoolsApi.md#destroy_primary_scan) | **Delete** /pools/{pool_id}/primary_scan | Disable the primary pool scan.
[**index_pools**](PoolsApi.md#index_pools) | **Get** /pools | Lists all pools.
[**mount_pool**](PoolsApi.md#mount_pool) | **Put** /pools/{pool_id}/mount | Mount Pool.
[**mount_status_pool**](PoolsApi.md#mount_status_pool) | **Get** /pools/{pool_id}/mount | Get mount status of Pool.
[**show_pool**](PoolsApi.md#show_pool) | **Get** /pools/{pool_id} | Displays a specific pool.
[**show_primary_scan**](PoolsApi.md#show_primary_scan) | **Get** /pools/{pool_id}/primary_scan | Displays the primary pool scan status.
[**sync_primary_pool**](PoolsApi.md#sync_primary_pool) | **Post** /pools/{pool_id}/sync | Synchronize a primary after a scan (for internal use only).
[**unmount_pool**](PoolsApi.md#unmount_pool) | **Delete** /pools/{pool_id}/mount | Unmount Pool.
[**update_pool**](PoolsApi.md#update_pool) | **Put** /pools/{pool_id} | Updates a specific pool.
[**update_primary_scan**](PoolsApi.md#update_primary_scan) | **Put** /pools/{pool_id}/primary_scan | Updates the existing primary pool scan option.



## create_pool

> crate::models::Pool create_pool(pool_body)
Creates a new pool.

**API Key Scope**: pools / create

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_body** | [**PoolUp**](PoolUp.md) |  | [required] |

### Return type

[**crate::models::Pool**](pool.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_primary_scan

> crate::models::PrimaryScan create_primary_scan(pool_id, primary_scan_body)
Set a new primary pool scan option.

**API Key Scope**: primary_scans / create

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |
**primary_scan_body** | [**PrimaryScan**](PrimaryScan.md) |  | [required] |

### Return type

[**crate::models::PrimaryScan**](primary_scan.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_pool

> destroy_pool(pool_id)
Destroys a specific tape pool.

**API Key Scope**: pools / destroy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_primary_scan

> destroy_primary_scan(pool_id)
Disable the primary pool scan.

**API Key Scope**: primary_scans / destroy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_pools

> crate::models::PoolCollection index_pools(limit, offset, sort_by, id, name, comment, _type, content, primary_id)
Lists all pools.

**API Key Scope**: pools / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Sort results by attribute.  Can sort on multiple attributes, separated by `|`. Order direction can be suffixing the attribute by either `:asc` (default) or `:desc`. |  |
**id** | Option<**String**> | Filter on id |  |
**name** | Option<**String**> | Filter on name |  |
**comment** | Option<**String**> | Filter on comment |  |
**_type** | Option<**String**> | Filter on type |  |
**content** | Option<**String**> | Filter on content |  |
**primary_id** | Option<**String**> | Filter on primary id |  |

### Return type

[**crate::models::PoolCollection**](pool_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mount_pool

> crate::models::MountStatus mount_pool(pool_id)
Mount Pool.

**API Key Scope**: pools / mount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |

### Return type

[**crate::models::MountStatus**](mount_status.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mount_status_pool

> crate::models::MountStatus mount_status_pool(pool_id)
Get mount status of Pool.

**API Key Scope**: pools / mount_status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |

### Return type

[**crate::models::MountStatus**](mount_status.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_pool

> crate::models::Pool show_pool(pool_id)
Displays a specific pool.

**API Key Scope**: pools / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |

### Return type

[**crate::models::Pool**](pool.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_primary_scan

> crate::models::PrimaryScan show_primary_scan(pool_id)
Displays the primary pool scan status.

**API Key Scope**: primary_scans / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |

### Return type

[**crate::models::PrimaryScan**](primary_scan.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_primary_pool

> sync_primary_pool(pool_id, tx)
Synchronize a primary after a scan (for internal use only).

**API Key Scope**: pools / sync_primary

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |
**tx** | **i32** | New transaction number. | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unmount_pool

> crate::models::MountStatus unmount_pool(pool_id)
Unmount Pool.

**API Key Scope**: pools / unmount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |

### Return type

[**crate::models::MountStatus**](mount_status.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_pool

> crate::models::Pool update_pool(pool_id, pool_body)
Updates a specific pool.

**API Key Scope**: pools / update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |
**pool_body** | [**PoolUp**](PoolUp.md) |  | [required] |

### Return type

[**crate::models::Pool**](pool.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_primary_scan

> crate::models::PrimaryScan update_primary_scan(pool_id, primary_scan_body)
Updates the existing primary pool scan option.

**API Key Scope**: primary_scans / update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |
**primary_scan_body** | [**PrimaryScan**](PrimaryScan.md) |  | [required] |

### Return type

[**crate::models::PrimaryScan**](primary_scan.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

