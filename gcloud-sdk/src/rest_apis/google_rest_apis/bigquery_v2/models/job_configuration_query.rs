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
pub struct JobConfigurationQuery {
    /// [Optional] If true and query uses legacy SQL dialect, allows the query to produce arbitrarily large result tables at a slight cost in performance. Requires destinationTable to be set. For standard SQL queries, this flag is ignored and large results are always allowed. However, you must still set destinationTable when result size exceeds the allowed maximum response size.
    #[serde(rename = "allowLargeResults", skip_serializing_if = "Option::is_none")]
    pub allow_large_results: Option<bool>,
    #[serde(rename = "clustering", skip_serializing_if = "Option::is_none")]
    pub clustering: Option<Box<models::Clustering>>,
    /// Connection properties.
    #[serde(
        rename = "connectionProperties",
        skip_serializing_if = "Option::is_none"
    )]
    pub connection_properties: Option<Vec<models::ConnectionProperty>>,
    /// [Optional] Specifies whether the job is allowed to create new tables. The following values are supported: CREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table. CREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result. The default value is CREATE_IF_NEEDED. Creation, truncation and append actions occur as one atomic update upon job completion.
    #[serde(rename = "createDisposition", skip_serializing_if = "Option::is_none")]
    pub create_disposition: Option<String>,
    /// If true, creates a new session, where session id will be a server generated random id. If false, runs query with an existing session_id passed in ConnectionProperty, otherwise runs query in non-session mode.
    #[serde(rename = "createSession", skip_serializing_if = "Option::is_none")]
    pub create_session: Option<bool>,
    #[serde(rename = "defaultDataset", skip_serializing_if = "Option::is_none")]
    pub default_dataset: Option<Box<models::DatasetReference>>,
    #[serde(
        rename = "destinationEncryptionConfiguration",
        skip_serializing_if = "Option::is_none"
    )]
    pub destination_encryption_configuration: Option<Box<models::EncryptionConfiguration>>,
    #[serde(rename = "destinationTable", skip_serializing_if = "Option::is_none")]
    pub destination_table: Option<Box<models::TableReference>>,
    /// [Optional] If true and query uses legacy SQL dialect, flattens all nested and repeated fields in the query results. allowLargeResults must be true if this is set to false. For standard SQL queries, this flag is ignored and results are never flattened.
    #[serde(rename = "flattenResults", skip_serializing_if = "Option::is_none")]
    pub flatten_results: Option<bool>,
    /// [Optional] Limits the billing tier for this job. Queries that have resource usage beyond this tier will fail (without incurring a charge). If unspecified, this will be set to your project default.
    #[serde(rename = "maximumBillingTier", skip_serializing_if = "Option::is_none")]
    pub maximum_billing_tier: Option<i32>,
    /// [Optional] Limits the bytes billed for this job. Queries that will have bytes billed beyond this limit will fail (without incurring a charge). If unspecified, this will be set to your project default.
    #[serde(rename = "maximumBytesBilled", skip_serializing_if = "Option::is_none")]
    pub maximum_bytes_billed: Option<String>,
    /// Standard SQL only. Set to POSITIONAL to use positional (?) query parameters or to NAMED to use named (@myparam) query parameters in this query.
    #[serde(rename = "parameterMode", skip_serializing_if = "Option::is_none")]
    pub parameter_mode: Option<String>,
    /// [Deprecated] This property is deprecated.
    #[serde(rename = "preserveNulls", skip_serializing_if = "Option::is_none")]
    pub preserve_nulls: Option<bool>,
    /// [Optional] Specifies a priority for the query. Possible values include INTERACTIVE and BATCH. The default value is INTERACTIVE.
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    /// [Required] SQL query text to execute. The useLegacySql field can be used to indicate whether the query uses legacy SQL or standard SQL.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Query parameters for standard SQL queries.
    #[serde(rename = "queryParameters", skip_serializing_if = "Option::is_none")]
    pub query_parameters: Option<Vec<models::QueryParameter>>,
    #[serde(rename = "rangePartitioning", skip_serializing_if = "Option::is_none")]
    pub range_partitioning: Option<Box<models::RangePartitioning>>,
    /// Allows the schema of the destination table to be updated as a side effect of the query job. Schema update options are supported in two cases: when writeDisposition is WRITE_APPEND; when writeDisposition is WRITE_TRUNCATE and the destination table is a partition of a table, specified by partition decorators. For normal tables, WRITE_TRUNCATE will always overwrite the schema. One or more of the following values are specified: ALLOW_FIELD_ADDITION: allow adding a nullable field to the schema. ALLOW_FIELD_RELAXATION: allow relaxing a required field in the original schema to nullable.
    #[serde(
        rename = "schemaUpdateOptions",
        skip_serializing_if = "Option::is_none"
    )]
    pub schema_update_options: Option<Vec<String>>,
    /// [Optional] If querying an external data source outside of BigQuery, describes the data format, location and other properties of the data source. By defining these properties, the data source can then be queried as if it were a standard BigQuery table.
    #[serde(rename = "tableDefinitions", skip_serializing_if = "Option::is_none")]
    pub table_definitions:
        Option<std::collections::HashMap<String, models::ExternalDataConfiguration>>,
    #[serde(rename = "timePartitioning", skip_serializing_if = "Option::is_none")]
    pub time_partitioning: Option<Box<models::TimePartitioning>>,
    /// Specifies whether to use BigQuery's legacy SQL dialect for this query. The default value is true. If set to false, the query will use BigQuery's standard SQL: https://cloud.google.com/bigquery/sql-reference/ When useLegacySql is set to false, the value of flattenResults is ignored; query will be run as if flattenResults is false.
    #[serde(rename = "useLegacySql", skip_serializing_if = "Option::is_none")]
    pub use_legacy_sql: Option<bool>,
    /// [Optional] Whether to look for the result in the query cache. The query cache is a best-effort cache that will be flushed whenever tables in the query are modified. Moreover, the query cache is only available when a query does not have a destination table specified. The default value is true.
    #[serde(rename = "useQueryCache", skip_serializing_if = "Option::is_none")]
    pub use_query_cache: Option<bool>,
    /// Describes user-defined function resources used in the query.
    #[serde(
        rename = "userDefinedFunctionResources",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_defined_function_resources: Option<Vec<models::UserDefinedFunctionResource>>,
    /// [Optional] Specifies the action that occurs if the destination table already exists. The following values are supported: WRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data and uses the schema from the query result. WRITE_APPEND: If the table already exists, BigQuery appends the data to the table. WRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result. The default value is WRITE_EMPTY. Each action is atomic and only occurs if BigQuery is able to complete the job successfully. Creation, truncation and append actions occur as one atomic update upon job completion.
    #[serde(rename = "writeDisposition", skip_serializing_if = "Option::is_none")]
    pub write_disposition: Option<String>,
}

impl JobConfigurationQuery {
    pub fn new() -> JobConfigurationQuery {
        JobConfigurationQuery {
            allow_large_results: None,
            clustering: None,
            connection_properties: None,
            create_disposition: None,
            create_session: None,
            default_dataset: None,
            destination_encryption_configuration: None,
            destination_table: None,
            flatten_results: None,
            maximum_billing_tier: None,
            maximum_bytes_billed: None,
            parameter_mode: None,
            preserve_nulls: None,
            priority: None,
            query: None,
            query_parameters: None,
            range_partitioning: None,
            schema_update_options: None,
            table_definitions: None,
            time_partitioning: None,
            use_legacy_sql: None,
            use_query_cache: None,
            user_defined_function_resources: None,
            write_disposition: None,
        }
    }
}
