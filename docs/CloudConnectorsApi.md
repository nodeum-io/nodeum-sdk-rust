# \CloudConnectorsApi

All URIs are relative to *http://localhost/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_cloud_connector**](CloudConnectorsApi.md#create_cloud_connector) | **Post** /cloud_connectors | Creates a new cloud connector.
[**destroy_cloud_connector**](CloudConnectorsApi.md#destroy_cloud_connector) | **Delete** /cloud_connectors/{cloud_connector_id} | Destroys a specific cloud connector.
[**index_cloud_connectors**](CloudConnectorsApi.md#index_cloud_connectors) | **Get** /cloud_connectors | Lists all cloud connectors.
[**show_cloud_connector**](CloudConnectorsApi.md#show_cloud_connector) | **Get** /cloud_connectors/{cloud_connector_id} | Displays a specific cloud connector.
[**test_cloud_connector**](CloudConnectorsApi.md#test_cloud_connector) | **Put** /cloud_connectors/-/test | Test an unsaved cloud connector.
[**test_result_cloud_connector**](CloudConnectorsApi.md#test_result_cloud_connector) | **Get** /cloud_connectors/-/test | Check result of cloud connector test job.
[**update_cloud_connector**](CloudConnectorsApi.md#update_cloud_connector) | **Put** /cloud_connectors/{cloud_connector_id} | Updates a specific cloud connector.



## create_cloud_connector

> crate::models::CloudConnector create_cloud_connector(cloud_connector_body)
Creates a new cloud connector.

**API Key Scope**: cloud_connectors / create

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cloud_connector_body** | [**CloudConnector**](CloudConnector.md) |  | [required] |

### Return type

[**crate::models::CloudConnector**](cloud_connector.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_cloud_connector

> destroy_cloud_connector(cloud_connector_id)
Destroys a specific cloud connector.

**API Key Scope**: cloud_connectors / destroy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cloud_connector_id** | **String** | Numeric ID or name of cloud connector. | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_cloud_connectors

> crate::models::CloudConnectorCollection index_cloud_connectors(limit, offset, sort_by, id, name, url, url_proxy, provider, region, access_key)
Lists all cloud connectors.

**API Key Scope**: cloud_connectors / index   Optional API Key Explicit Scope: cloud_connectors / get_secret_key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Sort results by attribute.  Can sort on multiple attributes, separated by `|`. Order direction can be suffixing the attribute by either `:asc` (default) or `:desc`. |  |
**id** | Option<**String**> | Filter on id |  |
**name** | Option<**String**> | Filter on name |  |
**url** | Option<**String**> | Filter on url |  |
**url_proxy** | Option<**String**> | Filter on url proxy |  |
**provider** | Option<**String**> | Filter on provider |  |
**region** | Option<**String**> | Filter on region |  |
**access_key** | Option<**String**> | Filter on access key |  |

### Return type

[**crate::models::CloudConnectorCollection**](cloud_connector_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_cloud_connector

> crate::models::CloudConnector show_cloud_connector(cloud_connector_id)
Displays a specific cloud connector.

**API Key Scope**: cloud_connectors / show   Optional API Key Explicit Scope: cloud_connectors / get_secret_key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cloud_connector_id** | **String** | Numeric ID or name of cloud connector. | [required] |

### Return type

[**crate::models::CloudConnector**](cloud_connector.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_cloud_connector

> crate::models::ActiveJobStatus test_cloud_connector(cloud_connector_body)
Test an unsaved cloud connector.

**API Key Scope**: cloud_connectors / test

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cloud_connector_body** | [**CloudConnector**](CloudConnector.md) |  | [required] |

### Return type

[**crate::models::ActiveJobStatus**](active_job_status.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, queued, working, failed

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_result_cloud_connector

> crate::models::CloudBucketSimpleCollection test_result_cloud_connector(job_id)
Check result of cloud connector test job.

**API Key Scope**: cloud_connectors / test

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | ID of active job | [required] |

### Return type

[**crate::models::CloudBucketSimpleCollection**](cloud_bucket_simple_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, queued, working, failed, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_cloud_connector

> crate::models::CloudConnector update_cloud_connector(cloud_connector_id, cloud_connector_body)
Updates a specific cloud connector.

**API Key Scope**: cloud_connectors / update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cloud_connector_id** | **String** | Numeric ID or name of cloud connector. | [required] |
**cloud_connector_body** | [**CloudConnector**](CloudConnector.md) |  | [required] |

### Return type

[**crate::models::CloudConnector**](cloud_connector.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

