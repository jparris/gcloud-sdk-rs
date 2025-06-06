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

/// Organization : The root node in the resource hierarchy to which a particular entity's (a company, for example) resources belong.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Organization {
    /// Output only. Timestamp when the Organization was created.
    #[serde(rename = "createTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// Output only. Timestamp when the Organization was requested for deletion.
    #[serde(rename = "deleteTime", skip_serializing_if = "Option::is_none")]
    pub delete_time: Option<String>,
    /// Immutable. The G Suite / Workspace customer id used in the Directory API.
    #[serde(
        rename = "directoryCustomerId",
        skip_serializing_if = "Option::is_none"
    )]
    pub directory_customer_id: Option<String>,
    /// Output only. A human-readable string that refers to the organization in the Google Cloud Console. This string is set by the server and cannot be changed. The string will be set to the primary domain (for example, \"google.com\") of the Google Workspace customer that owns the organization.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Output only. A checksum computed by the server based on the current value of the Organization resource. This may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
    #[serde(rename = "etag", skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    /// Output only. The resource name of the organization. This is the organization's relative path in the API. Its format is \"organizations/[organization_id]\". For example, \"organizations/1234\".
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Output only. The organization's current lifecycle state.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// Output only. Timestamp when the Organization was last modified.
    #[serde(rename = "updateTime", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

impl Organization {
    /// The root node in the resource hierarchy to which a particular entity's (a company, for example) resources belong.
    pub fn new() -> Organization {
        Organization {
            create_time: None,
            delete_time: None,
            directory_customer_id: None,
            display_name: None,
            etag: None,
            name: None,
            state: None,
            update_time: None,
        }
    }
}
/// Output only. The organization's current lifecycle state.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "STATE_UNSPECIFIED")]
    StateUnspecified,
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "DELETE_REQUESTED")]
    DeleteRequested,
}

impl Default for State {
    fn default() -> State {
        Self::StateUnspecified
    }
}
