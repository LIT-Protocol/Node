use crate::guest::cloud_init::meta_data::CloudInitMetaData;
use crate::guest::cloud_init::network_config::CloudInitNetworkConfig;
use crate::guest::cloud_init::user_data::CloudInitUserData;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct CloudInitContext {
    path: PathBuf,
    meta_data: CloudInitMetaData,
    user_data: CloudInitUserData,
    network_config: CloudInitNetworkConfig,
}

impl CloudInitContext {
    pub fn new(
        path: PathBuf, meta_data: CloudInitMetaData, user_data: CloudInitUserData,
        network_config: CloudInitNetworkConfig,
    ) -> Self {
        Self { path, meta_data, user_data, network_config }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn meta_data(&self) -> &CloudInitMetaData {
        &self.meta_data
    }

    pub fn user_data(&self) -> &CloudInitUserData {
        &self.user_data
    }

    pub fn network_config(&self) -> &CloudInitNetworkConfig {
        &self.network_config
    }
}
