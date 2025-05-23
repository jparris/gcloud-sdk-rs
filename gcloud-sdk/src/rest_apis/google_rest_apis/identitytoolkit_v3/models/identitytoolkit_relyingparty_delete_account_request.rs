use serde::{Deserialize, Serialize}; /*
                                      * Google Identity Toolkit API
                                      *
                                      * Help the third party sites to implement federated login.
                                      *
                                      * The version of the OpenAPI document: v3
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::identitytoolkit_v3::models;

/// IdentitytoolkitRelyingpartyDeleteAccountRequest : Request to delete account.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentitytoolkitRelyingpartyDeleteAccountRequest {
    /// GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration.
    #[serde(
        rename = "delegatedProjectNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub delegated_project_number: Option<String>,
    /// The GITKit token or STS id token of the authenticated user.
    #[serde(rename = "idToken", skip_serializing_if = "Option::is_none")]
    pub id_token: Option<String>,
    /// The local ID of the user.
    #[serde(rename = "localId", skip_serializing_if = "Option::is_none")]
    pub local_id: Option<String>,
}

impl IdentitytoolkitRelyingpartyDeleteAccountRequest {
    /// Request to delete account.
    pub fn new() -> IdentitytoolkitRelyingpartyDeleteAccountRequest {
        IdentitytoolkitRelyingpartyDeleteAccountRequest {
            delegated_project_number: None,
            id_token: None,
            local_id: None,
        }
    }
}
