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

/// ListLiensResponse : The response message for Liens.ListLiens.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListLiensResponse {
    /// A list of Liens.
    #[serde(rename = "liens", skip_serializing_if = "Option::is_none")]
    pub liens: Option<Vec<models::Lien>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl ListLiensResponse {
    /// The response message for Liens.ListLiens.
    pub fn new() -> ListLiensResponse {
        ListLiensResponse {
            liens: None,
            next_page_token: None,
        }
    }
}
