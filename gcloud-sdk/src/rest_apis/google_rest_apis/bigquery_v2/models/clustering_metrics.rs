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

/// ClusteringMetrics : Evaluation metrics for clustering models.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusteringMetrics {
    /// Information for all clusters.
    #[serde(rename = "clusters", skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<models::Cluster>>,
    /// Davies-Bouldin index.
    #[serde(rename = "daviesBouldinIndex", skip_serializing_if = "Option::is_none")]
    pub davies_bouldin_index: Option<f64>,
    /// Mean of squared distances between each sample to its cluster centroid.
    #[serde(
        rename = "meanSquaredDistance",
        skip_serializing_if = "Option::is_none"
    )]
    pub mean_squared_distance: Option<f64>,
}

impl ClusteringMetrics {
    /// Evaluation metrics for clustering models.
    pub fn new() -> ClusteringMetrics {
        ClusteringMetrics {
            clusters: None,
            davies_bouldin_index: None,
            mean_squared_distance: None,
        }
    }
}
