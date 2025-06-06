use serde::{Deserialize, Serialize}; /*
                                      * Service Control API
                                      *
                                      * Provides admission control and telemetry reporting for services integrated with Service Infrastructure.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::servicecontrol_v1::models;

/// ReportRequest : Request message for the Report method.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportRequest {
    /// Operations to be reported. Typically the service should report one operation per request. Putting multiple operations into a single request is allowed, but should be used only when multiple operations are natually available at the time of the report. There is no limit on the number of operations in the same ReportRequest, however the ReportRequest size should be no larger than 1MB. See ReportResponse.report_errors for partial failure behavior.
    #[serde(rename = "operations", skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<models::Operation>>,
    /// Specifies which version of service config should be used to process the request. If unspecified or no matching version can be found, the latest one will be used.
    #[serde(rename = "serviceConfigId", skip_serializing_if = "Option::is_none")]
    pub service_config_id: Option<String>,
}

impl ReportRequest {
    /// Request message for the Report method.
    pub fn new() -> ReportRequest {
        ReportRequest {
            operations: None,
            service_config_id: None,
        }
    }
}
