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

/// BulkRestoreObjectsRequest : A bulk restore objects request.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkRestoreObjectsRequest {
    /// If false (default), the restore will not overwrite live objects with the same name at the destination. This means some deleted objects may be skipped. If true, live objects will be overwritten resulting in a noncurrent object (if versioning is enabled). If versioning is not enabled, overwriting the object will result in a soft-deleted object. In either case, if a noncurrent object already exists with the same name, a live version can be written without issue.
    #[serde(rename = "allowOverwrite", skip_serializing_if = "Option::is_none")]
    pub allow_overwrite: Option<bool>,
    /// If true, copies the source object's ACL; otherwise, uses the bucket's default object ACL. The default is false.
    #[serde(rename = "copySourceAcl", skip_serializing_if = "Option::is_none")]
    pub copy_source_acl: Option<bool>,
    /// Restores only the objects matching any of the specified glob(s). If this parameter is not specified, all objects will be restored within the specified time range.
    #[serde(rename = "matchGlobs", skip_serializing_if = "Option::is_none")]
    pub match_globs: Option<Vec<String>>,
    /// Restores only the objects that were soft-deleted after this time.
    #[serde(
        rename = "softDeletedAfterTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub soft_deleted_after_time: Option<String>,
    /// Restores only the objects that were soft-deleted before this time.
    #[serde(
        rename = "softDeletedBeforeTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub soft_deleted_before_time: Option<String>,
}

impl BulkRestoreObjectsRequest {
    /// A bulk restore objects request.
    pub fn new() -> BulkRestoreObjectsRequest {
        BulkRestoreObjectsRequest {
            allow_overwrite: None,
            copy_source_acl: None,
            match_globs: None,
            soft_deleted_after_time: None,
            soft_deleted_before_time: None,
        }
    }
}
