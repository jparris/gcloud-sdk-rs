use serde::{Deserialize, Serialize}; /*
                                      * BigQuery API
                                      *
                                      * A data platform for customers to create, manage, share and query data.
                                      *
                                      * The version of the OpenAPI document: v2
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::bigquery_v2::models;

/// ClusterInfo : Information about a single cluster for clustering model.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterInfo {
    /// Centroid id.
    #[serde(rename = "centroidId", skip_serializing_if = "Option::is_none")]
    pub centroid_id: Option<String>,
    /// Cluster radius, the average distance from centroid to each point assigned to the cluster.
    #[serde(rename = "clusterRadius", skip_serializing_if = "Option::is_none")]
    pub cluster_radius: Option<f64>,
    /// Cluster size, the total number of points assigned to the cluster.
    #[serde(rename = "clusterSize", skip_serializing_if = "Option::is_none")]
    pub cluster_size: Option<String>,
}

impl ClusterInfo {
    /// Information about a single cluster for clustering model.
    pub fn new() -> ClusterInfo {
        ClusterInfo {
            centroid_id: None,
            cluster_radius: None,
            cluster_size: None,
        }
    }
}
