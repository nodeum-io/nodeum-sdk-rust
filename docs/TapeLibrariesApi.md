# \TapeLibrariesApi

All URIs are relative to *http://localhost/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tape_library**](TapeLibrariesApi.md#create_tape_library) | **Post** /tape_libraries | Creates a new tape library.
[**destroy_tape_library**](TapeLibrariesApi.md#destroy_tape_library) | **Delete** /tape_libraries/{tape_library_id} | Destroys a specific tape library.
[**index_tape_libraries**](TapeLibrariesApi.md#index_tape_libraries) | **Get** /tape_libraries | Lists all tape libraries.
[**index_tape_library_devices**](TapeLibrariesApi.md#index_tape_library_devices) | **Get** /tape_libraries/-/devices | Lists tape libraries devices.
[**show_tape_library**](TapeLibrariesApi.md#show_tape_library) | **Get** /tape_libraries/{tape_library_id} | Displays a specific tape library.
[**update_tape_library**](TapeLibrariesApi.md#update_tape_library) | **Put** /tape_libraries/{tape_library_id} | Updates a specific tape library.



## create_tape_library

> crate::models::TapeLibrary create_tape_library(tape_library_body)
Creates a new tape library.

**API Key Scope**: tape_libraries / create

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tape_library_body** | [**TapeLibrary**](TapeLibrary.md) |  | [required] |

### Return type

[**crate::models::TapeLibrary**](tape_library.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_tape_library

> destroy_tape_library(tape_library_id)
Destroys a specific tape library.

**API Key Scope**: tape_libraries / destroy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tape_library_id** | **String** | Numeric ID, serial, or name of tape library. | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_tape_libraries

> crate::models::TapeLibraryCollection index_tape_libraries(limit, offset, sort_by, id, name, serial, comment, protocol, vendor, product, firmware, device, libso, acs, status, storage_slots, storage_slots_address, io_slots, io_slots_address, price)
Lists all tape libraries.

**API Key Scope**: tape_libraries / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Sort results by attribute.  Can sort on multiple attributes, separated by `|`. Order direction can be suffixing the attribute by either `:asc` (default) or `:desc`. |  |
**id** | Option<**String**> | Filter on id |  |
**name** | Option<**String**> | Filter on name |  |
**serial** | Option<**String**> | Filter on serial |  |
**comment** | Option<**String**> | Filter on comment |  |
**protocol** | Option<**String**> | Filter on protocol |  |
**vendor** | Option<**String**> | Filter on vendor |  |
**product** | Option<**String**> | Filter on product |  |
**firmware** | Option<**String**> | Filter on firmware |  |
**device** | Option<**String**> | Filter on device |  |
**libso** | Option<**String**> | Filter on libso |  |
**acs** | Option<**String**> | Filter on acs |  |
**status** | Option<**String**> | Filter on status |  |
**storage_slots** | Option<**String**> | Filter on storage slots |  |
**storage_slots_address** | Option<**String**> | Filter on storage slots address |  |
**io_slots** | Option<**String**> | Filter on io slots |  |
**io_slots_address** | Option<**String**> | Filter on io slots address |  |
**price** | Option<**String**> | Filter on price |  |

### Return type

[**crate::models::TapeLibraryCollection**](tape_library_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_tape_library_devices

> crate::models::TapeLibraryDeviceCollection index_tape_library_devices(job_id)
Lists tape libraries devices.

**API Key Scope**: tape_libraries / devices

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | ID of active job | [required] |

### Return type

[**crate::models::TapeLibraryDeviceCollection**](tape_library_device_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, queued, working, failed, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_tape_library

> crate::models::TapeLibrary show_tape_library(tape_library_id)
Displays a specific tape library.

**API Key Scope**: tape_libraries / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tape_library_id** | **String** | Numeric ID, serial, or name of tape library. | [required] |

### Return type

[**crate::models::TapeLibrary**](tape_library.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tape_library

> crate::models::TapeLibrary update_tape_library(tape_library_id, tape_library_body)
Updates a specific tape library.

**API Key Scope**: tape_libraries / update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tape_library_id** | **String** | Numeric ID, serial, or name of tape library. | [required] |
**tape_library_body** | [**TapeLibrary**](TapeLibrary.md) |  | [required] |

### Return type

[**crate::models::TapeLibrary**](tape_library.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

