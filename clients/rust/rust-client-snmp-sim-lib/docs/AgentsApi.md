# \AgentsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**agents_get**](AgentsApi.md#agents_get) | **GET** /agents | List agents
[**agents_id_delete**](AgentsApi.md#agents_id_delete) | **DELETE** /agents/{id} | Delete agent by ID
[**agents_id_get**](AgentsApi.md#agents_id_get) | **GET** /agents/{id} | Get agent by ID
[**agents_id_put**](AgentsApi.md#agents_id_put) | **PUT** /agents/{id} | Update agent
[**agents_post**](AgentsApi.md#agents_post) | **POST** /agents | Create a new agent



## agents_get

> crate::models::ResponseAgents agents_get(page, page_size)
List agents

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i64**> | Page index starts from zero, default value is 1. |  |
**page_size** | Option<**i64**> | Number of results on a page, default value is 20. |  |

### Return type

[**crate::models::ResponseAgents**](ResponseAgents.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## agents_id_delete

> crate::models::ResponseAgent agents_id_delete(id)
Delete agent by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::ResponseAgent**](ResponseAgent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## agents_id_get

> crate::models::ResponseAgent agents_id_get(id)
Get agent by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::ResponseAgent**](ResponseAgent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## agents_id_put

> crate::models::ResponseAgent agents_id_put(id, body)
Update agent

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | [**RequestAgent**](RequestAgent.md) |  | [required] |

### Return type

[**crate::models::ResponseAgent**](ResponseAgent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## agents_post

> crate::models::ResponseAgent agents_post(body)
Create a new agent

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**RequestAgent**](RequestAgent.md) |  | [required] |

### Return type

[**crate::models::ResponseAgent**](ResponseAgent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

