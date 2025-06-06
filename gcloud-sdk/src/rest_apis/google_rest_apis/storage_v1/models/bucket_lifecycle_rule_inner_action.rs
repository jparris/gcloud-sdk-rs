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

/// BucketLifecycleRuleInnerAction : The action to take.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BucketLifecycleRuleInnerAction {
    /// Target storage class. Required iff the type of the action is SetStorageClass.
    #[serde(rename = "storageClass", skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
    /// Type of the action. Currently, only Delete, SetStorageClass, and AbortIncompleteMultipartUpload are supported.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl BucketLifecycleRuleInnerAction {
    /// The action to take.
    pub fn new() -> BucketLifecycleRuleInnerAction {
        BucketLifecycleRuleInnerAction {
            storage_class: None,
            r#type: None,
        }
    }
}
