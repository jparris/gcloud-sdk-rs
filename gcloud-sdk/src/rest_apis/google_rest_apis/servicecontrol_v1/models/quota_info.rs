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

/// QuotaInfo : Contains the quota information for a quota check response.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaInfo {
    /// Quota Metrics that have exceeded quota limits. For QuotaGroup-based quota, this is QuotaGroup.name For QuotaLimit-based quota, this is QuotaLimit.name See: google.api.Quota Deprecated: Use quota_metrics to get per quota group limit exceeded status.
    #[serde(rename = "limitExceeded", skip_serializing_if = "Option::is_none")]
    pub limit_exceeded: Option<Vec<String>>,
    /// Map of quota group name to the actual number of tokens consumed. If the quota check was not successful, then this will not be populated due to no quota consumption. We are not merging this field with 'quota_metrics' field because of the complexity of scaling in Chemist client code base. For simplicity, we will keep this field for Castor (that scales quota usage) and 'quota_metrics' for SuperQuota (that doesn't scale quota usage).
    #[serde(rename = "quotaConsumed", skip_serializing_if = "Option::is_none")]
    pub quota_consumed: Option<std::collections::HashMap<String, i32>>,
    /// Quota metrics to indicate the usage. Depending on the check request, one or more of the following metrics will be included: 1. For rate quota, per quota group or per quota metric incremental usage will be specified using the following delta metric: \"serviceruntime.googleapis.com/api/consumer/quota_used_count\" 2. For allocation quota, per quota metric total usage will be specified using the following gauge metric: \"serviceruntime.googleapis.com/allocation/consumer/quota_used_count\" 3. For both rate quota and allocation quota, the quota limit reached condition will be specified using the following boolean metric: \"serviceruntime.googleapis.com/quota/exceeded\"
    #[serde(rename = "quotaMetrics", skip_serializing_if = "Option::is_none")]
    pub quota_metrics: Option<Vec<models::MetricValueSet>>,
}

impl QuotaInfo {
    /// Contains the quota information for a quota check response.
    pub fn new() -> QuotaInfo {
        QuotaInfo {
            limit_exceeded: None,
            quota_consumed: None,
            quota_metrics: None,
        }
    }
}
