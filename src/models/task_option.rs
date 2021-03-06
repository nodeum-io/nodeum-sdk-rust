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
pub struct TaskOption {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl TaskOption {
    pub fn new() -> TaskOption {
        TaskOption {
            id: None,
            _type: None,
            value: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "file_checksum")]
    FileChecksum,
    #[serde(rename = "reset_file_date")]
    ResetFileDate,
    #[serde(rename = "reset_folder_date")]
    ResetFolderDate,
    #[serde(rename = "rename_to")]
    RenameTo,
    #[serde(rename = "execution_task_id")]
    ExecutionTaskId,
    #[serde(rename = "selected_drive")]
    SelectedDrive,
    #[serde(rename = "parallel_processing")]
    ParallelProcessing,
    #[serde(rename = "task_id")]
    TaskId,
    #[serde(rename = "full_report")]
    FullReport,
    #[serde(rename = "processing_order")]
    ProcessingOrder,
    #[serde(rename = "rehydration_usage_threshold")]
    RehydrationUsageThreshold,
    #[serde(rename = "rehydration_deleted_files_threshold")]
    RehydrationDeletedFilesThreshold,
    #[serde(rename = "learning_type")]
    LearningType,
    #[serde(rename = "learning_threshold")]
    LearningThreshold,
    #[serde(rename = "tape_to_scratch")]
    TapeToScratch,
    #[serde(rename = "eject_tape_after")]
    EjectTapeAfter,
    #[serde(rename = "barcodes")]
    Barcodes,
    #[serde(rename = "overwrite")]
    Overwrite,
    #[serde(rename = "soft_erase")]
    SoftErase,
    #[serde(rename = "task_template")]
    TaskTemplate,
}

