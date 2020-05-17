# \NasSharesApi

All URIs are relative to *http://localhost/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_nas_share_by_nas**](NasSharesApi.md#create_nas_share_by_nas) | **Post** /nas/{nas_id}/nas_shares | Creates a new NAS share.
[**destroy_nas_share**](NasSharesApi.md#destroy_nas_share) | **Delete** /nas_shares/{nas_share_id} | Destroys a specific NAS share.
[**destroy_nas_share_by_nas**](NasSharesApi.md#destroy_nas_share_by_nas) | **Delete** /nas/{nas_id}/nas_shares/{nas_share_id} | Destroys a specific NAS share.
[**destroy_nas_share_by_pool**](NasSharesApi.md#destroy_nas_share_by_pool) | **Delete** /pools/{pool_id}/nas_shares/{nas_share_id} | Destroys a specific NAS share.
[**index_nas_shares**](NasSharesApi.md#index_nas_shares) | **Get** /nas_shares | Lists all NAS shares.
[**index_nas_shares_by_nas**](NasSharesApi.md#index_nas_shares_by_nas) | **Get** /nas/{nas_id}/nas_shares | Lists all NAS shares.
[**index_nas_shares_by_pool**](NasSharesApi.md#index_nas_shares_by_pool) | **Get** /pools/{pool_id}/nas_shares | Lists all NAS shares from pool.
[**mount_status_nas_share**](NasSharesApi.md#mount_status_nas_share) | **Get** /nas_shares/{nas_share_id}/mount | Get mount status of NAS Share.
[**mount_status_nas_share_by_nas**](NasSharesApi.md#mount_status_nas_share_by_nas) | **Get** /nas/{nas_id}/nas_shares/{nas_share_id}/mount | Get mount status of NAS Share.
[**mount_status_nas_share_by_pool**](NasSharesApi.md#mount_status_nas_share_by_pool) | **Get** /pools/{pool_id}/nas_shares/{nas_share_id}/mount | Get mount status of NAS Share.
[**show_nas_share**](NasSharesApi.md#show_nas_share) | **Get** /nas_shares/{nas_share_id} | Displays a specific NAS share.
[**show_nas_share_by_nas**](NasSharesApi.md#show_nas_share_by_nas) | **Get** /nas/{nas_id}/nas_shares/{nas_share_id} | Displays a specific NAS share.
[**show_nas_share_by_pool**](NasSharesApi.md#show_nas_share_by_pool) | **Get** /pools/{pool_id}/nas_shares/{nas_share_id} | Displays a specific NAS share.
[**test_nas_share**](NasSharesApi.md#test_nas_share) | **Put** /nas/{nas_id}/nas_shares/-/test | Test an unsaved NAS Share.
[**test_result_nas_share**](NasSharesApi.md#test_result_nas_share) | **Get** /nas/{nas_id}/nas_shares/-/test | Check result of a NAS Share test job.
[**update_nas_share**](NasSharesApi.md#update_nas_share) | **Put** /nas_shares/{nas_share_id} | Updates a specific NAS share.
[**update_nas_share_by_nas**](NasSharesApi.md#update_nas_share_by_nas) | **Put** /nas/{nas_id}/nas_shares/{nas_share_id} | Updates a specific NAS share.
[**update_nas_share_by_pool**](NasSharesApi.md#update_nas_share_by_pool) | **Put** /pools/{pool_id}/nas_shares/{nas_share_id} | Updates a specific NAS share.



## create_nas_share_by_nas

> crate::models::NasShare create_nas_share_by_nas(nas_id, nas_share_body)
Creates a new NAS share.

**API Key Scope**: nas_shares / create

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nas_id** | **String** | Numeric ID or name of NAS. | [required] |
**nas_share_body** | [**NasShare**](NasShare.md) |  | [required] |

### Return type

[**crate::models::NasShare**](nas_share.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_nas_share

> destroy_nas_share(nas_share_id)
Destroys a specific NAS share.

**API Key Scope**: nas_shares / destroy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nas_share_id** | **String** | Numeric ID or name of NAS share. | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_nas_share_by_nas

> destroy_nas_share_by_nas(nas_id, nas_share_id)
Destroys a specific NAS share.

**API Key Scope**: nas_shares / destroy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nas_id** | **String** | Numeric ID or name of NAS. | [required] |
**nas_share_id** | **String** | Numeric ID or name of NAS share. | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_nas_share_by_pool

> destroy_nas_share_by_pool(pool_id, nas_share_id)
Destroys a specific NAS share.

**API Key Scope**: nas_shares / destroy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |
**nas_share_id** | **String** | Numeric ID or name of NAS share. | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_nas_shares

> crate::models::NasShareCollection index_nas_shares(limit, offset, sort_by, id, name, path, options, username, nas_id, pool_id)
Lists all NAS shares.

**API Key Scope**: nas_shares / index   Optional API Key Explicit Scope: nas_shares / get_password

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Sort results by attribute.  Can sort on multiple attributes, separated by `|`. Order direction can be suffixing the attribute by either `:asc` (default) or `:desc`. |  |
**id** | Option<**String**> | Filter on id |  |
**name** | Option<**String**> | Filter on name |  |
**path** | Option<**String**> | Filter on path |  |
**options** | Option<**String**> | Filter on options |  |
**username** | Option<**String**> | Filter on username |  |
**nas_id** | Option<**String**> | Filter on NAS id |  |
**pool_id** | Option<**String**> | Filter on a pool id |  |

### Return type

[**crate::models::NasShareCollection**](nas_share_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_nas_shares_by_nas

> crate::models::NasShareCollection index_nas_shares_by_nas(nas_id, limit, offset, sort_by, id, name, path, options, username, pool_id)
Lists all NAS shares.

**API Key Scope**: nas_shares / index   Optional API Key Explicit Scope: nas_shares / get_password

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nas_id** | **String** | Numeric ID or name of NAS. | [required] |
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Sort results by attribute.  Can sort on multiple attributes, separated by `|`. Order direction can be suffixing the attribute by either `:asc` (default) or `:desc`. |  |
**id** | Option<**String**> | Filter on id |  |
**name** | Option<**String**> | Filter on name |  |
**path** | Option<**String**> | Filter on path |  |
**options** | Option<**String**> | Filter on options |  |
**username** | Option<**String**> | Filter on username |  |
**pool_id** | Option<**String**> | Filter on a pool id |  |

### Return type

[**crate::models::NasShareCollection**](nas_share_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_nas_shares_by_pool

> crate::models::NasShareCollection index_nas_shares_by_pool(pool_id, limit, offset, sort_by, id, name, path, options, username, nas_id)
Lists all NAS shares from pool.

**API Key Scope**: nas_shares / index   Optional API Key Explicit Scope: nas_shares / get_password

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Sort results by attribute.  Can sort on multiple attributes, separated by `|`. Order direction can be suffixing the attribute by either `:asc` (default) or `:desc`. |  |
**id** | Option<**String**> | Filter on id |  |
**name** | Option<**String**> | Filter on name |  |
**path** | Option<**String**> | Filter on path |  |
**options** | Option<**String**> | Filter on options |  |
**username** | Option<**String**> | Filter on username |  |
**nas_id** | Option<**String**> | Filter on NAS id |  |

### Return type

[**crate::models::NasShareCollection**](nas_share_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mount_status_nas_share

> crate::models::MountStatus mount_status_nas_share(nas_share_id)
Get mount status of NAS Share.

**API Key Scope**: nas_shares / mount_status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nas_share_id** | **String** | Numeric ID or name of NAS share. | [required] |

### Return type

[**crate::models::MountStatus**](mount_status.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mount_status_nas_share_by_nas

> crate::models::MountStatus mount_status_nas_share_by_nas(nas_id, nas_share_id)
Get mount status of NAS Share.

**API Key Scope**: nas_shares / mount_status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nas_id** | **String** | Numeric ID or name of NAS. | [required] |
**nas_share_id** | **String** | Numeric ID or name of NAS share. | [required] |

### Return type

[**crate::models::MountStatus**](mount_status.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mount_status_nas_share_by_pool

> crate::models::MountStatus mount_status_nas_share_by_pool(pool_id, nas_share_id)
Get mount status of NAS Share.

**API Key Scope**: nas_shares / mount_status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |
**nas_share_id** | **String** | Numeric ID or name of NAS share. | [required] |

### Return type

[**crate::models::MountStatus**](mount_status.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_nas_share

> crate::models::NasShare show_nas_share(nas_share_id)
Displays a specific NAS share.

**API Key Scope**: nas_shares / show   Optional API Key Explicit Scope: nas_shares / get_password

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nas_share_id** | **String** | Numeric ID or name of NAS share. | [required] |

### Return type

[**crate::models::NasShare**](nas_share.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_nas_share_by_nas

> crate::models::NasShare show_nas_share_by_nas(nas_id, nas_share_id)
Displays a specific NAS share.

**API Key Scope**: nas_shares / show   Optional API Key Explicit Scope: nas_shares / get_password

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nas_id** | **String** | Numeric ID or name of NAS. | [required] |
**nas_share_id** | **String** | Numeric ID or name of NAS share. | [required] |

### Return type

[**crate::models::NasShare**](nas_share.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_nas_share_by_pool

> crate::models::NasShare show_nas_share_by_pool(pool_id, nas_share_id)
Displays a specific NAS share.

**API Key Scope**: nas_shares / show   Optional API Key Explicit Scope: nas_shares / get_password

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |
**nas_share_id** | **String** | Numeric ID or name of NAS share. | [required] |

### Return type

[**crate::models::NasShare**](nas_share.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_nas_share

> crate::models::ActiveJobStatus test_nas_share(nas_id, nas_share_body)
Test an unsaved NAS Share.

**API Key Scope**: nas_shares / test

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nas_id** | **String** | Numeric ID or name of NAS. | [required] |
**nas_share_body** | [**NasShare**](NasShare.md) |  | [required] |

### Return type

[**crate::models::ActiveJobStatus**](active_job_status.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, queued, working, failed

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_result_nas_share

> crate::models::ActiveJobStatus test_result_nas_share(nas_id, job_id)
Check result of a NAS Share test job.

**API Key Scope**: nas_shares / test

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nas_id** | **String** | Numeric ID or name of NAS. | [required] |
**job_id** | **String** | ID of active job | [required] |

### Return type

[**crate::models::ActiveJobStatus**](active_job_status.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, queued, working, failed

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_nas_share

> crate::models::NasShare update_nas_share(nas_share_id, nas_share_body)
Updates a specific NAS share.

**API Key Scope**: nas_shares / update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nas_share_id** | **String** | Numeric ID or name of NAS share. | [required] |
**nas_share_body** | [**NasShare**](NasShare.md) |  | [required] |

### Return type

[**crate::models::NasShare**](nas_share.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_nas_share_by_nas

> crate::models::NasShare update_nas_share_by_nas(nas_id, nas_share_id, nas_share_body)
Updates a specific NAS share.

**API Key Scope**: nas_shares / update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nas_id** | **String** | Numeric ID or name of NAS. | [required] |
**nas_share_id** | **String** | Numeric ID or name of NAS share. | [required] |
**nas_share_body** | [**NasShare**](NasShare.md) |  | [required] |

### Return type

[**crate::models::NasShare**](nas_share.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_nas_share_by_pool

> crate::models::NasShare update_nas_share_by_pool(pool_id, nas_share_id, nas_share_body)
Updates a specific NAS share.

**API Key Scope**: nas_shares / update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |
**nas_share_id** | **String** | Numeric ID or name of NAS share. | [required] |
**nas_share_body** | [**NasShare**](NasShare.md) |  | [required] |

### Return type

[**crate::models::NasShare**](nas_share.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

