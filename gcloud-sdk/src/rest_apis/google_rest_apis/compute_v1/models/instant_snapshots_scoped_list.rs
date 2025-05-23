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
pub struct InstantSnapshotsScopedList {
    /// [Output Only] A list of instantSnapshots contained in this scope.
    #[serde(rename = "instantSnapshots", skip_serializing_if = "Option::is_none")]
    pub instant_snapshots: Option<Vec<models::InstantSnapshot>>,
    #[serde(rename = "warning", skip_serializing_if = "Option::is_none")]
    pub warning: Option<Box<models::InstantSnapshotsScopedListWarning>>,
}

impl InstantSnapshotsScopedList {
    pub fn new() -> InstantSnapshotsScopedList {
        InstantSnapshotsScopedList {
            instant_snapshots: None,
            warning: None,
        }
    }
}
