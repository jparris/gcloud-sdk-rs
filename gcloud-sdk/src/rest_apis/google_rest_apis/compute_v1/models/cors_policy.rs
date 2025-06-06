use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::compute_v1::models;

/// CorsPolicy : The specification for allowing client-side cross-origin requests. For more information about the W3C recommendation for cross-origin resource sharing (CORS), see Fetch API Living Standard.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CorsPolicy {
    /// In response to a preflight request, setting this to true indicates that the actual request can include user credentials. This field translates to the Access-Control-Allow-Credentials header. Default is false.
    #[serde(rename = "allowCredentials", skip_serializing_if = "Option::is_none")]
    pub allow_credentials: Option<bool>,
    /// Specifies the content for the Access-Control-Allow-Headers header.
    #[serde(rename = "allowHeaders", skip_serializing_if = "Option::is_none")]
    pub allow_headers: Option<Vec<String>>,
    /// Specifies the content for the Access-Control-Allow-Methods header.
    #[serde(rename = "allowMethods", skip_serializing_if = "Option::is_none")]
    pub allow_methods: Option<Vec<String>>,
    /// Specifies a regular expression that matches allowed origins. For more information about the regular expression syntax, see Syntax. An origin is allowed if it matches either an item in allowOrigins or an item in allowOriginRegexes. Regular expressions can only be used when the loadBalancingScheme is set to INTERNAL_SELF_MANAGED.
    #[serde(rename = "allowOriginRegexes", skip_serializing_if = "Option::is_none")]
    pub allow_origin_regexes: Option<Vec<String>>,
    /// Specifies the list of origins that is allowed to do CORS requests. An origin is allowed if it matches either an item in allowOrigins or an item in allowOriginRegexes.
    #[serde(rename = "allowOrigins", skip_serializing_if = "Option::is_none")]
    pub allow_origins: Option<Vec<String>>,
    /// If true, the setting specifies the CORS policy is disabled. The default value of false, which indicates that the CORS policy is in effect.
    #[serde(rename = "disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// Specifies the content for the Access-Control-Expose-Headers header.
    #[serde(rename = "exposeHeaders", skip_serializing_if = "Option::is_none")]
    pub expose_headers: Option<Vec<String>>,
    /// Specifies how long results of a preflight request can be cached in seconds. This field translates to the Access-Control-Max-Age header.
    #[serde(rename = "maxAge", skip_serializing_if = "Option::is_none")]
    pub max_age: Option<i32>,
}

impl CorsPolicy {
    /// The specification for allowing client-side cross-origin requests. For more information about the W3C recommendation for cross-origin resource sharing (CORS), see Fetch API Living Standard.
    pub fn new() -> CorsPolicy {
        CorsPolicy {
            allow_credentials: None,
            allow_headers: None,
            allow_methods: None,
            allow_origin_regexes: None,
            allow_origins: None,
            disabled: None,
            expose_headers: None,
            max_age: None,
        }
    }
}
