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

/// InitialStateConfig : Initial State for shielded instance, these are public keys which are safe to store in public
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InitialStateConfig {
    /// The Key Database (db).
    #[serde(rename = "dbs", skip_serializing_if = "Option::is_none")]
    pub dbs: Option<Vec<models::FileContentBuffer>>,
    /// The forbidden key database (dbx).
    #[serde(rename = "dbxs", skip_serializing_if = "Option::is_none")]
    pub dbxs: Option<Vec<models::FileContentBuffer>>,
    /// The Key Exchange Key (KEK).
    #[serde(rename = "keks", skip_serializing_if = "Option::is_none")]
    pub keks: Option<Vec<models::FileContentBuffer>>,
    #[serde(rename = "pk", skip_serializing_if = "Option::is_none")]
    pub pk: Option<Box<models::FileContentBuffer>>,
}

impl InitialStateConfig {
    /// Initial State for shielded instance, these are public keys which are safe to store in public
    pub fn new() -> InitialStateConfig {
        InitialStateConfig {
            dbs: None,
            dbxs: None,
            keks: None,
            pk: None,
        }
    }
}
