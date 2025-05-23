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

/// NetworkAttachmentConnectedEndpoint : [Output Only] A connection connected to this network attachment.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkAttachmentConnectedEndpoint {
    /// The IPv4 address assigned to the producer instance network interface. This value will be a range in case of Serverless.
    #[serde(rename = "ipAddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// The IPv6 address assigned to the producer instance network interface. This is only assigned when the stack types of both the instance network interface and the consumer subnet are IPv4_IPv6.
    #[serde(rename = "ipv6Address", skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
    /// The project id or number of the interface to which the IP was assigned.
    #[serde(rename = "projectIdOrNum", skip_serializing_if = "Option::is_none")]
    pub project_id_or_num: Option<String>,
    /// Alias IP ranges from the same subnetwork.
    #[serde(
        rename = "secondaryIpCidrRanges",
        skip_serializing_if = "Option::is_none"
    )]
    pub secondary_ip_cidr_ranges: Option<Vec<String>>,
    /// The status of a connected endpoint to this network attachment.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The subnetwork used to assign the IP to the producer instance network interface.
    #[serde(rename = "subnetwork", skip_serializing_if = "Option::is_none")]
    pub subnetwork: Option<String>,
    /// [Output Only] The CIDR range of the subnet from which the IPv4 internal IP was allocated from.
    #[serde(
        rename = "subnetworkCidrRange",
        skip_serializing_if = "Option::is_none"
    )]
    pub subnetwork_cidr_range: Option<String>,
}

impl NetworkAttachmentConnectedEndpoint {
    /// [Output Only] A connection connected to this network attachment.
    pub fn new() -> NetworkAttachmentConnectedEndpoint {
        NetworkAttachmentConnectedEndpoint {
            ip_address: None,
            ipv6_address: None,
            project_id_or_num: None,
            secondary_ip_cidr_ranges: None,
            status: None,
            subnetwork: None,
            subnetwork_cidr_range: None,
        }
    }
}
/// The status of a connected endpoint to this network attachment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "ACCEPTED")]
    Accepted,
    #[serde(rename = "CLOSED")]
    Closed,
    #[serde(rename = "NEEDS_ATTENTION")]
    NeedsAttention,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "REJECTED")]
    Rejected,
    #[serde(rename = "STATUS_UNSPECIFIED")]
    StatusUnspecified,
}

impl Default for Status {
    fn default() -> Status {
        Self::Accepted
    }
}
