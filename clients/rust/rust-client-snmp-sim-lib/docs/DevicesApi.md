# \DevicesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**devices_get**](DevicesApi.md#devices_get) | **GET** /devices | List managed devices
[**devices_id_delete**](DevicesApi.md#devices_id_delete) | **DELETE** /devices/{id} | Delete managed device by ID
[**devices_id_get**](DevicesApi.md#devices_id_get) | **GET** /devices/{id} | Get managed device by ID
[**devices_id_put**](DevicesApi.md#devices_id_put) | **PUT** /devices/{id} | Update managed device
[**devices_id_start_put**](DevicesApi.md#devices_id_start_put) | **PUT** /devices/{id}/start | Start an existing managed device
[**devices_id_stop_put**](DevicesApi.md#devices_id_stop_put) | **PUT** /devices/{id}/stop | Stop an existing managed device
[**devices_post**](DevicesApi.md#devices_post) | **POST** /devices | Create a new managed device



## devices_get

> Vec<crate::models::ResponseDevice> devices_get(page, page_size)
List managed devices

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i64**> | Page index starts from zero, default value is 1. |  |
**page_size** | Option<**i64**> | Number of results on a page, default value is 20. |  |

### Return type

[**Vec<crate::models::ResponseDevice>**](ResponseDevice.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## devices_id_delete

> crate::models::ResponseDevice devices_id_delete(id)
Delete managed device by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::ResponseDevice**](ResponseDevice.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## devices_id_get

> crate::models::ResponseDevice devices_id_get(id)
Get managed device by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::ResponseDevice**](ResponseDevice.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## devices_id_put

> crate::models::ResponseDevice devices_id_put(id, body)
Update managed device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**RequestDevice**](RequestDevice.md) |  | [required] |

### Return type

[**crate::models::ResponseDevice**](ResponseDevice.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## devices_id_start_put

> bool devices_id_start_put(id)
Start an existing managed device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## devices_id_stop_put

> bool devices_id_stop_put(id)
Stop an existing managed device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## devices_post

> crate::models::ResponseDevice devices_post(body)
Create a new managed device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**RequestDevice**](RequestDevice.md) |  | [required] |

### Return type

[**crate::models::ResponseDevice**](ResponseDevice.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

