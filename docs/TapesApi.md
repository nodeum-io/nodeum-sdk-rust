# \TapesApi

All URIs are relative to *http://localhost/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**index_tape_stats**](TapesApi.md#index_tape_stats) | **Get** /tape_stats | List all tape statistics.
[**index_tapes**](TapesApi.md#index_tapes) | **Get** /tapes | Lists all tapes.
[**index_tapes_by_pool**](TapesApi.md#index_tapes_by_pool) | **Get** /pools/{pool_id}/tapes | Lists all tapes.
[**index_tapes_by_tape_library**](TapesApi.md#index_tapes_by_tape_library) | **Get** /tape_libraries/{tape_library_id}/tapes | Lists all tapes.
[**mount_status_tape**](TapesApi.md#mount_status_tape) | **Get** /tapes/{tape_id}/mount | Get mount status of Tape.
[**mount_status_tape_by_pool**](TapesApi.md#mount_status_tape_by_pool) | **Get** /pools/{pool_id}/tapes/{tape_id}/mount | Get mount status of Tape.
[**mount_status_tape_by_tape_library**](TapesApi.md#mount_status_tape_by_tape_library) | **Get** /tape_libraries/{tape_library_id}/tapes/{tape_id}/mount | Get mount status of Tape.
[**show_tape**](TapesApi.md#show_tape) | **Get** /tapes/{tape_id} | Displays a specific tape.
[**show_tape_by_pool**](TapesApi.md#show_tape_by_pool) | **Get** /pools/{pool_id}/tapes/{tape_id} | Displays a specific tape.
[**show_tape_by_tape_library**](TapesApi.md#show_tape_by_tape_library) | **Get** /tape_libraries/{tape_library_id}/tapes/{tape_id} | Displays a specific tape.
[**show_tape_stat**](TapesApi.md#show_tape_stat) | **Get** /tapes/{tape_id}/tape_stat | Display statistic for a specific tape.
[**show_tape_stat_by_pool**](TapesApi.md#show_tape_stat_by_pool) | **Get** /pools/{pool_id}/tapes/{tape_id}/tape_stat | Display statistic for a specific tape.
[**show_tape_stat_by_tape_library**](TapesApi.md#show_tape_stat_by_tape_library) | **Get** /tape_libraries/{tape_library_id}/tapes/{tape_id}/tape_stat | Display statistic for a specific tape.



## index_tape_stats

> crate::models::TapeStatCollection index_tape_stats(limit, offset)
List all tape statistics.

**API Key Scope**: tape_stats / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |

### Return type

[**crate::models::TapeStatCollection**](tape_stat_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_tapes

> crate::models::TapeCollection index_tapes(limit, offset, sort_by, id, tape_library_id, pool_id, barcode, location, _type, locked, scratch, cleaning, write_protect, mounted, ejected, known, mount_count, date_in, date_move, free, max, last_size_update, last_maintenance, last_repack, repack_status, hash, force_import_type, need_to_check)
Lists all tapes.

**API Key Scope**: tapes / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Sort results by attribute.  Can sort on multiple attributes, separated by `|`. Order direction can be suffixing the attribute by either `:asc` (default) or `:desc`. |  |
**id** | Option<**String**> | Filter on id |  |
**tape_library_id** | Option<**String**> | Filter on tape library id |  |
**pool_id** | Option<**String**> | Filter on a pool id |  |
**barcode** | Option<**String**> | Filter on barcode |  |
**location** | Option<**String**> | Filter on location |  |
**_type** | Option<**String**> | Filter on type |  |
**locked** | Option<**String**> | Filter on locked |  |
**scratch** | Option<**String**> | Filter on scratch |  |
**cleaning** | Option<**String**> | Filter on cleaning |  |
**write_protect** | Option<**String**> | Filter on write protect |  |
**mounted** | Option<**String**> | Filter on mounted |  |
**ejected** | Option<**String**> | Filter on ejected |  |
**known** | Option<**String**> | Filter on known |  |
**mount_count** | Option<**String**> | Filter on mount count |  |
**date_in** | Option<**String**> | Filter on date in |  |
**date_move** | Option<**String**> | Filter on date move |  |
**free** | Option<**String**> | Filter on free |  |
**max** | Option<**String**> | Filter on max |  |
**last_size_update** | Option<**String**> | Filter on last size update |  |
**last_maintenance** | Option<**String**> | Filter on last maintenance |  |
**last_repack** | Option<**String**> | Filter on last repack |  |
**repack_status** | Option<**String**> | Filter on repack status |  |
**hash** | Option<**String**> | Filter on hash |  |
**force_import_type** | Option<**String**> | Filter on force import type |  |
**need_to_check** | Option<**String**> | Filter on need to check |  |

### Return type

[**crate::models::TapeCollection**](tape_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_tapes_by_pool

> crate::models::TapeCollection index_tapes_by_pool(pool_id, limit, offset, sort_by, id, tape_library_id, barcode, location, _type, locked, scratch, cleaning, write_protect, mounted, ejected, known, mount_count, date_in, date_move, free, max, last_size_update, last_maintenance, last_repack, repack_status, hash, force_import_type, need_to_check)
Lists all tapes.

**API Key Scope**: tapes / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Sort results by attribute.  Can sort on multiple attributes, separated by `|`. Order direction can be suffixing the attribute by either `:asc` (default) or `:desc`. |  |
**id** | Option<**String**> | Filter on id |  |
**tape_library_id** | Option<**String**> | Filter on tape library id |  |
**barcode** | Option<**String**> | Filter on barcode |  |
**location** | Option<**String**> | Filter on location |  |
**_type** | Option<**String**> | Filter on type |  |
**locked** | Option<**String**> | Filter on locked |  |
**scratch** | Option<**String**> | Filter on scratch |  |
**cleaning** | Option<**String**> | Filter on cleaning |  |
**write_protect** | Option<**String**> | Filter on write protect |  |
**mounted** | Option<**String**> | Filter on mounted |  |
**ejected** | Option<**String**> | Filter on ejected |  |
**known** | Option<**String**> | Filter on known |  |
**mount_count** | Option<**String**> | Filter on mount count |  |
**date_in** | Option<**String**> | Filter on date in |  |
**date_move** | Option<**String**> | Filter on date move |  |
**free** | Option<**String**> | Filter on free |  |
**max** | Option<**String**> | Filter on max |  |
**last_size_update** | Option<**String**> | Filter on last size update |  |
**last_maintenance** | Option<**String**> | Filter on last maintenance |  |
**last_repack** | Option<**String**> | Filter on last repack |  |
**repack_status** | Option<**String**> | Filter on repack status |  |
**hash** | Option<**String**> | Filter on hash |  |
**force_import_type** | Option<**String**> | Filter on force import type |  |
**need_to_check** | Option<**String**> | Filter on need to check |  |

### Return type

[**crate::models::TapeCollection**](tape_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_tapes_by_tape_library

> crate::models::TapeCollection index_tapes_by_tape_library(tape_library_id, limit, offset, sort_by, id, pool_id, barcode, location, _type, locked, scratch, cleaning, write_protect, mounted, ejected, known, mount_count, date_in, date_move, free, max, last_size_update, last_maintenance, last_repack, repack_status, hash, force_import_type, need_to_check)
Lists all tapes.

**API Key Scope**: tapes / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tape_library_id** | **String** | Numeric ID, serial, or name of tape library. | [required] |
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |
**sort_by** | Option<[**Vec<String>**](String.md)> | Sort results by attribute.  Can sort on multiple attributes, separated by `|`. Order direction can be suffixing the attribute by either `:asc` (default) or `:desc`. |  |
**id** | Option<**String**> | Filter on id |  |
**pool_id** | Option<**String**> | Filter on a pool id |  |
**barcode** | Option<**String**> | Filter on barcode |  |
**location** | Option<**String**> | Filter on location |  |
**_type** | Option<**String**> | Filter on type |  |
**locked** | Option<**String**> | Filter on locked |  |
**scratch** | Option<**String**> | Filter on scratch |  |
**cleaning** | Option<**String**> | Filter on cleaning |  |
**write_protect** | Option<**String**> | Filter on write protect |  |
**mounted** | Option<**String**> | Filter on mounted |  |
**ejected** | Option<**String**> | Filter on ejected |  |
**known** | Option<**String**> | Filter on known |  |
**mount_count** | Option<**String**> | Filter on mount count |  |
**date_in** | Option<**String**> | Filter on date in |  |
**date_move** | Option<**String**> | Filter on date move |  |
**free** | Option<**String**> | Filter on free |  |
**max** | Option<**String**> | Filter on max |  |
**last_size_update** | Option<**String**> | Filter on last size update |  |
**last_maintenance** | Option<**String**> | Filter on last maintenance |  |
**last_repack** | Option<**String**> | Filter on last repack |  |
**repack_status** | Option<**String**> | Filter on repack status |  |
**hash** | Option<**String**> | Filter on hash |  |
**force_import_type** | Option<**String**> | Filter on force import type |  |
**need_to_check** | Option<**String**> | Filter on need to check |  |

### Return type

[**crate::models::TapeCollection**](tape_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mount_status_tape

> crate::models::MountStatus mount_status_tape(tape_id)
Get mount status of Tape.

**API Key Scope**: tapes / mount_status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tape_id** | **String** | Numeric ID, or barcode of tape. | [required] |

### Return type

[**crate::models::MountStatus**](mount_status.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mount_status_tape_by_pool

> crate::models::MountStatus mount_status_tape_by_pool(pool_id, tape_id)
Get mount status of Tape.

**API Key Scope**: tapes / mount_status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |
**tape_id** | **String** | Numeric ID, or barcode of tape. | [required] |

### Return type

[**crate::models::MountStatus**](mount_status.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mount_status_tape_by_tape_library

> crate::models::MountStatus mount_status_tape_by_tape_library(tape_library_id, tape_id)
Get mount status of Tape.

**API Key Scope**: tapes / mount_status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tape_library_id** | **String** | Numeric ID, serial, or name of tape library. | [required] |
**tape_id** | **String** | Numeric ID, or barcode of tape. | [required] |

### Return type

[**crate::models::MountStatus**](mount_status.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_tape

> crate::models::Tape show_tape(tape_id)
Displays a specific tape.

**API Key Scope**: tapes / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tape_id** | **String** | Numeric ID, or barcode of tape. | [required] |

### Return type

[**crate::models::Tape**](tape.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_tape_by_pool

> crate::models::Tape show_tape_by_pool(pool_id, tape_id)
Displays a specific tape.

**API Key Scope**: tapes / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |
**tape_id** | **String** | Numeric ID, or barcode of tape. | [required] |

### Return type

[**crate::models::Tape**](tape.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_tape_by_tape_library

> crate::models::Tape show_tape_by_tape_library(tape_library_id, tape_id)
Displays a specific tape.

**API Key Scope**: tapes / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tape_library_id** | **String** | Numeric ID, serial, or name of tape library. | [required] |
**tape_id** | **String** | Numeric ID, or barcode of tape. | [required] |

### Return type

[**crate::models::Tape**](tape.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_tape_stat

> crate::models::TapeStat show_tape_stat(tape_id)
Display statistic for a specific tape.

**API Key Scope**: tape_stats / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tape_id** | **String** | Numeric ID, or barcode of tape. | [required] |

### Return type

[**crate::models::TapeStat**](tape_stat.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_tape_stat_by_pool

> crate::models::TapeStat show_tape_stat_by_pool(pool_id, tape_id)
Display statistic for a specific tape.

**API Key Scope**: tape_stats / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | **String** | Numeric ID, or name of pool. | [required] |
**tape_id** | **String** | Numeric ID, or barcode of tape. | [required] |

### Return type

[**crate::models::TapeStat**](tape_stat.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_tape_stat_by_tape_library

> crate::models::TapeStat show_tape_stat_by_tape_library(tape_library_id, tape_id)
Display statistic for a specific tape.

**API Key Scope**: tape_stats / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tape_library_id** | **String** | Numeric ID, serial, or name of tape library. | [required] |
**tape_id** | **String** | Numeric ID, or barcode of tape. | [required] |

### Return type

[**crate::models::TapeStat**](tape_stat.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

