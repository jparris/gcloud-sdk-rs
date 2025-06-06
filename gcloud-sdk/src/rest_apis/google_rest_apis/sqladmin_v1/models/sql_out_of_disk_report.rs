use serde::{Deserialize, Serialize}; /*
                                      * Cloud SQL Admin API
                                      *
                                      * API for Cloud SQL database instance management
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::sqladmin_v1::models;

/// SqlOutOfDiskReport : This message wraps up the information written by out-of-disk detection job.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlOutOfDiskReport {
    /// The minimum recommended increase size in GigaBytes This field is consumed by the frontend * Writers: * the proactive database wellness job for OOD. * Readers:
    #[serde(
        rename = "sqlMinRecommendedIncreaseSizeGb",
        skip_serializing_if = "Option::is_none"
    )]
    pub sql_min_recommended_increase_size_gb: Option<i32>,
    /// This field represents the state generated by the proactive database wellness job for OutOfDisk issues. * Writers: * the proactive database wellness job for OOD. * Readers: * the proactive database wellness job
    #[serde(rename = "sqlOutOfDiskState", skip_serializing_if = "Option::is_none")]
    pub sql_out_of_disk_state: Option<SqlOutOfDiskState>,
}

impl SqlOutOfDiskReport {
    /// This message wraps up the information written by out-of-disk detection job.
    pub fn new() -> SqlOutOfDiskReport {
        SqlOutOfDiskReport {
            sql_min_recommended_increase_size_gb: None,
            sql_out_of_disk_state: None,
        }
    }
}
/// This field represents the state generated by the proactive database wellness job for OutOfDisk issues. * Writers: * the proactive database wellness job for OOD. * Readers: * the proactive database wellness job
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SqlOutOfDiskState {
    #[serde(rename = "SQL_OUT_OF_DISK_STATE_UNSPECIFIED")]
    SqlOutOfDiskStateUnspecified,
    #[serde(rename = "NORMAL")]
    Normal,
    #[serde(rename = "SOFT_SHUTDOWN")]
    SoftShutdown,
}

impl Default for SqlOutOfDiskState {
    fn default() -> SqlOutOfDiskState {
        Self::SqlOutOfDiskStateUnspecified
    }
}
