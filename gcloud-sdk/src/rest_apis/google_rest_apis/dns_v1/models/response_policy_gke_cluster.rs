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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponsePolicyGkeCluster {
    /// The resource name of the cluster to bind this response policy to. This should be specified in the format like: projects/_*_/locations/_*_/clusters/_*. This is referenced from GKE projects.locations.clusters.get API: https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters/get
    #[serde(rename = "gkeClusterName", skip_serializing_if = "Option::is_none")]
    pub gke_cluster_name: Option<String>,
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

impl ResponsePolicyGkeCluster {
    pub fn new() -> ResponsePolicyGkeCluster {
        ResponsePolicyGkeCluster {
            gke_cluster_name: None,
            kind: None,
        }
    }
}
