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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RouterBgpPeerCustomLearnedIpRange {
    /// The custom learned route IP address range. Must be a valid CIDR-formatted prefix. If an IP address is provided without a subnet mask, it is interpreted as, for IPv4, a `/32` singular IP address range, and, for IPv6, `/128`.
    #[serde(rename = "range", skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
}

impl RouterBgpPeerCustomLearnedIpRange {
    pub fn new() -> RouterBgpPeerCustomLearnedIpRange {
        RouterBgpPeerCustomLearnedIpRange { range: None }
    }
}
