/*
 * Nodeum API
 *
 * The Nodeum API makes it easy to tap into the digital data mesh that runs across your organisation. Make requests to our API endpoints and we’ll give you everything you need to interconnect your business workflows with your storage.  All production API requests are made to:  http://nodeumhostname/api/  The current production version of the API is v1.   **REST** The Nodeum API is a RESTful API. This means that the API is designed to allow you to get, create, update, & delete objects with the HTTP verbs GET, POST, PUT, PATCH, & DELETE.  **JSON** The Nodeum API speaks exclusively in JSON. This means that you should always set the Content-Type header to application/json to ensure that your requests are properly accepted and processed by the API.  **Authentication** All API calls require user-password authentication.   **Cross-Origin Resource Sharing** The Nodeum API supports CORS for communicating from Javascript for these endpoints. You will need to specify an Origin URI when creating your application to allow for CORS to be whitelisted for your domain.   **Pagination** Some endpoints such as File Listing return a potentially lengthy array of objects. In order to keep the response sizes manageable the API will take advantage of pagination. Pagination is a mechanism for returning a subset of the results for a request and allowing for subsequent requests to “page” through the rest of the results until the end is reached. Paginated endpoints follow a standard interface that accepts two query parameters, limit and offset, and return a payload that follows a standard form. These parameters names and their behavior are borrowed from SQL LIMIT and OFFSET keywords.  **Versioning** The Nodeum API is constantly being worked on to add features, make improvements, and fix bugs. This means that you should expect changes to be introduced and documented.   However, there are some changes or additions that are considered backwards-compatible and your applications should be flexible enough to handle them. These include:  - Adding new endpoints to the API - Adding new attributes to the response of an existing endpoint - Changing the order of attributes of responses (JSON by definition is an object of unordered key/value pairs)  **Filter parameters** When browsing a list of items, multiple filter parameters may be applied. Some operators can be added to the value as a prefix:  - `=` value is equal. Default operator, may be omitted  - `!=` value is different  - `>` greater than  - `>=` greater than or equal  - `<` lower than  - `>=` lower than or equal  - `><` included in list, items should be separated by `|`  - `!><` not included in list, items should be separated by `|`  - `~` pattern matching, may include `%` (any characters) and `_` (one character)  - `!~` pattern not matching, may include `%` (any characters) and `_` (one character)  
 *
 * The version of the OpenAPI document: 2.1.0
 * Contact: info@nodeum.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ByDateFacet {
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "files_count", skip_serializing_if = "Option::is_none")]
    pub files_count: Option<i32>,
    #[serde(rename = "file_size_sum", skip_serializing_if = "Option::is_none")]
    pub file_size_sum: Option<i32>,
    #[serde(rename = "cost", skip_serializing_if = "Option::is_none")]
    pub cost: Option<f32>,
    #[serde(rename = "in_cache", skip_serializing_if = "Option::is_none")]
    pub in_cache: Option<crate::models::FileFacet>,
    #[serde(rename = "less_1_week", skip_serializing_if = "Option::is_none")]
    pub less_1_week: Option<crate::models::FileFacet>,
    #[serde(rename = "less_1_month", skip_serializing_if = "Option::is_none")]
    pub less_1_month: Option<crate::models::FileFacet>,
    #[serde(rename = "less_3_months", skip_serializing_if = "Option::is_none")]
    pub less_3_months: Option<crate::models::FileFacet>,
    #[serde(rename = "less_6_months", skip_serializing_if = "Option::is_none")]
    pub less_6_months: Option<crate::models::FileFacet>,
    #[serde(rename = "less_1_year", skip_serializing_if = "Option::is_none")]
    pub less_1_year: Option<crate::models::FileFacet>,
    #[serde(rename = "less_2_years", skip_serializing_if = "Option::is_none")]
    pub less_2_years: Option<crate::models::FileFacet>,
    #[serde(rename = "more_2_years", skip_serializing_if = "Option::is_none")]
    pub more_2_years: Option<crate::models::FileFacet>,
}

impl ByDateFacet {
    pub fn new() -> ByDateFacet {
        ByDateFacet {
            count: None,
            files_count: None,
            file_size_sum: None,
            cost: None,
            in_cache: None,
            less_1_week: None,
            less_1_month: None,
            less_3_months: None,
            less_6_months: None,
            less_1_year: None,
            less_2_years: None,
            more_2_years: None,
        }
    }
}


