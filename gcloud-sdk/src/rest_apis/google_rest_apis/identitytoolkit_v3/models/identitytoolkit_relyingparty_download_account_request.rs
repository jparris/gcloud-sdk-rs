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

/// IdentitytoolkitRelyingpartyDownloadAccountRequest : Request to download user account in batch.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentitytoolkitRelyingpartyDownloadAccountRequest {
    /// GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration.
    #[serde(
        rename = "delegatedProjectNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub delegated_project_number: Option<String>,
    /// The max number of results to return in the response.
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// The token for the next page. This should be taken from the previous response.
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// Specify which project (field value is actually project id) to operate. Only used when provided credential.
    #[serde(rename = "targetProjectId", skip_serializing_if = "Option::is_none")]
    pub target_project_id: Option<String>,
}

impl IdentitytoolkitRelyingpartyDownloadAccountRequest {
    /// Request to download user account in batch.
    pub fn new() -> IdentitytoolkitRelyingpartyDownloadAccountRequest {
        IdentitytoolkitRelyingpartyDownloadAccountRequest {
            delegated_project_number: None,
            max_results: None,
            next_page_token: None,
            target_project_id: None,
        }
    }
}
