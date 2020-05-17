use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient {
    cloud_buckets_api: Box<dyn crate::apis::CloudBucketsApi>,
    cloud_connectors_api: Box<dyn crate::apis::CloudConnectorsApi>,
    containers_api: Box<dyn crate::apis::ContainersApi>,
    files_api: Box<dyn crate::apis::FilesApi>,
    mounts_api: Box<dyn crate::apis::MountsApi>,
    nas_api: Box<dyn crate::apis::NasApi>,
    nas_shares_api: Box<dyn crate::apis::NasSharesApi>,
    pools_api: Box<dyn crate::apis::PoolsApi>,
    statistics_api: Box<dyn crate::apis::StatisticsApi>,
    systems_api: Box<dyn crate::apis::SystemsApi>,
    tape_drives_api: Box<dyn crate::apis::TapeDrivesApi>,
    tape_libraries_api: Box<dyn crate::apis::TapeLibrariesApi>,
    tapes_api: Box<dyn crate::apis::TapesApi>,
    task_callbacks_api: Box<dyn crate::apis::TaskCallbacksApi>,
    task_destinations_api: Box<dyn crate::apis::TaskDestinationsApi>,
    task_executions_api: Box<dyn crate::apis::TaskExecutionsApi>,
    task_metadata_api: Box<dyn crate::apis::TaskMetadataApi>,
    task_options_api: Box<dyn crate::apis::TaskOptionsApi>,
    task_schedules_api: Box<dyn crate::apis::TaskSchedulesApi>,
    task_sources_api: Box<dyn crate::apis::TaskSourcesApi>,
    tasks_api: Box<dyn crate::apis::TasksApi>,
    users_api: Box<dyn crate::apis::UsersApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::Connect>(configuration: Configuration<C>) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            cloud_buckets_api: Box::new(crate::apis::CloudBucketsApiClient::new(rc.clone())),
            cloud_connectors_api: Box::new(crate::apis::CloudConnectorsApiClient::new(rc.clone())),
            containers_api: Box::new(crate::apis::ContainersApiClient::new(rc.clone())),
            files_api: Box::new(crate::apis::FilesApiClient::new(rc.clone())),
            mounts_api: Box::new(crate::apis::MountsApiClient::new(rc.clone())),
            nas_api: Box::new(crate::apis::NasApiClient::new(rc.clone())),
            nas_shares_api: Box::new(crate::apis::NasSharesApiClient::new(rc.clone())),
            pools_api: Box::new(crate::apis::PoolsApiClient::new(rc.clone())),
            statistics_api: Box::new(crate::apis::StatisticsApiClient::new(rc.clone())),
            systems_api: Box::new(crate::apis::SystemsApiClient::new(rc.clone())),
            tape_drives_api: Box::new(crate::apis::TapeDrivesApiClient::new(rc.clone())),
            tape_libraries_api: Box::new(crate::apis::TapeLibrariesApiClient::new(rc.clone())),
            tapes_api: Box::new(crate::apis::TapesApiClient::new(rc.clone())),
            task_callbacks_api: Box::new(crate::apis::TaskCallbacksApiClient::new(rc.clone())),
            task_destinations_api: Box::new(crate::apis::TaskDestinationsApiClient::new(rc.clone())),
            task_executions_api: Box::new(crate::apis::TaskExecutionsApiClient::new(rc.clone())),
            task_metadata_api: Box::new(crate::apis::TaskMetadataApiClient::new(rc.clone())),
            task_options_api: Box::new(crate::apis::TaskOptionsApiClient::new(rc.clone())),
            task_schedules_api: Box::new(crate::apis::TaskSchedulesApiClient::new(rc.clone())),
            task_sources_api: Box::new(crate::apis::TaskSourcesApiClient::new(rc.clone())),
            tasks_api: Box::new(crate::apis::TasksApiClient::new(rc.clone())),
            users_api: Box::new(crate::apis::UsersApiClient::new(rc.clone())),
        }
    }

    pub fn cloud_buckets_api(&self) -> &dyn crate::apis::CloudBucketsApi{
        self.cloud_buckets_api.as_ref()
    }

    pub fn cloud_connectors_api(&self) -> &dyn crate::apis::CloudConnectorsApi{
        self.cloud_connectors_api.as_ref()
    }

    pub fn containers_api(&self) -> &dyn crate::apis::ContainersApi{
        self.containers_api.as_ref()
    }

    pub fn files_api(&self) -> &dyn crate::apis::FilesApi{
        self.files_api.as_ref()
    }

    pub fn mounts_api(&self) -> &dyn crate::apis::MountsApi{
        self.mounts_api.as_ref()
    }

    pub fn nas_api(&self) -> &dyn crate::apis::NasApi{
        self.nas_api.as_ref()
    }

    pub fn nas_shares_api(&self) -> &dyn crate::apis::NasSharesApi{
        self.nas_shares_api.as_ref()
    }

    pub fn pools_api(&self) -> &dyn crate::apis::PoolsApi{
        self.pools_api.as_ref()
    }

    pub fn statistics_api(&self) -> &dyn crate::apis::StatisticsApi{
        self.statistics_api.as_ref()
    }

    pub fn systems_api(&self) -> &dyn crate::apis::SystemsApi{
        self.systems_api.as_ref()
    }

    pub fn tape_drives_api(&self) -> &dyn crate::apis::TapeDrivesApi{
        self.tape_drives_api.as_ref()
    }

    pub fn tape_libraries_api(&self) -> &dyn crate::apis::TapeLibrariesApi{
        self.tape_libraries_api.as_ref()
    }

    pub fn tapes_api(&self) -> &dyn crate::apis::TapesApi{
        self.tapes_api.as_ref()
    }

    pub fn task_callbacks_api(&self) -> &dyn crate::apis::TaskCallbacksApi{
        self.task_callbacks_api.as_ref()
    }

    pub fn task_destinations_api(&self) -> &dyn crate::apis::TaskDestinationsApi{
        self.task_destinations_api.as_ref()
    }

    pub fn task_executions_api(&self) -> &dyn crate::apis::TaskExecutionsApi{
        self.task_executions_api.as_ref()
    }

    pub fn task_metadata_api(&self) -> &dyn crate::apis::TaskMetadataApi{
        self.task_metadata_api.as_ref()
    }

    pub fn task_options_api(&self) -> &dyn crate::apis::TaskOptionsApi{
        self.task_options_api.as_ref()
    }

    pub fn task_schedules_api(&self) -> &dyn crate::apis::TaskSchedulesApi{
        self.task_schedules_api.as_ref()
    }

    pub fn task_sources_api(&self) -> &dyn crate::apis::TaskSourcesApi{
        self.task_sources_api.as_ref()
    }

    pub fn tasks_api(&self) -> &dyn crate::apis::TasksApi{
        self.tasks_api.as_ref()
    }

    pub fn users_api(&self) -> &dyn crate::apis::UsersApi{
        self.users_api.as_ref()
    }

}
