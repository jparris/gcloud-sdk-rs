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
pub struct ConnectionProperty {
    /// [Required] Name of the connection property to set.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// [Required] Value of the connection property.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ConnectionProperty {
    pub fn new() -> ConnectionProperty {
        ConnectionProperty {
            key: None,
            value: None,
        }
    }
}
