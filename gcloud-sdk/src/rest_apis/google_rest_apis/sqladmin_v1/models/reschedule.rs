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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Reschedule {
    /// Required. The type of the reschedule.
    #[serde(rename = "rescheduleType", skip_serializing_if = "Option::is_none")]
    pub reschedule_type: Option<RescheduleType>,
    /// Optional. Timestamp when the maintenance shall be rescheduled to if reschedule_type=SPECIFIC_TIME, in [RFC 3339](https://tools.ietf.org/html/rfc3339) format, for example `2012-11-15T16:19:00.094Z`.
    #[serde(rename = "scheduleTime", skip_serializing_if = "Option::is_none")]
    pub schedule_time: Option<String>,
}

impl Reschedule {
    pub fn new() -> Reschedule {
        Reschedule {
            reschedule_type: None,
            schedule_time: None,
        }
    }
}
/// Required. The type of the reschedule.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RescheduleType {
    #[serde(rename = "RESCHEDULE_TYPE_UNSPECIFIED")]
    RescheduleTypeUnspecified,
    #[serde(rename = "IMMEDIATE")]
    Immediate,
    #[serde(rename = "NEXT_AVAILABLE_WINDOW")]
    NextAvailableWindow,
    #[serde(rename = "SPECIFIC_TIME")]
    SpecificTime,
}

impl Default for RescheduleType {
    fn default() -> RescheduleType {
        Self::RescheduleTypeUnspecified
    }
}
