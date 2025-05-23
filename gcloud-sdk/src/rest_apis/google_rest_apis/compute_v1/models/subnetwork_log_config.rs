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

/// SubnetworkLogConfig : The available logging options for this subnetwork.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubnetworkLogConfig {
    /// Can only be specified if VPC flow logging for this subnetwork is enabled. Toggles the aggregation interval for collecting flow logs. Increasing the interval time will reduce the amount of generated flow logs for long lasting connections. Default is an interval of 5 seconds per connection.
    #[serde(
        rename = "aggregationInterval",
        skip_serializing_if = "Option::is_none"
    )]
    pub aggregation_interval: Option<AggregationInterval>,
    /// Whether to enable flow logging for this subnetwork. If this field is not explicitly set, it will not appear in get listings. If not set the default behavior is determined by the org policy, if there is no org policy specified, then it will default to disabled. Flow logging isn't supported if the subnet purpose field is set to REGIONAL_MANAGED_PROXY.
    #[serde(rename = "enable", skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    /// Can only be specified if VPC flow logs for this subnetwork is enabled. The filter expression is used to define which VPC flow logs should be exported to Cloud Logging.
    #[serde(rename = "filterExpr", skip_serializing_if = "Option::is_none")]
    pub filter_expr: Option<String>,
    /// Can only be specified if VPC flow logging for this subnetwork is enabled. The value of the field must be in [0, 1]. Set the sampling rate of VPC flow logs within the subnetwork where 1.0 means all collected logs are reported and 0.0 means no logs are reported. Default is 0.5 unless otherwise specified by the org policy, which means half of all collected logs are reported.
    #[serde(rename = "flowSampling", skip_serializing_if = "Option::is_none")]
    pub flow_sampling: Option<f32>,
    /// Can only be specified if VPC flow logs for this subnetwork is enabled. Configures whether all, none or a subset of metadata fields should be added to the reported VPC flow logs. Default is EXCLUDE_ALL_METADATA.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// Can only be specified if VPC flow logs for this subnetwork is enabled and \"metadata\" was set to CUSTOM_METADATA.
    #[serde(rename = "metadataFields", skip_serializing_if = "Option::is_none")]
    pub metadata_fields: Option<Vec<String>>,
}

impl SubnetworkLogConfig {
    /// The available logging options for this subnetwork.
    pub fn new() -> SubnetworkLogConfig {
        SubnetworkLogConfig {
            aggregation_interval: None,
            enable: None,
            filter_expr: None,
            flow_sampling: None,
            metadata: None,
            metadata_fields: None,
        }
    }
}
/// Can only be specified if VPC flow logging for this subnetwork is enabled. Toggles the aggregation interval for collecting flow logs. Increasing the interval time will reduce the amount of generated flow logs for long lasting connections. Default is an interval of 5 seconds per connection.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AggregationInterval {
    #[serde(rename = "INTERVAL_10_MIN")]
    Variant10Min,
    #[serde(rename = "INTERVAL_15_MIN")]
    Variant15Min,
    #[serde(rename = "INTERVAL_1_MIN")]
    Variant1Min,
    #[serde(rename = "INTERVAL_30_SEC")]
    Variant30Sec,
    #[serde(rename = "INTERVAL_5_MIN")]
    Variant5Min,
    #[serde(rename = "INTERVAL_5_SEC")]
    Variant5Sec,
}

impl Default for AggregationInterval {
    fn default() -> AggregationInterval {
        Self::Variant10Min
    }
}
/// Can only be specified if VPC flow logs for this subnetwork is enabled. Configures whether all, none or a subset of metadata fields should be added to the reported VPC flow logs. Default is EXCLUDE_ALL_METADATA.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Metadata {
    #[serde(rename = "CUSTOM_METADATA")]
    CustomMetadata,
    #[serde(rename = "EXCLUDE_ALL_METADATA")]
    ExcludeAllMetadata,
    #[serde(rename = "INCLUDE_ALL_METADATA")]
    IncludeAllMetadata,
}

impl Default for Metadata {
    fn default() -> Metadata {
        Self::CustomMetadata
    }
}
