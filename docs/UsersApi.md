# \UsersApi

All URIs are relative to *http://localhost/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_api_key**](UsersApi.md#create_api_key) | **Post** /users/me/api_keys | Creates a new API Key for current user.
[**create_configuration**](UsersApi.md#create_configuration) | **Post** /users/me/configurations | Creates a new configuration value for current user.
[**destroy_api_key**](UsersApi.md#destroy_api_key) | **Delete** /users/me/api_keys/{api_key_id} | Destroys a specific API Key.
[**destroy_configuration**](UsersApi.md#destroy_configuration) | **Delete** /users/me/configurations/{configuration_id} | Destroys a specific configuration value.
[**index_api_keys**](UsersApi.md#index_api_keys) | **Get** /users/me/api_keys | Lists all API keys of current user.
[**index_configurations**](UsersApi.md#index_configurations) | **Get** /users/me/configurations | Lists all configurations of current user.
[**index_system_groups**](UsersApi.md#index_system_groups) | **Get** /groups/-/systems | List all system groups.
[**index_system_users**](UsersApi.md#index_system_users) | **Get** /users/-/systems | List all system users.
[**show_api_key**](UsersApi.md#show_api_key) | **Get** /users/me/api_keys/{api_key_id} | Displays a specific API Key with its scopes.
[**show_configuration**](UsersApi.md#show_configuration) | **Get** /users/me/configurations/{configuration_id} | Displays a specific configuration value.
[**update_api_key**](UsersApi.md#update_api_key) | **Put** /users/me/api_keys/{api_key_id} | Updates a specific API Key.
[**update_configuration**](UsersApi.md#update_configuration) | **Put** /users/me/configurations/{configuration_id} | Updates a specific configuration value.



## create_api_key

> crate::models::ApiKeyFull create_api_key(api_key_body)
Creates a new API Key for current user.

**API Key Scope**: api_keys / create

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_body** | [**ApiKeyFull**](ApiKeyFull.md) |  | [required] |

### Return type

[**crate::models::ApiKeyFull**](api_key_full.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_configuration

> crate::models::UserConfiguration create_configuration(configuration_body)
Creates a new configuration value for current user.

**API Key Scope**: configurations / create

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**configuration_body** | [**UserConfiguration**](UserConfiguration.md) |  | [required] |

### Return type

[**crate::models::UserConfiguration**](user_configuration.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_api_key

> destroy_api_key(api_key_id)
Destroys a specific API Key.

**API Key Scope**: api_keys / destroy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_id** | **i32** | Numeric ID of API Key. | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_configuration

> destroy_configuration(configuration_id)
Destroys a specific configuration value.

**API Key Scope**: configurations / destroy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**configuration_id** | **String** | Numeric ID, or key of configuration. | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_api_keys

> crate::models::ApiKeyCollection index_api_keys(limit, offset)
Lists all API keys of current user.

**API Key Scope**: api_keys / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |

### Return type

[**crate::models::ApiKeyCollection**](api_key_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_configurations

> crate::models::UserConfigurationCollection index_configurations(limit, offset)
Lists all configurations of current user.

**API Key Scope**: configurations / index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The number of items to display for pagination. |  |
**offset** | Option<**i32**> | The number of items to skip for pagination. |  |

### Return type

[**crate::models::UserConfigurationCollection**](user_configuration_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_system_groups

> crate::models::SystemGroupCollection index_system_groups()
List all system groups.

**API Key Scope**: groups / systems

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SystemGroupCollection**](system_group_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_system_users

> crate::models::SystemUserCollection index_system_users()
List all system users.

**API Key Scope**: users / systems

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SystemUserCollection**](system_user_collection.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_api_key

> crate::models::ApiKeyFull show_api_key(api_key_id)
Displays a specific API Key with its scopes.

**API Key Scope**: api_keys / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_id** | **i32** | Numeric ID of API Key. | [required] |

### Return type

[**crate::models::ApiKeyFull**](api_key_full.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_configuration

> crate::models::UserConfiguration show_configuration(configuration_id)
Displays a specific configuration value.

**API Key Scope**: configurations / show

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**configuration_id** | **String** | Numeric ID, or key of configuration. | [required] |

### Return type

[**crate::models::UserConfiguration**](user_configuration.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_key

> crate::models::ApiKeyFull update_api_key(api_key_id, api_key_body)
Updates a specific API Key.

**API Key Scope**: api_keys / update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_id** | **i32** | Numeric ID of API Key. | [required] |
**api_key_body** | [**ApiKeyFull**](ApiKeyFull.md) |  | [required] |

### Return type

[**crate::models::ApiKeyFull**](api_key_full.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_configuration

> crate::models::UserConfiguration update_configuration(configuration_id, configuration_body)
Updates a specific configuration value.

**API Key Scope**: configurations / update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**configuration_id** | **String** | Numeric ID, or key of configuration. | [required] |
**configuration_body** | [**UserConfiguration**](UserConfiguration.md) |  | [required] |

### Return type

[**crate::models::UserConfiguration**](user_configuration.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

