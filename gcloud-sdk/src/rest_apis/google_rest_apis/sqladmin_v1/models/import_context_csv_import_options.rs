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

/// ImportContextCsvImportOptions : Options for importing data as CSV.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportContextCsvImportOptions {
    /// The columns to which CSV data is imported. If not specified, all columns of the database table are loaded with CSV data.
    #[serde(rename = "columns", skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<String>>,
    /// Specifies the character that should appear before a data character that needs to be escaped.
    #[serde(rename = "escapeCharacter", skip_serializing_if = "Option::is_none")]
    pub escape_character: Option<String>,
    /// Specifies the character that separates columns within each row (line) of the file.
    #[serde(rename = "fieldsTerminatedBy", skip_serializing_if = "Option::is_none")]
    pub fields_terminated_by: Option<String>,
    /// This is used to separate lines. If a line does not contain all fields, the rest of the columns are set to their default values.
    #[serde(rename = "linesTerminatedBy", skip_serializing_if = "Option::is_none")]
    pub lines_terminated_by: Option<String>,
    /// Specifies the quoting character to be used when a data value is quoted.
    #[serde(rename = "quoteCharacter", skip_serializing_if = "Option::is_none")]
    pub quote_character: Option<String>,
    /// The table to which CSV data is imported.
    #[serde(rename = "table", skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
}

impl ImportContextCsvImportOptions {
    /// Options for importing data as CSV.
    pub fn new() -> ImportContextCsvImportOptions {
        ImportContextCsvImportOptions {
            columns: None,
            escape_character: None,
            fields_terminated_by: None,
            lines_terminated_by: None,
            quote_character: None,
            table: None,
        }
    }
}
