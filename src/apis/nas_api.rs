/*
 * Nodeum API
 *
 * The Nodeum API makes it easy to tap into the digital data mesh that runs across your organisation. Make requests to our API endpoints and we’ll give you everything you need to interconnect your business workflows with your storage.  All production API requests are made to:  http://nodeumhostname/api/  The current production version of the API is v1.   **REST** The Nodeum API is a RESTful API. This means that the API is designed to allow you to get, create, update, & delete objects with the HTTP verbs GET, POST, PUT, PATCH, & DELETE.  **JSON** The Nodeum API speaks exclusively in JSON. This means that you should always set the Content-Type header to application/json to ensure that your requests are properly accepted and processed by the API.  **Authentication** All API calls require user-password authentication.   **Cross-Origin Resource Sharing** The Nodeum API supports CORS for communicating from Javascript for these endpoints. You will need to specify an Origin URI when creating your application to allow for CORS to be whitelisted for your domain.   **Pagination** Some endpoints such as File Listing return a potentially lengthy array of objects. In order to keep the response sizes manageable the API will take advantage of pagination. Pagination is a mechanism for returning a subset of the results for a request and allowing for subsequent requests to “page” through the rest of the results until the end is reached. Paginated endpoints follow a standard interface that accepts two query parameters, limit and offset, and return a payload that follows a standard form. These parameters names and their behavior are borrowed from SQL LIMIT and OFFSET keywords.  **Versioning** The Nodeum API is constantly being worked on to add features, make improvements, and fix bugs. This means that you should expect changes to be introduced and documented.   However, there are some changes or additions that are considered backwards-compatible and your applications should be flexible enough to handle them. These include:  - Adding new endpoints to the API - Adding new attributes to the response of an existing endpoint - Changing the order of attributes of responses (JSON by definition is an object of unordered key/value pairs)  **Filter parameters** When browsing a list of items, multiple filter parameters may be applied. Some operators can be added to the value as a prefix:  - `=` value is equal. Default operator, may be omitted  - `!=` value is different  - `>` greater than  - `>=` greater than or equal  - `<` lower than  - `>=` lower than or equal  - `><` included in list, items should be separated by `|`  - `!><` not included in list, items should be separated by `|`  - `~` pattern matching, may include `%` (any characters) and `_` (one character)  - `!~` pattern not matching, may include `%` (any characters) and `_` (one character)  
 *
 * The version of the OpenAPI document: 2.1.0
 * Contact: info@nodeum.io
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct NasApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> NasApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> NasApiClient<C> {
        NasApiClient {
            configuration,
        }
    }
}

pub trait NasApi {
    fn create_nas(&self, nas_body: crate::models::Nas) -> Box<dyn Future<Item = crate::models::Nas, Error = Error<serde_json::Value>>>;
    fn destroy_nas(&self, nas_id: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn index_nas(&self, limit: Option<i32>, offset: Option<i32>, sort_by: Option<Vec<String>>, id: Option<&str>, name: Option<&str>, comment: Option<&str>, host: Option<&str>, _type: Option<&str>, price: Option<&str>) -> Box<dyn Future<Item = crate::models::NasCollection, Error = Error<serde_json::Value>>>;
    fn show_nas(&self, nas_id: &str) -> Box<dyn Future<Item = crate::models::Nas, Error = Error<serde_json::Value>>>;
    fn update_nas(&self, nas_id: &str, nas_body: crate::models::Nas) -> Box<dyn Future<Item = crate::models::Nas, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>NasApi for NasApiClient<C> {
    fn create_nas(&self, nas_body: crate::models::Nas) -> Box<dyn Future<Item = crate::models::Nas, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/nas".to_string())
            .with_auth(__internal_request::Auth::Basic)
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
        ;
        req = req.with_body_param(nas_body);

        req.execute(self.configuration.borrow())
    }

    fn destroy_nas(&self, nas_id: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/nas/{nas_id}".to_string())
            .with_auth(__internal_request::Auth::Basic)
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
        ;
        req = req.with_path_param("nas_id".to_string(), nas_id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn index_nas(&self, limit: Option<i32>, offset: Option<i32>, sort_by: Option<Vec<String>>, id: Option<&str>, name: Option<&str>, comment: Option<&str>, host: Option<&str>, _type: Option<&str>, price: Option<&str>) -> Box<dyn Future<Item = crate::models::NasCollection, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/nas".to_string())
            .with_auth(__internal_request::Auth::Basic)
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
        ;
        if let Some(ref s) = limit {
            req = req.with_query_param("limit".to_string(), s.to_string());
        }
        if let Some(ref s) = offset {
            req = req.with_query_param("offset".to_string(), s.to_string());
        }
        if let Some(ref s) = sort_by {
            req = req.with_query_param("sort_by".to_string(), s.join(",").to_string());
        }
        if let Some(ref s) = id {
            req = req.with_query_param("id".to_string(), s.to_string());
        }
        if let Some(ref s) = name {
            req = req.with_query_param("name".to_string(), s.to_string());
        }
        if let Some(ref s) = comment {
            req = req.with_query_param("comment".to_string(), s.to_string());
        }
        if let Some(ref s) = host {
            req = req.with_query_param("host".to_string(), s.to_string());
        }
        if let Some(ref s) = _type {
            req = req.with_query_param("type".to_string(), s.to_string());
        }
        if let Some(ref s) = price {
            req = req.with_query_param("price".to_string(), s.to_string());
        }

        req.execute(self.configuration.borrow())
    }

    fn show_nas(&self, nas_id: &str) -> Box<dyn Future<Item = crate::models::Nas, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/nas/{nas_id}".to_string())
            .with_auth(__internal_request::Auth::Basic)
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
        ;
        req = req.with_path_param("nas_id".to_string(), nas_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn update_nas(&self, nas_id: &str, nas_body: crate::models::Nas) -> Box<dyn Future<Item = crate::models::Nas, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/nas/{nas_id}".to_string())
            .with_auth(__internal_request::Auth::Basic)
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "Authorization".to_owned(),
            }))
        ;
        req = req.with_path_param("nas_id".to_string(), nas_id.to_string());
        req = req.with_body_param(nas_body);

        req.execute(self.configuration.borrow())
    }

}