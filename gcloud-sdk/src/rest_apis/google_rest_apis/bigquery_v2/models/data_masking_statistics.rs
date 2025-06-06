use serde::{Deserialize, Serialize}; /*
                                      * BigQuery API
                                      *
                                      * A data platform for customers to create, manage, share and query data.
                                      *
                                      * The version of the OpenAPI document: v2
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::bigquery_v2::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataMaskingStatistics {
    /// [Output-only] [Preview] Whether any accessed data was protected by data masking. The actual evaluation is done by accessStats.masked_field_count > 0. Since this is only used for the discovery_doc generation purpose, as long as the type (boolean) matches, client library can leverage this. The actual evaluation of the variable is done else-where.
    #[serde(rename = "dataMaskingApplied", skip_serializing_if = "Option::is_none")]
    pub data_masking_applied: Option<bool>,
}

impl DataMaskingStatistics {
    pub fn new() -> DataMaskingStatistics {
        DataMaskingStatistics {
            data_masking_applied: None,
        }
    }
}
