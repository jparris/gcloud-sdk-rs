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

/// UploadAccountResponse : Respone of uploading accounts in batch.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UploadAccountResponse {
    /// The error encountered while processing the account info.
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Vec<models::UploadAccountResponseErrorInner>>,
    /// The fixed string \"identitytoolkit#UploadAccountResponse\".
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

impl UploadAccountResponse {
    /// Respone of uploading accounts in batch.
    pub fn new() -> UploadAccountResponse {
        UploadAccountResponse {
            error: None,
            kind: None,
        }
    }
}
