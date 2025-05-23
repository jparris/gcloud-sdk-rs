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
pub struct InstanceConsumptionData {
    #[serde(rename = "consumptionInfo", skip_serializing_if = "Option::is_none")]
    pub consumption_info: Option<Box<models::InstanceConsumptionInfo>>,
    /// Server-defined URL for the instance.
    #[serde(rename = "instance", skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
}

impl InstanceConsumptionData {
    pub fn new() -> InstanceConsumptionData {
        InstanceConsumptionData {
            consumption_info: None,
            instance: None,
        }
    }
}
