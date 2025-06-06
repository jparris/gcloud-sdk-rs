use serde::{Deserialize, Serialize}; /*
                                      * Cloud SQL Admin API
                                      *
                                      * API for Cloud SQL database instance management
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::sqladmin_v1::models;

/// ExportContextSqlExportOptions : Options for exporting data as SQL statements.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportContextSqlExportOptions {
    #[serde(rename = "mysqlExportOptions", skip_serializing_if = "Option::is_none")]
    pub mysql_export_options: Option<Box<models::ExportContextSqlExportOptionsMysqlExportOptions>>,
    /// Export only schemas.
    #[serde(rename = "schemaOnly", skip_serializing_if = "Option::is_none")]
    pub schema_only: Option<bool>,
    /// Tables to export, or that were exported, from the specified database. If you specify tables, specify one and only one database. For PostgreSQL instances, you can specify only one table.
    #[serde(rename = "tables", skip_serializing_if = "Option::is_none")]
    pub tables: Option<Vec<String>>,
}

impl ExportContextSqlExportOptions {
    /// Options for exporting data as SQL statements.
    pub fn new() -> ExportContextSqlExportOptions {
        ExportContextSqlExportOptions {
            mysql_export_options: None,
            schema_only: None,
            tables: None,
        }
    }
}
