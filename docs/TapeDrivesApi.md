# \TapeDrivesApi

All URIs are relative to *http://localhost/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tape_drive_by_tape_library**](TapeDrivesApi.md#create_tape_drive_by_tape_library) | **Post** /tape_libraries/{tape_library_id}/tape_drives | Creates a new tape drive.
[**destroy_tape_drive**](TapeDrivesApi.md#destroy_tape_drive) | **Delete** /tape_drives/{tape_drive_id} | Destroys a specific tape drive.
[**destroy_tape_drive_by_tape_library**](TapeDrivesApi.md#destroy_tape_drive_by_tape_library) | **Delete** /tape_libraries/{tape_library_id}/tape_drives/{tape_drive_id} | Destroys a specific tape drive.
[**index_tape_drive_devices**](TapeDrivesApi.md#index_tape_drive_devices) | **Get** /tape_libraries/{tape_library_id}/tape_drives/-/devices | Lists tape drives devices.
[**index_tape_drives**](TapeDrivesApi.md#index_tape_drives) | **Get** /tape_drives | Lists all tape drives.
[**index_tape_drives_by_tape_library**](TapeDrivesApi.md#index_tape_drives_by_tape_library) | **Get** /tape_libraries/{tape_library_id}/tape_drives | Lists all tape drives.
[**show_tape_drive**](TapeDrivesApi.md#show_tape_drive) | **Get** /tape_drives/{tape_drive_id} | Displays a specific tape drive.
[**show_tape_drive_by_tape_library**](TapeDrivesApi.md#show_tape_drive_by_tape_library) | **Get** /tape_libraries/{tape_library_id}/tape_drives/{tape_drive_id} | Displays a specific tape drive.
[**update_tape_drive**](TapeDrivesApi.md#update_tape_drive) | **Put** /tape_drives/{tape_drive_id} | Updates a specific tape drive.
[**update_tape_drive_by_tape_library**](TapeDrivesApi.md#update_tape_drive_by_tape_library) | **Put** /tape_libraries/{tape_library_id}/tape_drives/{tape_drive_id} | Updates a specific tape drive.



## create_tape_drive_by_tape_library

> crate::models::TapeDrive create_tape_drive_by_tape_library(tape_library_id, tape_drive_body)
Creates a new tape drive.

**API Key Scope**: tape_drives / create

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tape_library_id** | **String** | Numeric ID, serial, or name of tape library. | [required] |
**tape_drive_body** | [**TapeDrive**](TapeDrive.md) |  | [required] |

### Return type

[**crate::models::TapeDrive**](tape_drive.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_tape_drive

> destroy_tape_drive(tape_drive_id)
Destroys a specific tape drive.

**API Key Scope**: tape_drives / destroy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tape_drive_id** | **String** | Numeric ID, serial, or name of tape drive. | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_tape_drive_by_tape_library

> destroy_tape_drive_by_tape_library(tape_library_id, tape_drive_id)
Destroys a specific tape drive.

**API Key Scope**: tape_drives / destroy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tape_library_id** | **String** | Numeric ID, serial, or name of tape library. | [required] |
**tape_drive_id** | **String** | Numeric ID, serial, or name of tape drive. | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_tape_drive_devices

> crate::models::TapeDriveDeviceCollection index_tape_drive_devices(tape_library_id, job_id)
Lists tape drives devices.

**API Key Scope**: tape_drives / devices

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tape_library_id** | **String** | Numeric ID, serial, or name of tape library. | [required] |
**job_id** | **String** | ID of active job | [required] |

### Return type

[**crate::models::TapeDriveDeviceCollection**](tape_drive_device_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, queued, working, failed, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_tape_drives

> crate::models::TapeDriveCollection index_tape_drives(limit, offset, sort_by, id, tape_library_id, name, serial, comment, scsi_address, vendor, product, firmware, device, sgdevice, libso, acs, lsm, panel, transport, status, full, mount_count, use_to, use_by, barcode, task_id, use_file_processed_size, use_file_size_to_process, use_file_name_processed, bandwidth)
Lists all tape drives.

**API Key Scope**: tape_drives / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Sort results by attribute.  Can sort on multiple attributes, separated by `|`. Order direction can be suffixing the attribute by either `:asc` (default) or `:desc`. |  |
**id** | Option<**String**> | Filter on id |  |
**tape_library_id** | Option<**String**> | Filter on tape library id |  |
**name** | Option<**String**> | Filter on name |  |
**serial** | Option<**String**> | Filter on serial |  |
**comment** | Option<**String**> | Filter on comment |  |
**scsi_address** | Option<**String**> | Filter on scsi address |  |
**vendor** | Option<**String**> | Filter on vendor |  |
**product** | Option<**String**> | Filter on product |  |
**firmware** | Option<**String**> | Filter on firmware |  |
**device** | Option<**String**> | Filter on device |  |
**sgdevice** | Option<**String**> | Filter on sgdevice |  |
**libso** | Option<**String**> | Filter on libso |  |
**acs** | Option<**String**> | Filter on acs |  |
**lsm** | Option<**String**> | Filter on lsm |  |
**panel** | Option<**String**> | Filter on panel |  |
**transport** | Option<**String**> | Filter on transport |  |
**status** | Option<**String**> | Filter on status |  |
**full** | Option<**String**> | Filter on full |  |
**mount_count** | Option<**String**> | Filter on mount count |  |
**use_to** | Option<**String**> | Filter on use to |  |
**use_by** | Option<**String**> | Filter on use by |  |
**barcode** | Option<**String**> | Filter on barcode |  |
**task_id** | Option<**String**> | Filter on task id |  |
**use_file_processed_size** | Option<**String**> | Filter on use file processed size |  |
**use_file_size_to_process** | Option<**String**> | Filter on use file size to process |  |
**use_file_name_processed** | Option<**String**> | Filter on use file name processed |  |
**bandwidth** | Option<**String**> | Filter on bandwidth |  |

### Return type

[**crate::models::TapeDriveCollection**](tape_drive_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_tape_drives_by_tape_library

> crate::models::TapeDriveCollection index_tape_drives_by_tape_library(tape_library_id, limit, offset, sort_by, id, name, serial, comment, scsi_address, vendor, product, firmware, device, sgdevice, libso, acs, lsm, panel, transport, status, full, mount_count, use_to, use_by, barcode, task_id, use_file_processed_size, use_file_size_to_process, use_file_name_processed, bandwidth)
Lists all tape drives.

**API Key Scope**: tape_drives / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tape_library_id** | **String** | Numeric ID, serial, or name of tape library. | [required] |
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Sort results by attribute.  Can sort on multiple attributes, separated by `|`. Order direction can be suffixing the attribute by either `:asc` (default) or `:desc`. |  |
**id** | Option<**String**> | Filter on id |  |
**name** | Option<**String**> | Filter on name |  |
**serial** | Option<**String**> | Filter on serial |  |
**comment** | Option<**String**> | Filter on comment |  |
**scsi_address** | Option<**String**> | Filter on scsi address |  |
**vendor** | Option<**String**> | Filter on vendor |  |
**product** | Option<**String**> | Filter on product |  |
**firmware** | Option<**String**> | Filter on firmware |  |
**device** | Option<**String**> | Filter on device |  |
**sgdevice** | Option<**String**> | Filter on sgdevice |  |
**libso** | Option<**String**> | Filter on libso |  |
**acs** | Option<**String**> | Filter on acs |  |
**lsm** | Option<**String**> | Filter on lsm |  |
**panel** | Option<**String**> | Filter on panel |  |
**transport** | Option<**String**> | Filter on transport |  |
**status** | Option<**String**> | Filter on status |  |
**full** | Option<**String**> | Filter on full |  |
**mount_count** | Option<**String**> | Filter on mount count |  |
**use_to** | Option<**String**> | Filter on use to |  |
**use_by** | Option<**String**> | Filter on use by |  |
**barcode** | Option<**String**> | Filter on barcode |  |
**task_id** | Option<**String**> | Filter on task id |  |
**use_file_processed_size** | Option<**String**> | Filter on use file processed size |  |
**use_file_size_to_process** | Option<**String**> | Filter on use file size to process |  |
**use_file_name_processed** | Option<**String**> | Filter on use file name processed |  |
**bandwidth** | Option<**String**> | Filter on bandwidth |  |

### Return type

[**crate::models::TapeDriveCollection**](tape_drive_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_tape_drive

> crate::models::TapeDrive show_tape_drive(tape_drive_id)
Displays a specific tape drive.

**API Key Scope**: tape_drives / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tape_drive_id** | **String** | Numeric ID, serial, or name of tape drive. | [required] |

### Return type

[**crate::models::TapeDrive**](tape_drive.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_tape_drive_by_tape_library

> crate::models::TapeDrive show_tape_drive_by_tape_library(tape_library_id, tape_drive_id)
Displays a specific tape drive.

**API Key Scope**: tape_drives / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tape_library_id** | **String** | Numeric ID, serial, or name of tape library. | [required] |
**tape_drive_id** | **String** | Numeric ID, serial, or name of tape drive. | [required] |

### Return type

[**crate::models::TapeDrive**](tape_drive.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tape_drive

> crate::models::TapeDrive update_tape_drive(tape_drive_id, tape_drive_body)
Updates a specific tape drive.

**API Key Scope**: tape_drives / update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tape_drive_id** | **String** | Numeric ID, serial, or name of tape drive. | [required] |
**tape_drive_body** | [**TapeDrive**](TapeDrive.md) |  | [required] |

### Return type

[**crate::models::TapeDrive**](tape_drive.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tape_drive_by_tape_library

> crate::models::TapeDrive update_tape_drive_by_tape_library(tape_library_id, tape_drive_id, tape_drive_body)
Updates a specific tape drive.

**API Key Scope**: tape_drives / update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tape_library_id** | **String** | Numeric ID, serial, or name of tape library. | [required] |
**tape_drive_id** | **String** | Numeric ID, serial, or name of tape drive. | [required] |
**tape_drive_body** | [**TapeDrive**](TapeDrive.md) |  | [required] |

### Return type

[**crate::models::TapeDrive**](tape_drive.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

