use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::compute_v1::models;

/// LogConfigCloudAuditOptions : This is deprecated and has no effect. Do not use.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogConfigCloudAuditOptions {
    #[serde(
        rename = "authorizationLoggingOptions",
        skip_serializing_if = "Option::is_none"
    )]
    pub authorization_logging_options: Option<Box<models::AuthorizationLoggingOptions>>,
    /// This is deprecated and has no effect. Do not use.
    #[serde(rename = "logName", skip_serializing_if = "Option::is_none")]
    pub log_name: Option<LogName>,
}

impl LogConfigCloudAuditOptions {
    /// This is deprecated and has no effect. Do not use.
    pub fn new() -> LogConfigCloudAuditOptions {
        LogConfigCloudAuditOptions {
            authorization_logging_options: None,
            log_name: None,
        }
    }
}
/// This is deprecated and has no effect. Do not use.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogName {
    #[serde(rename = "ADMIN_ACTIVITY")]
    AdminActivity,
    #[serde(rename = "DATA_ACCESS")]
    DataAccess,
    #[serde(rename = "UNSPECIFIED_LOG_NAME")]
    UnspecifiedLogName,
}

impl Default for LogName {
    fn default() -> LogName {
        Self::AdminActivity
    }
}
