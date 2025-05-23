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

/// NotificationEndpointGrpcSettings : Represents a gRPC setting that describes one gRPC notification endpoint and the retry duration attempting to send notification to this endpoint.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationEndpointGrpcSettings {
    /// Optional. If specified, this field is used to set the authority header by the sender of notifications. See https://tools.ietf.org/html/rfc7540#section-8.1.2.3
    #[serde(rename = "authority", skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
    /// Endpoint to which gRPC notifications are sent. This must be a valid gRPCLB DNS name.
    #[serde(rename = "endpoint", skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// Optional. If specified, this field is used to populate the \"name\" field in gRPC requests.
    #[serde(rename = "payloadName", skip_serializing_if = "Option::is_none")]
    pub payload_name: Option<String>,
    #[serde(rename = "resendInterval", skip_serializing_if = "Option::is_none")]
    pub resend_interval: Option<Box<models::Duration>>,
    /// How much time (in seconds) is spent attempting notification retries until a successful response is received. Default is 30s. Limit is 20m (1200s). Must be a positive number.
    #[serde(rename = "retryDurationSec", skip_serializing_if = "Option::is_none")]
    pub retry_duration_sec: Option<i32>,
}

impl NotificationEndpointGrpcSettings {
    /// Represents a gRPC setting that describes one gRPC notification endpoint and the retry duration attempting to send notification to this endpoint.
    pub fn new() -> NotificationEndpointGrpcSettings {
        NotificationEndpointGrpcSettings {
            authority: None,
            endpoint: None,
            payload_name: None,
            resend_interval: None,
            retry_duration_sec: None,
        }
    }
}
