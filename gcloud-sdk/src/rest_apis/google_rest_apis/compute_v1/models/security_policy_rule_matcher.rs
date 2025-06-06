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

/// SecurityPolicyRuleMatcher : Represents a match condition that incoming traffic is evaluated against. Exactly one field must be specified.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityPolicyRuleMatcher {
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Box<models::SecurityPolicyRuleMatcherConfig>>,
    #[serde(rename = "expr", skip_serializing_if = "Option::is_none")]
    pub expr: Option<Box<models::Expr>>,
    #[serde(rename = "exprOptions", skip_serializing_if = "Option::is_none")]
    pub expr_options: Option<Box<models::SecurityPolicyRuleMatcherExprOptions>>,
    /// Preconfigured versioned expression. If this field is specified, config must also be specified. Available preconfigured expressions along with their requirements are: SRC_IPS_V1 - must specify the corresponding src_ip_range field in config.
    #[serde(rename = "versionedExpr", skip_serializing_if = "Option::is_none")]
    pub versioned_expr: Option<VersionedExpr>,
}

impl SecurityPolicyRuleMatcher {
    /// Represents a match condition that incoming traffic is evaluated against. Exactly one field must be specified.
    pub fn new() -> SecurityPolicyRuleMatcher {
        SecurityPolicyRuleMatcher {
            config: None,
            expr: None,
            expr_options: None,
            versioned_expr: None,
        }
    }
}
/// Preconfigured versioned expression. If this field is specified, config must also be specified. Available preconfigured expressions along with their requirements are: SRC_IPS_V1 - must specify the corresponding src_ip_range field in config.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VersionedExpr {
    #[serde(rename = "SRC_IPS_V1")]
    SrcIpsV1,
}

impl Default for VersionedExpr {
    fn default() -> VersionedExpr {
        Self::SrcIpsV1
    }
}
