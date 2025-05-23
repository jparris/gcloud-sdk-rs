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

/// RegressionMetrics : Evaluation metrics for regression and explicit feedback type matrix factorization models.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegressionMetrics {
    /// Mean absolute error.
    #[serde(rename = "meanAbsoluteError", skip_serializing_if = "Option::is_none")]
    pub mean_absolute_error: Option<f64>,
    /// Mean squared error.
    #[serde(rename = "meanSquaredError", skip_serializing_if = "Option::is_none")]
    pub mean_squared_error: Option<f64>,
    /// Mean squared log error.
    #[serde(
        rename = "meanSquaredLogError",
        skip_serializing_if = "Option::is_none"
    )]
    pub mean_squared_log_error: Option<f64>,
    /// Median absolute error.
    #[serde(
        rename = "medianAbsoluteError",
        skip_serializing_if = "Option::is_none"
    )]
    pub median_absolute_error: Option<f64>,
    /// R^2 score. This corresponds to r2_score in ML.EVALUATE.
    #[serde(rename = "rSquared", skip_serializing_if = "Option::is_none")]
    pub r_squared: Option<f64>,
}

impl RegressionMetrics {
    /// Evaluation metrics for regression and explicit feedback type matrix factorization models.
    pub fn new() -> RegressionMetrics {
        RegressionMetrics {
            mean_absolute_error: None,
            mean_squared_error: None,
            mean_squared_log_error: None,
            median_absolute_error: None,
            r_squared: None,
        }
    }
}
