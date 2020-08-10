# \MetadataApi

All URIs are relative to *http://localhost/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**index_file_metadata_definitions**](MetadataApi.md#index_file_metadata_definitions) | **Get** /file_metadata_definitions | List file metadata definitions
[**index_task_metadata_definitions**](MetadataApi.md#index_task_metadata_definitions) | **Get** /task_metadata_definitions | List task metadata definitions
[**show_file_metadata_definition**](MetadataApi.md#show_file_metadata_definition) | **Get** /file_metadata_definitions/{metadata_definition_id} | Displays a specific task metadata definition.
[**show_task_metadata_definition**](MetadataApi.md#show_task_metadata_definition) | **Get** /task_metadata_definitions/{metadata_definition_id} | Displays a specific task metadata definition.



## index_file_metadata_definitions

> crate::models::FileMetadataDefinitionCollection index_file_metadata_definitions(limit, offset)
List file metadata definitions

**API Key Scope**: file_metadata_definitions / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |

### Return type

[**crate::models::FileMetadataDefinitionCollection**](file_metadata_definition_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_task_metadata_definitions

> crate::models::TaskMetadataDefinitionCollection index_task_metadata_definitions(limit, offset)
List task metadata definitions

**API Key Scope**: task_metadata_definitions / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |

### Return type

[**crate::models::TaskMetadataDefinitionCollection**](task_metadata_definition_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_file_metadata_definition

> crate::models::FileMetadataDefinition show_file_metadata_definition(metadata_definition_id)
Displays a specific task metadata definition.

**API Key Scope**: file_metadata_definitions / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metadata_definition_id** | **String** | Numeric ID or key of a metadata definition | [required] |

### Return type

[**crate::models::FileMetadataDefinition**](file_metadata_definition.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_task_metadata_definition

> crate::models::TaskMetadataDefinition show_task_metadata_definition(metadata_definition_id)
Displays a specific task metadata definition.

**API Key Scope**: task_metadata_definitions / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metadata_definition_id** | **String** | Numeric ID or key of a metadata definition | [required] |

### Return type

[**crate::models::TaskMetadataDefinition**](task_metadata_definition.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

