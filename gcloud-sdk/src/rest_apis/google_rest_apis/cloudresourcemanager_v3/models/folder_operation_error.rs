use serde::{Deserialize, Serialize}; /*
                                      * Cloud Resource Manager API
                                      *
                                      * Creates, reads, and updates metadata for Google Cloud Platform resource containers.
                                      *
                                      * The version of the OpenAPI document: v3
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::cloudresourcemanager_v3::models;

/// FolderOperationError : A classification of the Folder Operation error.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FolderOperationError {
    /// The type of operation error experienced.
    #[serde(rename = "errorMessageId", skip_serializing_if = "Option::is_none")]
    pub error_message_id: Option<ErrorMessageId>,
}

impl FolderOperationError {
    /// A classification of the Folder Operation error.
    pub fn new() -> FolderOperationError {
        FolderOperationError {
            error_message_id: None,
        }
    }
}
/// The type of operation error experienced.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ErrorMessageId {
    #[serde(rename = "ERROR_TYPE_UNSPECIFIED")]
    ErrorTypeUnspecified,
    #[serde(rename = "ACTIVE_FOLDER_HEIGHT_VIOLATION")]
    ActiveFolderHeightViolation,
    #[serde(rename = "MAX_CHILD_FOLDERS_VIOLATION")]
    MaxChildFoldersViolation,
    #[serde(rename = "FOLDER_NAME_UNIQUENESS_VIOLATION")]
    FolderNameUniquenessViolation,
    #[serde(rename = "RESOURCE_DELETED_VIOLATION")]
    ResourceDeletedViolation,
    #[serde(rename = "PARENT_DELETED_VIOLATION")]
    ParentDeletedViolation,
    #[serde(rename = "CYCLE_INTRODUCED_VIOLATION")]
    CycleIntroducedViolation,
    #[serde(rename = "FOLDER_BEING_MOVED_VIOLATION")]
    FolderBeingMovedViolation,
    #[serde(rename = "FOLDER_TO_DELETE_NON_EMPTY_VIOLATION")]
    FolderToDeleteNonEmptyViolation,
    #[serde(rename = "DELETED_FOLDER_HEIGHT_VIOLATION")]
    DeletedFolderHeightViolation,
}

impl Default for ErrorMessageId {
    fn default() -> ErrorMessageId {
        Self::ErrorTypeUnspecified
    }
}
