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

/// Backend : Message containing information of one individual backend.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Backend {
    /// Specifies how to determine whether the backend of a load balancer can handle additional traffic or is fully loaded. For usage guidelines, see Connection balancing mode. Backends must use compatible balancing modes. For more information, see Supported balancing modes and target capacity settings and Restrictions and guidance for instance groups. Note: Currently, if you use the API to configure incompatible balancing modes, the configuration might be accepted even though it has no impact and is ignored. Specifically, Backend.maxUtilization is ignored when Backend.balancingMode is RATE. In the future, this incompatible combination will be rejected.
    #[serde(rename = "balancingMode", skip_serializing_if = "Option::is_none")]
    pub balancing_mode: Option<BalancingMode>,
    /// A multiplier applied to the backend's target capacity of its balancing mode. The default value is 1, which means the group serves up to 100% of its configured capacity (depending on balancingMode). A setting of 0 means the group is completely drained, offering 0% of its available capacity. The valid ranges are 0.0 and [0.1,1.0]. You cannot configure a setting larger than 0 and smaller than 0.1. You cannot configure a setting of 0 when there is only one backend attached to the backend service. Not available with backends that don't support using a balancingMode. This includes backends such as global internet NEGs, regional serverless NEGs, and PSC NEGs.
    #[serde(rename = "capacityScaler", skip_serializing_if = "Option::is_none")]
    pub capacity_scaler: Option<f32>,
    /// An optional description of this resource. Provide this property when you create the resource.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// This field designates whether this is a failover backend. More than one failover backend can be configured for a given BackendService.
    #[serde(rename = "failover", skip_serializing_if = "Option::is_none")]
    pub failover: Option<bool>,
    /// The fully-qualified URL of an instance group or network endpoint group (NEG) resource. To determine what types of backends a load balancer supports, see the [Backend services overview](https://cloud.google.com/load-balancing/docs/backend-service#backends). You must use the *fully-qualified* URL (starting with https://www.googleapis.com/) to specify the instance group or NEG. Partial URLs are not supported.
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Defines a target maximum number of simultaneous connections. For usage guidelines, see Connection balancing mode and Utilization balancing mode. Not available if the backend's balancingMode is RATE.
    #[serde(rename = "maxConnections", skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i32>,
    /// Defines a target maximum number of simultaneous connections. For usage guidelines, see Connection balancing mode and Utilization balancing mode. Not available if the backend's balancingMode is RATE.
    #[serde(
        rename = "maxConnectionsPerEndpoint",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_connections_per_endpoint: Option<i32>,
    /// Defines a target maximum number of simultaneous connections. For usage guidelines, see Connection balancing mode and Utilization balancing mode. Not available if the backend's balancingMode is RATE.
    #[serde(
        rename = "maxConnectionsPerInstance",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_connections_per_instance: Option<i32>,
    /// Defines a maximum number of HTTP requests per second (RPS). For usage guidelines, see Rate balancing mode and Utilization balancing mode. Not available if the backend's balancingMode is CONNECTION.
    #[serde(rename = "maxRate", skip_serializing_if = "Option::is_none")]
    pub max_rate: Option<i32>,
    /// Defines a maximum target for requests per second (RPS). For usage guidelines, see Rate balancing mode and Utilization balancing mode. Not available if the backend's balancingMode is CONNECTION.
    #[serde(rename = "maxRatePerEndpoint", skip_serializing_if = "Option::is_none")]
    pub max_rate_per_endpoint: Option<f32>,
    /// Defines a maximum target for requests per second (RPS). For usage guidelines, see Rate balancing mode and Utilization balancing mode. Not available if the backend's balancingMode is CONNECTION.
    #[serde(rename = "maxRatePerInstance", skip_serializing_if = "Option::is_none")]
    pub max_rate_per_instance: Option<f32>,
    /// Optional parameter to define a target capacity for the UTILIZATION balancing mode. The valid range is [0.0, 1.0]. For usage guidelines, see Utilization balancing mode.
    #[serde(rename = "maxUtilization", skip_serializing_if = "Option::is_none")]
    pub max_utilization: Option<f32>,
    /// This field indicates whether this backend should be fully utilized before sending traffic to backends with default preference. The possible values are: - PREFERRED: Backends with this preference level will be filled up to their capacity limits first, based on RTT. - DEFAULT: If preferred backends don't have enough capacity, backends in this layer would be used and traffic would be assigned based on the load balancing algorithm you use. This is the default
    #[serde(rename = "preference", skip_serializing_if = "Option::is_none")]
    pub preference: Option<Preference>,
}

impl Backend {
    /// Message containing information of one individual backend.
    pub fn new() -> Backend {
        Backend {
            balancing_mode: None,
            capacity_scaler: None,
            description: None,
            failover: None,
            group: None,
            max_connections: None,
            max_connections_per_endpoint: None,
            max_connections_per_instance: None,
            max_rate: None,
            max_rate_per_endpoint: None,
            max_rate_per_instance: None,
            max_utilization: None,
            preference: None,
        }
    }
}
/// Specifies how to determine whether the backend of a load balancer can handle additional traffic or is fully loaded. For usage guidelines, see Connection balancing mode. Backends must use compatible balancing modes. For more information, see Supported balancing modes and target capacity settings and Restrictions and guidance for instance groups. Note: Currently, if you use the API to configure incompatible balancing modes, the configuration might be accepted even though it has no impact and is ignored. Specifically, Backend.maxUtilization is ignored when Backend.balancingMode is RATE. In the future, this incompatible combination will be rejected.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BalancingMode {
    #[serde(rename = "CONNECTION")]
    Connection,
    #[serde(rename = "RATE")]
    Rate,
    #[serde(rename = "UTILIZATION")]
    Utilization,
}

impl Default for BalancingMode {
    fn default() -> BalancingMode {
        Self::Connection
    }
}
/// This field indicates whether this backend should be fully utilized before sending traffic to backends with default preference. The possible values are: - PREFERRED: Backends with this preference level will be filled up to their capacity limits first, based on RTT. - DEFAULT: If preferred backends don't have enough capacity, backends in this layer would be used and traffic would be assigned based on the load balancing algorithm you use. This is the default
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Preference {
    #[serde(rename = "DEFAULT")]
    Default,
    #[serde(rename = "PREFERENCE_UNSPECIFIED")]
    PreferenceUnspecified,
    #[serde(rename = "PREFERRED")]
    Preferred,
}

impl Default for Preference {
    fn default() -> Preference {
        Self::Default
    }
}
