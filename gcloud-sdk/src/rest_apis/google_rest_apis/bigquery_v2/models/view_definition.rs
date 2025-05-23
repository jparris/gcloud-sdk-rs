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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ViewDefinition {
    /// [Required] A query that BigQuery executes when the view is referenced.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// True if the column names are explicitly specified. For example by using the 'CREATE VIEW v(c1, c2) AS ...' syntax. Can only be set using BigQuery's standard SQL: https://cloud.google.com/bigquery/sql-reference/
    #[serde(
        rename = "useExplicitColumnNames",
        skip_serializing_if = "Option::is_none"
    )]
    pub use_explicit_column_names: Option<bool>,
    /// Specifies whether to use BigQuery's legacy SQL for this view. The default value is true. If set to false, the view will use BigQuery's standard SQL: https://cloud.google.com/bigquery/sql-reference/ Queries and views that reference this view must use the same flag value.
    #[serde(rename = "useLegacySql", skip_serializing_if = "Option::is_none")]
    pub use_legacy_sql: Option<bool>,
    /// Describes user-defined function resources used in the query.
    #[serde(
        rename = "userDefinedFunctionResources",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_defined_function_resources: Option<Vec<models::UserDefinedFunctionResource>>,
}

impl ViewDefinition {
    pub fn new() -> ViewDefinition {
        ViewDefinition {
            query: None,
            use_explicit_column_names: None,
            use_legacy_sql: None,
            user_defined_function_resources: None,
        }
    }
}
