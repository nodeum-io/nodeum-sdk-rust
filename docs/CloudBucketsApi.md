# \CloudBucketsApi

All URIs are relative to *http://localhost/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**index_cloud_buckets**](CloudBucketsApi.md#index_cloud_buckets) | **Get** /cloud_buckets | Lists all cloud buckets.
[**index_cloud_buckets_by_cloud_connector**](CloudBucketsApi.md#index_cloud_buckets_by_cloud_connector) | **Get** /cloud_connectors/{cloud_connector_id}/cloud_buckets | Lists all cloud buckets.
[**index_cloud_buckets_by_pool**](CloudBucketsApi.md#index_cloud_buckets_by_pool) | **Get** /pools/{pool_id}/cloud_buckets | Lists all cloud buckets from pool.
[**mount_status_cloud_bucket**](CloudBucketsApi.md#mount_status_cloud_bucket) | **Get** /cloud_buckets/{cloud_bucket_id}/mount | Get mount status of Cloud bucket.
[**mount_status_cloud_bucket_by_cloud_connector**](CloudBucketsApi.md#mount_status_cloud_bucket_by_cloud_connector) | **Get** /cloud_connectors/{cloud_connector_id}/cloud_buckets/{cloud_bucket_id}/mount | Get mount status of Cloud bucket.
[**mount_status_cloud_bucket_by_pool**](CloudBucketsApi.md#mount_status_cloud_bucket_by_pool) | **Get** /pools/{pool_id}/cloud_buckets/{cloud_bucket_id}/mount | Get mount status of Cloud bucket.
[**show_cloud_bucket**](CloudBucketsApi.md#show_cloud_bucket) | **Get** /cloud_buckets/{cloud_bucket_id} | Displays a specific cloud bucket.
[**show_cloud_bucket_by_cloud_connector**](CloudBucketsApi.md#show_cloud_bucket_by_cloud_connector) | **Get** /cloud_connectors/{cloud_connector_id}/cloud_buckets/{cloud_bucket_id} | Displays a specific cloud bucket.
[**show_cloud_bucket_by_pool**](CloudBucketsApi.md#show_cloud_bucket_by_pool) | **Get** /pools/{pool_id}/cloud_buckets/{cloud_bucket_id} | Displays a specific cloud bucket.
[**sync_cloud_buckets**](CloudBucketsApi.md#sync_cloud_buckets) | **Put** /cloud_connectors/{cloud_connector_id}/cloud_buckets/-/sync | Synchronize internal cloud buckets with their remote equivalent.
[**sync_result_cloud_buckets**](CloudBucketsApi.md#sync_result_cloud_buckets) | **Get** /cloud_connectors/{cloud_connector_id}/cloud_buckets/-/sync | Check result of cloud connector sync job.
[**update_cloud_bucket**](CloudBucketsApi.md#update_cloud_bucket) | **Put** /cloud_buckets/{cloud_bucket_id} | Updates a specific cloud bucket.
[**update_cloud_bucket_by_cloud_connector**](CloudBucketsApi.md#update_cloud_bucket_by_cloud_connector) | **Put** /cloud_connectors/{cloud_connector_id}/cloud_buckets/{cloud_bucket_id} | Updates a specific cloud bucket.
[**update_cloud_bucket_by_pool**](CloudBucketsApi.md#update_cloud_bucket_by_pool) | **Put** /pools/{pool_id}/cloud_buckets/{cloud_bucket_id} | Updates a specific cloud bucket.



## index_cloud_buckets

> crate::models::CloudBucketCollection index_cloud_buckets(limit, offset, sort_by, id, cloud_connector_id, pool_id, name, location, price)
Lists all cloud buckets.

**API Key Scope**: cloud_buckets / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Sort results by attribute.  Can sort on multiple attributes, separated by `|`. Order direction can be suffixing the attribute by either `:asc` (default) or `:desc`. |  |
**id** | Option<**String**> | Filter on id |  |
**cloud_connector_id** | Option<**String**> | Filter on cloud connector id |  |
**pool_id** | Option<**String**> | Filter on a pool id |  |
**name** | Option<**String**> | Filter on name |  |
**location** | Option<**String**> | Filter on location |  |
**price** | Option<**String**> | Filter on price |  |

### Return type

[**crate::models::CloudBucketCollection**](cloud_bucket_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_cloud_buckets_by_cloud_connector

> crate::models::CloudBucketCollection index_cloud_buckets_by_cloud_connector(cloud_connector_id, limit, offset, sort_by, id, pool_id, name, location, price)
Lists all cloud buckets.

**API Key Scope**: cloud_buckets / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cloud_connector_id** | **String** | Numeric ID or name of cloud connector. | [required] |
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Sort results by attribute.  Can sort on multiple attributes, separated by `|`. Order direction can be suffixing the attribute by either `:asc` (default) or `:desc`. |  |
**id** | Option<**String**> | Filter on id |  |
**pool_id** | Option<**String**> | Filter on a pool id |  |
**name** | Option<**String**> | Filter on name |  |
**location** | Option<**String**> | Filter on location |  |
**price** | Option<**String**> | Filter on price |  |

### Return type

[**crate::models::CloudBucketCollection**](cloud_bucket_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_cloud_buckets_by_pool

> crate::models::CloudBucketCollection index_cloud_buckets_by_pool(pool_id, limit, offset, sort_by, id, cloud_connector_id, name, location, price)
Lists all cloud buckets from pool.

**API Key Scope**: cloud_buckets / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Sort results by attribute.  Can sort on multiple attributes, separated by `|`. Order direction can be suffixing the attribute by either `:asc` (default) or `:desc`. |  |
**id** | Option<**String**> | Filter on id |  |
**cloud_connector_id** | Option<**String**> | Filter on cloud connector id |  |
**name** | Option<**String**> | Filter on name |  |
**location** | Option<**String**> | Filter on location |  |
**price** | Option<**String**> | Filter on price |  |

### Return type

[**crate::models::CloudBucketCollection**](cloud_bucket_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mount_status_cloud_bucket

> crate::models::MountStatus mount_status_cloud_bucket(cloud_bucket_id)
Get mount status of Cloud bucket.

**API Key Scope**: cloud_buckets / mount_status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cloud_bucket_id** | **String** | Numeric ID or name of cloud bucket. | [required] |

### Return type

[**crate::models::MountStatus**](mount_status.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mount_status_cloud_bucket_by_cloud_connector

> crate::models::MountStatus mount_status_cloud_bucket_by_cloud_connector(cloud_connector_id, cloud_bucket_id)
Get mount status of Cloud bucket.

**API Key Scope**: cloud_buckets / mount_status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cloud_connector_id** | **String** | Numeric ID or name of cloud connector. | [required] |
**cloud_bucket_id** | **String** | Numeric ID or name of cloud bucket. | [required] |

### Return type

[**crate::models::MountStatus**](mount_status.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mount_status_cloud_bucket_by_pool

> crate::models::MountStatus mount_status_cloud_bucket_by_pool(pool_id, cloud_bucket_id)
Get mount status of Cloud bucket.

**API Key Scope**: cloud_buckets / mount_status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |
**cloud_bucket_id** | **String** | Numeric ID or name of cloud bucket. | [required] |

### Return type

[**crate::models::MountStatus**](mount_status.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_cloud_bucket

> crate::models::CloudBucket show_cloud_bucket(cloud_bucket_id)
Displays a specific cloud bucket.

**API Key Scope**: cloud_buckets / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cloud_bucket_id** | **String** | Numeric ID or name of cloud bucket. | [required] |

### Return type

[**crate::models::CloudBucket**](cloud_bucket.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_cloud_bucket_by_cloud_connector

> crate::models::CloudBucket show_cloud_bucket_by_cloud_connector(cloud_connector_id, cloud_bucket_id)
Displays a specific cloud bucket.

**API Key Scope**: cloud_buckets / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cloud_connector_id** | **String** | Numeric ID or name of cloud connector. | [required] |
**cloud_bucket_id** | **String** | Numeric ID or name of cloud bucket. | [required] |

### Return type

[**crate::models::CloudBucket**](cloud_bucket.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_cloud_bucket_by_pool

> crate::models::CloudBucket show_cloud_bucket_by_pool(pool_id, cloud_bucket_id)
Displays a specific cloud bucket.

**API Key Scope**: cloud_buckets / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |
**cloud_bucket_id** | **String** | Numeric ID or name of cloud bucket. | [required] |

### Return type

[**crate::models::CloudBucket**](cloud_bucket.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_cloud_buckets

> crate::models::ActiveJobStatus sync_cloud_buckets(cloud_connector_id)
Synchronize internal cloud buckets with their remote equivalent.

**API Key Scope**: cloud_buckets / sync

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cloud_connector_id** | **String** | Numeric ID or name of cloud connector. | [required] |

### Return type

[**crate::models::ActiveJobStatus**](active_job_status.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, queued, working, failed

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_result_cloud_buckets

> crate::models::CloudBucketSimpleCollection sync_result_cloud_buckets(cloud_connector_id, job_id)
Check result of cloud connector sync job.

**API Key Scope**: cloud_buckets / sync

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cloud_connector_id** | **String** | Numeric ID or name of cloud connector. | [required] |
**job_id** | **String** | ID of active job | [required] |

### Return type

[**crate::models::CloudBucketSimpleCollection**](cloud_bucket_simple_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, queued, working, failed, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_cloud_bucket

> crate::models::CloudBucket update_cloud_bucket(cloud_bucket_id, cloud_bucket_body)
Updates a specific cloud bucket.

**API Key Scope**: cloud_buckets / update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cloud_bucket_id** | **String** | Numeric ID or name of cloud bucket. | [required] |
**cloud_bucket_body** | [**CloudBucket**](CloudBucket.md) |  | [required] |

### Return type

[**crate::models::CloudBucket**](cloud_bucket.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_cloud_bucket_by_cloud_connector

> crate::models::CloudBucket update_cloud_bucket_by_cloud_connector(cloud_connector_id, cloud_bucket_id, cloud_bucket_body)
Updates a specific cloud bucket.

**API Key Scope**: cloud_buckets / update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cloud_connector_id** | **String** | Numeric ID or name of cloud connector. | [required] |
**cloud_bucket_id** | **String** | Numeric ID or name of cloud bucket. | [required] |
**cloud_bucket_body** | [**CloudBucket**](CloudBucket.md) |  | [required] |

### Return type

[**crate::models::CloudBucket**](cloud_bucket.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_cloud_bucket_by_pool

> crate::models::CloudBucket update_cloud_bucket_by_pool(pool_id, cloud_bucket_id, cloud_bucket_body)
Updates a specific cloud bucket.

**API Key Scope**: cloud_buckets / update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |
**cloud_bucket_id** | **String** | Numeric ID or name of cloud bucket. | [required] |
**cloud_bucket_body** | [**CloudBucket**](CloudBucket.md) |  | [required] |

### Return type

[**crate::models::CloudBucket**](cloud_bucket.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

