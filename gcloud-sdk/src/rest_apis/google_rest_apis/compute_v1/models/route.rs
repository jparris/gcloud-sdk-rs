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

/// Route : Represents a Route resource. A route defines a path from VM instances in the VPC network to a specific destination. This destination can be inside or outside the VPC network. For more information, read the Routes overview.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Route {
    /// [Output Only] AS path.
    #[serde(rename = "asPaths", skip_serializing_if = "Option::is_none")]
    pub as_paths: Option<Vec<models::RouteAsPath>>,
    /// [Output Only] Creation timestamp in RFC3339 text format.
    #[serde(rename = "creationTimestamp", skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// An optional description of this resource. Provide this field when you create the resource.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The destination range of outgoing packets that this route applies to. Both IPv4 and IPv6 are supported. Must specify an IPv4 range (e.g. 192.0.2.0/24) or an IPv6 range in RFC 4291 format (e.g. 2001:db8::/32). IPv6 range will be displayed using RFC 5952 compressed format.
    #[serde(rename = "destRange", skip_serializing_if = "Option::is_none")]
    pub dest_range: Option<String>,
    /// [Output Only] The unique identifier for the resource. This identifier is defined by the server.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// [Output Only] Type of this resource. Always compute#routes for Route resources.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the resource. Provided by the client when the resource is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?`. The first character must be a lowercase letter, and all following characters (except for the last character) must be a dash, lowercase letter, or digit. The last character must be a lowercase letter or digit.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Fully-qualified URL of the network that this route applies to.
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// The URL to a gateway that should handle matching packets. You can only specify the internet gateway using a full or partial valid URL: projects/ project/global/gateways/default-internet-gateway
    #[serde(rename = "nextHopGateway", skip_serializing_if = "Option::is_none")]
    pub next_hop_gateway: Option<String>,
    /// [Output Only] The full resource name of the Network Connectivity Center hub that will handle matching packets.
    #[serde(rename = "nextHopHub", skip_serializing_if = "Option::is_none")]
    pub next_hop_hub: Option<String>,
    /// The URL to a forwarding rule of type loadBalancingScheme=INTERNAL that should handle matching packets or the IP address of the forwarding Rule. For example, the following are all valid URLs: - 10.128.0.56 - https://www.googleapis.com/compute/v1/projects/project/regions/region /forwardingRules/forwardingRule - regions/region/forwardingRules/forwardingRule
    #[serde(rename = "nextHopIlb", skip_serializing_if = "Option::is_none")]
    pub next_hop_ilb: Option<String>,
    /// The URL to an instance that should handle matching packets. You can specify this as a full or partial URL. For example: https://www.googleapis.com/compute/v1/projects/project/zones/zone/instances/
    #[serde(rename = "nextHopInstance", skip_serializing_if = "Option::is_none")]
    pub next_hop_instance: Option<String>,
    /// The network IP address of an instance that should handle matching packets. Both IPv6 address and IPv4 addresses are supported. Must specify an IPv4 address in dot-decimal notation (e.g. 192.0.2.99) or an IPv6 address in RFC 4291 format (e.g. 2001:db8::2d9:51:0:0 or 2001:db8:0:0:2d9:51:0:0). IPv6 addresses will be displayed using RFC 5952 compressed format (e.g. 2001:db8::2d9:51:0:0). Should never be an IPv4-mapped IPv6 address.
    #[serde(rename = "nextHopIp", skip_serializing_if = "Option::is_none")]
    pub next_hop_ip: Option<String>,
    /// The URL of the local network if it should handle matching packets.
    #[serde(rename = "nextHopNetwork", skip_serializing_if = "Option::is_none")]
    pub next_hop_network: Option<String>,
    /// [Output Only] The network peering name that should handle matching packets, which should conform to RFC1035.
    #[serde(rename = "nextHopPeering", skip_serializing_if = "Option::is_none")]
    pub next_hop_peering: Option<String>,
    /// The URL to a VpnTunnel that should handle matching packets.
    #[serde(rename = "nextHopVpnTunnel", skip_serializing_if = "Option::is_none")]
    pub next_hop_vpn_tunnel: Option<String>,
    /// The priority of this route. Priority is used to break ties in cases where there is more than one matching route of equal prefix length. In cases where multiple routes have equal prefix length, the one with the lowest-numbered priority value wins. The default value is `1000`. The priority value must be from `0` to `65535`, inclusive.
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// [Output only] The status of the route.
    #[serde(rename = "routeStatus", skip_serializing_if = "Option::is_none")]
    pub route_status: Option<RouteStatus>,
    /// [Output Only] The type of this route, which can be one of the following values: - 'TRANSIT' for a transit route that this router learned from another Cloud Router and will readvertise to one of its BGP peers - 'SUBNET' for a route from a subnet of the VPC - 'BGP' for a route learned from a BGP peer of this router - 'STATIC' for a static route
    #[serde(rename = "routeType", skip_serializing_if = "Option::is_none")]
    pub route_type: Option<RouteType>,
    /// [Output Only] Server-defined fully-qualified URL for this resource.
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    /// A list of instance tags to which this route applies.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// [Output Only] If potential misconfigurations are detected for this route, this field will be populated with warning messages.
    #[serde(rename = "warnings", skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<models::OperationWarningsInner>>,
}

impl Route {
    /// Represents a Route resource. A route defines a path from VM instances in the VPC network to a specific destination. This destination can be inside or outside the VPC network. For more information, read the Routes overview.
    pub fn new() -> Route {
        Route {
            as_paths: None,
            creation_timestamp: None,
            description: None,
            dest_range: None,
            id: None,
            kind: None,
            name: None,
            network: None,
            next_hop_gateway: None,
            next_hop_hub: None,
            next_hop_ilb: None,
            next_hop_instance: None,
            next_hop_ip: None,
            next_hop_network: None,
            next_hop_peering: None,
            next_hop_vpn_tunnel: None,
            priority: None,
            route_status: None,
            route_type: None,
            self_link: None,
            tags: None,
            warnings: None,
        }
    }
}
/// [Output only] The status of the route.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RouteStatus {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "DROPPED")]
    Dropped,
    #[serde(rename = "INACTIVE")]
    Inactive,
    #[serde(rename = "PENDING")]
    Pending,
}

impl Default for RouteStatus {
    fn default() -> RouteStatus {
        Self::Active
    }
}
/// [Output Only] The type of this route, which can be one of the following values: - 'TRANSIT' for a transit route that this router learned from another Cloud Router and will readvertise to one of its BGP peers - 'SUBNET' for a route from a subnet of the VPC - 'BGP' for a route learned from a BGP peer of this router - 'STATIC' for a static route
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RouteType {
    #[serde(rename = "BGP")]
    Bgp,
    #[serde(rename = "STATIC")]
    Static,
    #[serde(rename = "SUBNET")]
    Subnet,
    #[serde(rename = "TRANSIT")]
    Transit,
}

impl Default for RouteType {
    fn default() -> RouteType {
        Self::Bgp
    }
}
