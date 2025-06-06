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

/// InstancesRestoreBackupRequest : Database instance restore backup request.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstancesRestoreBackupRequest {
    #[serde(
        rename = "restoreBackupContext",
        skip_serializing_if = "Option::is_none"
    )]
    pub restore_backup_context: Option<Box<models::RestoreBackupContext>>,
}

impl InstancesRestoreBackupRequest {
    /// Database instance restore backup request.
    pub fn new() -> InstancesRestoreBackupRequest {
        InstancesRestoreBackupRequest {
            restore_backup_context: None,
        }
    }
}
