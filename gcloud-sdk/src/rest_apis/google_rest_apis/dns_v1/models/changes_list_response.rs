use serde::{Deserialize, Serialize}; /*
                                      * Cloud DNS API
                                      *
                                      * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::dns_v1::models;

/// ChangesListResponse : The response to a request to enumerate Changes to a ResourceRecordSets collection.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangesListResponse {
    /// The requested changes.
    #[serde(rename = "changes", skip_serializing_if = "Option::is_none")]
    pub changes: Option<Vec<models::Change>>,
    #[serde(rename = "header", skip_serializing_if = "Option::is_none")]
    pub header: Option<Box<models::ResponseHeader>>,
    /// Type of resource.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The presence of this field indicates that there exist more results following your last page of results in pagination order. To fetch them, make another list request using this value as your pagination token. This lets you retrieve the complete contents of even very large collections one page at a time. However, if the contents of the collection change between the first and last paginated list request, the set of all elements returned are an inconsistent view of the collection. You cannot retrieve a \"snapshot\" of collections larger than the maximum page size.
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl ChangesListResponse {
    /// The response to a request to enumerate Changes to a ResourceRecordSets collection.
    pub fn new() -> ChangesListResponse {
        ChangesListResponse {
            changes: None,
            header: None,
            kind: None,
            next_page_token: None,
        }
    }
}
