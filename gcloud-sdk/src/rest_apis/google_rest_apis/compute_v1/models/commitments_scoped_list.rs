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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitmentsScopedList {
    /// [Output Only] A list of commitments contained in this scope.
    #[serde(rename = "commitments", skip_serializing_if = "Option::is_none")]
    pub commitments: Option<Vec<models::Commitment>>,
    #[serde(rename = "warning", skip_serializing_if = "Option::is_none")]
    pub warning: Option<Box<models::CommitmentsScopedListWarning>>,
}

impl CommitmentsScopedList {
    pub fn new() -> CommitmentsScopedList {
        CommitmentsScopedList {
            commitments: None,
            warning: None,
        }
    }
}
