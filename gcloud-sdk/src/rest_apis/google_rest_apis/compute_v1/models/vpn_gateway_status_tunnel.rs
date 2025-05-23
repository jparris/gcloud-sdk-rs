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

/// VpnGatewayStatusTunnel : Contains some information about a VPN tunnel.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VpnGatewayStatusTunnel {
    /// The VPN gateway interface this VPN tunnel is associated with.
    #[serde(
        rename = "localGatewayInterface",
        skip_serializing_if = "Option::is_none"
    )]
    pub local_gateway_interface: Option<i32>,
    /// The peer gateway interface this VPN tunnel is connected to, the peer gateway could either be an external VPN gateway or a Google Cloud VPN gateway.
    #[serde(
        rename = "peerGatewayInterface",
        skip_serializing_if = "Option::is_none"
    )]
    pub peer_gateway_interface: Option<i32>,
    /// URL reference to the VPN tunnel.
    #[serde(rename = "tunnelUrl", skip_serializing_if = "Option::is_none")]
    pub tunnel_url: Option<String>,
}

impl VpnGatewayStatusTunnel {
    /// Contains some information about a VPN tunnel.
    pub fn new() -> VpnGatewayStatusTunnel {
        VpnGatewayStatusTunnel {
            local_gateway_interface: None,
            peer_gateway_interface: None,
            tunnel_url: None,
        }
    }
}
