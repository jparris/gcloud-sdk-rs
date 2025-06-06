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
pub struct RoutersScopedList {
    /// A list of routers contained in this scope.
    #[serde(rename = "routers", skip_serializing_if = "Option::is_none")]
    pub routers: Option<Vec<models::Router>>,
    #[serde(rename = "warning", skip_serializing_if = "Option::is_none")]
    pub warning: Option<Box<models::RoutersScopedListWarning>>,
}

impl RoutersScopedList {
    pub fn new() -> RoutersScopedList {
        RoutersScopedList {
            routers: None,
            warning: None,
        }
    }
}
