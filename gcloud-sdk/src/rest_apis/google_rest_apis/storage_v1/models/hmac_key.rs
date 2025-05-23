use serde::{Deserialize, Serialize}; /*
                                      * Cloud Storage JSON API
                                      *
                                      * Stores and retrieves potentially large, immutable data objects.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::storage_v1::models;

/// HmacKey : JSON template to produce a JSON-style HMAC Key resource for Create responses.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HmacKey {
    /// The kind of item this is. For HMAC keys, this is always storage#hmacKey.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<models::HmacKeyMetadata>>,
    /// HMAC secret key material.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

impl HmacKey {
    /// JSON template to produce a JSON-style HMAC Key resource for Create responses.
    pub fn new() -> HmacKey {
        HmacKey {
            kind: None,
            metadata: None,
            secret: None,
        }
    }
}
