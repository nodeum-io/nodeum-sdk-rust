use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    UriError(hyper::error::UriError),
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T> 
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError{
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError{
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

mod request;

mod cloud_buckets_api;
pub use self::cloud_buckets_api::{ CloudBucketsApi, CloudBucketsApiClient };
mod cloud_connectors_api;
pub use self::cloud_connectors_api::{ CloudConnectorsApi, CloudConnectorsApiClient };
mod containers_api;
pub use self::containers_api::{ ContainersApi, ContainersApiClient };
mod files_api;
pub use self::files_api::{ FilesApi, FilesApiClient };
mod mounts_api;
pub use self::mounts_api::{ MountsApi, MountsApiClient };
mod nas_api;
pub use self::nas_api::{ NasApi, NasApiClient };
mod nas_shares_api;
pub use self::nas_shares_api::{ NasSharesApi, NasSharesApiClient };
mod pools_api;
pub use self::pools_api::{ PoolsApi, PoolsApiClient };
mod statistics_api;
pub use self::statistics_api::{ StatisticsApi, StatisticsApiClient };
mod systems_api;
pub use self::systems_api::{ SystemsApi, SystemsApiClient };
mod tape_drives_api;
pub use self::tape_drives_api::{ TapeDrivesApi, TapeDrivesApiClient };
mod tape_libraries_api;
pub use self::tape_libraries_api::{ TapeLibrariesApi, TapeLibrariesApiClient };
mod tapes_api;
pub use self::tapes_api::{ TapesApi, TapesApiClient };
mod task_callbacks_api;
pub use self::task_callbacks_api::{ TaskCallbacksApi, TaskCallbacksApiClient };
mod task_destinations_api;
pub use self::task_destinations_api::{ TaskDestinationsApi, TaskDestinationsApiClient };
mod task_executions_api;
pub use self::task_executions_api::{ TaskExecutionsApi, TaskExecutionsApiClient };
mod task_metadata_api;
pub use self::task_metadata_api::{ TaskMetadataApi, TaskMetadataApiClient };
mod task_options_api;
pub use self::task_options_api::{ TaskOptionsApi, TaskOptionsApiClient };
mod task_schedules_api;
pub use self::task_schedules_api::{ TaskSchedulesApi, TaskSchedulesApiClient };
mod task_sources_api;
pub use self::task_sources_api::{ TaskSourcesApi, TaskSourcesApiClient };
mod tasks_api;
pub use self::tasks_api::{ TasksApi, TasksApiClient };
mod users_api;
pub use self::users_api::{ UsersApi, UsersApiClient };

pub mod configuration;
pub mod client;
