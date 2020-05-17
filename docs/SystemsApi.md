# \SystemsApi

All URIs are relative to *http://localhost/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**download_reset_vars**](SystemsApi.md#download_reset_vars) | **Post** /systems/reset/generate_vars | Creates a YAML file with selected tables and downloads it
[**result_download_traces**](SystemsApi.md#result_download_traces) | **Get** /systems/download_traces | Check result of a download traces job.
[**trigger_download_traces**](SystemsApi.md#trigger_download_traces) | **Put** /systems/download_traces | Trigger a download traces request.



## download_reset_vars

> std::path::PathBuf download_reset_vars(reset_form)
Creates a YAML file with selected tables and downloads it

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reset_form** | [**Reset**](Reset.md) |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/x-yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## result_download_traces

> std::path::PathBuf result_download_traces(job_id)
Check result of a download traces job.

**API Key Scope**: systems / download_traces

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | ID of active job | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/tar+gzip, queued, working, failed, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_download_traces

> crate::models::ActiveJobStatus trigger_download_traces(_type)
Trigger a download traces request.

**API Key Scope**: systems / download_traces

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | **String** | Type of traces to download | [required] |

### Return type

[**crate::models::ActiveJobStatus**](active_job_status.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, queued, working, failed

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

