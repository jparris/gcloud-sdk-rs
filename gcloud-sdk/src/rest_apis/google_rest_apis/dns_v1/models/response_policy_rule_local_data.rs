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
pub struct ResponsePolicyRuleLocalData {
    /// All resource record sets for this selector, one per resource record type. The name must match the dns_name.
    #[serde(rename = "localDatas", skip_serializing_if = "Option::is_none")]
    pub local_datas: Option<Vec<models::ResourceRecordSet>>,
}

impl ResponsePolicyRuleLocalData {
    pub fn new() -> ResponsePolicyRuleLocalData {
        ResponsePolicyRuleLocalData { local_datas: None }
    }
}
