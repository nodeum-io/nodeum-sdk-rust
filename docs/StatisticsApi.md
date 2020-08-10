# \StatisticsApi

All URIs are relative to *http://localhost/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**statistics_by_date**](StatisticsApi.md#statistics_by_date) | **Get** /statistics/by_date | Get statistics about files, grouped by date
[**statistics_by_file_extension**](StatisticsApi.md#statistics_by_file_extension) | **Get** /statistics/by_file_extension | Get statistics about files, grouped by file extension
[**statistics_by_group_owner**](StatisticsApi.md#statistics_by_group_owner) | **Get** /statistics/by_group_owner | Get statistics about files, grouped by owner (group)
[**statistics_by_metadata**](StatisticsApi.md#statistics_by_metadata) | **Get** /statistics/by_metadata | Get statistics about files, grouped by metadata
[**statistics_by_primary_cloud**](StatisticsApi.md#statistics_by_primary_cloud) | **Get** /statistics/by_primary_cloud | Get statistics about files, grouped by primary Cloud
[**statistics_by_primary_name**](StatisticsApi.md#statistics_by_primary_name) | **Get** /statistics/by_primary_name | Get statistics about files, grouped by primary storages
[**statistics_by_primary_nas**](StatisticsApi.md#statistics_by_primary_nas) | **Get** /statistics/by_primary_nas | Get statistics about files, grouped by primary NAS
[**statistics_by_primary_storage**](StatisticsApi.md#statistics_by_primary_storage) | **Get** /statistics/by_primary_storage | Get statistics about files, grouped by primary storage
[**statistics_by_secondary_cloud**](StatisticsApi.md#statistics_by_secondary_cloud) | **Get** /statistics/by_secondary_cloud | Get statistics about files, grouped by secondary Cloud
[**statistics_by_secondary_nas**](StatisticsApi.md#statistics_by_secondary_nas) | **Get** /statistics/by_secondary_nas | Get statistics about files, grouped by secondary NAS
[**statistics_by_secondary_storage**](StatisticsApi.md#statistics_by_secondary_storage) | **Get** /statistics/by_secondary_storage | Get statistics about files, grouped by secondary storage
[**statistics_by_secondary_tape**](StatisticsApi.md#statistics_by_secondary_tape) | **Get** /statistics/by_secondary_tape | Get statistics about files, grouped by secondary Tape
[**statistics_by_size**](StatisticsApi.md#statistics_by_size) | **Get** /statistics/by_size | Get statistics about files, grouped by size
[**statistics_by_user_owner**](StatisticsApi.md#statistics_by_user_owner) | **Get** /statistics/by_user_owner | Get statistics about files, grouped by owner (user)
[**statistics_storage**](StatisticsApi.md#statistics_storage) | **Get** /statistics/storage | Get statistics about storages, grouped by types
[**statistics_task_by_metadata**](StatisticsApi.md#statistics_task_by_metadata) | **Get** /statistics/task_by_metadata | Get statistics about tasks executions, grouped by metadata
[**statistics_task_by_status**](StatisticsApi.md#statistics_task_by_status) | **Get** /statistics/task_by_status | Get statistics about tasks executions, grouped by status
[**statistics_task_by_storage**](StatisticsApi.md#statistics_task_by_storage) | **Get** /statistics/task_by_storage | Get statistics about tasks executions, grouped by source and destination
[**statistics_task_by_workflow**](StatisticsApi.md#statistics_task_by_workflow) | **Get** /statistics/task_by_workflow | Get statistics about tasks executions, grouped by workflow



## statistics_by_date

> crate::models::ByDateFacet statistics_by_date(q, fq, date_attr)
Get statistics about files, grouped by date

**API Key Scope**: statistics / by_date

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Solr query |  |
**fq** | Option<[**Vec<String>**](String.md)> | Solr filter query  Multiple query can be separated by `|`. |  |
**date_attr** | Option<**String**> | Type of date to facet on |  |

### Return type

[**crate::models::ByDateFacet**](by_date_facet.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## statistics_by_file_extension

> crate::models::ByFileExtensionFacet statistics_by_file_extension(q, fq, date_attr, sort, limit)
Get statistics about files, grouped by file extension

**API Key Scope**: statistics / by_file_extension

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Solr query |  |
**fq** | Option<[**Vec<String>**](String.md)> | Solr filter query  Multiple query can be separated by `|`. |  |
**date_attr** | Option<**String**> | Type of date to facet on |  |
**sort** | Option<**String**> | Sort results of facet |  |[default to count]
**limit** | Option<**i32**> | Limit results of facet |  |[default to 10]

### Return type

[**crate::models::ByFileExtensionFacet**](by_file_extension_facet.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## statistics_by_group_owner

> crate::models::ByGroupOwnerFacet statistics_by_group_owner(q, fq, date_attr, sort, limit)
Get statistics about files, grouped by owner (group)

**API Key Scope**: statistics / by_group_owner

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Solr query |  |
**fq** | Option<[**Vec<String>**](String.md)> | Solr filter query  Multiple query can be separated by `|`. |  |
**date_attr** | Option<**String**> | Type of date to facet on |  |
**sort** | Option<**String**> | Sort results of facet |  |[default to count]
**limit** | Option<**i32**> | Limit results of facet |  |[default to 10]

### Return type

[**crate::models::ByGroupOwnerFacet**](by_group_owner_facet.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## statistics_by_metadata

> crate::models::ByMetadataFacet statistics_by_metadata(q, fq, date_attr, sort, limit)
Get statistics about files, grouped by metadata

**API Key Scope**: statistics / by_metadata

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Solr query |  |
**fq** | Option<[**Vec<String>**](String.md)> | Solr filter query  Multiple query can be separated by `|`. |  |
**date_attr** | Option<**String**> | Type of date to facet on |  |
**sort** | Option<**String**> | Sort results of facet |  |[default to count]
**limit** | Option<**i32**> | Limit results of facet |  |[default to 10]

### Return type

[**crate::models::ByMetadataFacet**](by_metadata_facet.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## statistics_by_primary_cloud

> crate::models::ByPrimaryCloudFacet statistics_by_primary_cloud(q, fq, date_attr, sort, limit)
Get statistics about files, grouped by primary Cloud

**API Key Scope**: statistics / by_primary_cloud

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Solr query |  |
**fq** | Option<[**Vec<String>**](String.md)> | Solr filter query  Multiple query can be separated by `|`. |  |
**date_attr** | Option<**String**> | Type of date to facet on |  |
**sort** | Option<**String**> | Sort results of facet |  |[default to count]
**limit** | Option<**i32**> | Limit results of facet |  |[default to 10]

### Return type

[**crate::models::ByPrimaryCloudFacet**](by_primary_cloud_facet.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## statistics_by_primary_name

> crate::models::ByPrimaryFacet statistics_by_primary_name(q, fq, date_attr, sort, limit)
Get statistics about files, grouped by primary storages

**API Key Scope**: statistics / by_primary_name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Solr query |  |
**fq** | Option<[**Vec<String>**](String.md)> | Solr filter query  Multiple query can be separated by `|`. |  |
**date_attr** | Option<**String**> | Type of date to facet on |  |
**sort** | Option<**String**> | Sort results of facet |  |[default to count]
**limit** | Option<**i32**> | Limit results of facet |  |[default to 10]

### Return type

[**crate::models::ByPrimaryFacet**](by_primary_facet.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## statistics_by_primary_nas

> crate::models::ByPrimaryNasFacet statistics_by_primary_nas(q, fq, date_attr, sort, limit)
Get statistics about files, grouped by primary NAS

**API Key Scope**: statistics / by_primary_nas

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Solr query |  |
**fq** | Option<[**Vec<String>**](String.md)> | Solr filter query  Multiple query can be separated by `|`. |  |
**date_attr** | Option<**String**> | Type of date to facet on |  |
**sort** | Option<**String**> | Sort results of facet |  |[default to count]
**limit** | Option<**i32**> | Limit results of facet |  |[default to 10]

### Return type

[**crate::models::ByPrimaryNasFacet**](by_primary_nas_facet.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## statistics_by_primary_storage

> crate::models::ByPrimaryStorageFacet statistics_by_primary_storage(q, fq, date_attr, sort, limit)
Get statistics about files, grouped by primary storage

**API Key Scope**: statistics / by_primary_storage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Solr query |  |
**fq** | Option<[**Vec<String>**](String.md)> | Solr filter query  Multiple query can be separated by `|`. |  |
**date_attr** | Option<**String**> | Type of date to facet on |  |
**sort** | Option<**String**> | Sort results of facet |  |[default to count]
**limit** | Option<**i32**> | Limit results of facet |  |[default to 10]

### Return type

[**crate::models::ByPrimaryStorageFacet**](by_primary_storage_facet.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## statistics_by_secondary_cloud

> crate::models::BySecondaryCloudFacet statistics_by_secondary_cloud(q, fq, date_attr, sort, limit)
Get statistics about files, grouped by secondary Cloud

**API Key Scope**: statistics / by_secondary_cloud

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Solr query |  |
**fq** | Option<[**Vec<String>**](String.md)> | Solr filter query  Multiple query can be separated by `|`. |  |
**date_attr** | Option<**String**> | Type of date to facet on |  |
**sort** | Option<**String**> | Sort results of facet |  |[default to count]
**limit** | Option<**i32**> | Limit results of facet |  |[default to 10]

### Return type

[**crate::models::BySecondaryCloudFacet**](by_secondary_cloud_facet.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## statistics_by_secondary_nas

> crate::models::BySecondaryNasFacet statistics_by_secondary_nas(q, fq, date_attr, sort, limit)
Get statistics about files, grouped by secondary NAS

**API Key Scope**: statistics / by_secondary_nas

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Solr query |  |
**fq** | Option<[**Vec<String>**](String.md)> | Solr filter query  Multiple query can be separated by `|`. |  |
**date_attr** | Option<**String**> | Type of date to facet on |  |
**sort** | Option<**String**> | Sort results of facet |  |[default to count]
**limit** | Option<**i32**> | Limit results of facet |  |[default to 10]

### Return type

[**crate::models::BySecondaryNasFacet**](by_secondary_nas_facet.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## statistics_by_secondary_storage

> crate::models::BySecondaryStorageFacet statistics_by_secondary_storage(q, fq, date_attr, sort, limit)
Get statistics about files, grouped by secondary storage

**API Key Scope**: statistics / by_secondary_storage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Solr query |  |
**fq** | Option<[**Vec<String>**](String.md)> | Solr filter query  Multiple query can be separated by `|`. |  |
**date_attr** | Option<**String**> | Type of date to facet on |  |
**sort** | Option<**String**> | Sort results of facet |  |[default to count]
**limit** | Option<**i32**> | Limit results of facet |  |[default to 10]

### Return type

[**crate::models::BySecondaryStorageFacet**](by_secondary_storage_facet.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## statistics_by_secondary_tape

> crate::models::BySecondaryTapeFacet statistics_by_secondary_tape(q, fq, date_attr, sort, limit)
Get statistics about files, grouped by secondary Tape

**API Key Scope**: statistics / by_secondary_tape

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Solr query |  |
**fq** | Option<[**Vec<String>**](String.md)> | Solr filter query  Multiple query can be separated by `|`. |  |
**date_attr** | Option<**String**> | Type of date to facet on |  |
**sort** | Option<**String**> | Sort results of facet |  |[default to count]
**limit** | Option<**i32**> | Limit results of facet |  |[default to 10]

### Return type

[**crate::models::BySecondaryTapeFacet**](by_secondary_tape_facet.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## statistics_by_size

> crate::models::BySizeFacet statistics_by_size(q, fq, date_attr)
Get statistics about files, grouped by size

**API Key Scope**: statistics / by_size

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Solr query |  |
**fq** | Option<[**Vec<String>**](String.md)> | Solr filter query  Multiple query can be separated by `|`. |  |
**date_attr** | Option<**String**> | Type of date to facet on |  |

### Return type

[**crate::models::BySizeFacet**](by_size_facet.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## statistics_by_user_owner

> crate::models::ByUserOwnerFacet statistics_by_user_owner(q, fq, date_attr, sort, limit)
Get statistics about files, grouped by owner (user)

**API Key Scope**: statistics / by_user_owner

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Solr query |  |
**fq** | Option<[**Vec<String>**](String.md)> | Solr filter query  Multiple query can be separated by `|`. |  |
**date_attr** | Option<**String**> | Type of date to facet on |  |
**sort** | Option<**String**> | Sort results of facet |  |[default to count]
**limit** | Option<**i32**> | Limit results of facet |  |[default to 10]

### Return type

[**crate::models::ByUserOwnerFacet**](by_user_owner_facet.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## statistics_storage

> crate::models::StorageFacet statistics_storage(q, fq)
Get statistics about storages, grouped by types

**API Key Scope**: statistics / storages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Solr query |  |
**fq** | Option<[**Vec<String>**](String.md)> | Solr filter query  Multiple query can be separated by `|`. |  |

### Return type

[**crate::models::StorageFacet**](storage_facet.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## statistics_task_by_metadata

> crate::models::ByTaskMetadataFacet statistics_task_by_metadata(q, fq, sort, limit)
Get statistics about tasks executions, grouped by metadata

**API Key Scope**: statistics / task_by_metadata

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Solr query |  |
**fq** | Option<[**Vec<String>**](String.md)> | Solr filter query  Multiple query can be separated by `|`. |  |
**sort** | Option<**String**> | Sort results of facet on task |  |[default to count]
**limit** | Option<**i32**> | Limit results of facet |  |[default to 10]

### Return type

[**crate::models::ByTaskMetadataFacet**](by_task_metadata_facet.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## statistics_task_by_status

> crate::models::ByTaskStatusFacet statistics_task_by_status(q, fq)
Get statistics about tasks executions, grouped by status

**API Key Scope**: statistics / task_by_status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Solr query |  |
**fq** | Option<[**Vec<String>**](String.md)> | Solr filter query  Multiple query can be separated by `|`. |  |

### Return type

[**crate::models::ByTaskStatusFacet**](by_task_status_facet.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## statistics_task_by_storage

> crate::models::ByTaskStorageFacet statistics_task_by_storage(q, fq)
Get statistics about tasks executions, grouped by source and destination

**API Key Scope**: statistics / task_by_storage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Solr query |  |
**fq** | Option<[**Vec<String>**](String.md)> | Solr filter query  Multiple query can be separated by `|`. |  |

### Return type

[**crate::models::ByTaskStorageFacet**](by_task_storage_facet.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## statistics_task_by_workflow

> crate::models::ByTaskWorkflowFacet statistics_task_by_workflow(q, fq)
Get statistics about tasks executions, grouped by workflow

**API Key Scope**: statistics / task_by_workflow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | Solr query |  |
**fq** | Option<[**Vec<String>**](String.md)> | Solr filter query  Multiple query can be separated by `|`. |  |

### Return type

[**crate::models::ByTaskWorkflowFacet**](by_task_workflow_facet.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

