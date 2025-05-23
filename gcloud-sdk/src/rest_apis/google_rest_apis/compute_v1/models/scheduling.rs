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

/// Scheduling : Sets the scheduling options for an Instance.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Scheduling {
    /// Specifies whether the instance should be automatically restarted if it is terminated by Compute Engine (not terminated by a user). You can only set the automatic restart option for standard instances. Preemptible instances cannot be automatically restarted. By default, this is set to true so an instance is automatically restarted if it is terminated by Compute Engine.
    #[serde(rename = "automaticRestart", skip_serializing_if = "Option::is_none")]
    pub automatic_restart: Option<bool>,
    /// Specifies the termination action for the instance.
    #[serde(
        rename = "instanceTerminationAction",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub instance_termination_action: Option<Option<InstanceTerminationAction>>,
    #[serde(
        rename = "localSsdRecoveryTimeout",
        skip_serializing_if = "Option::is_none"
    )]
    pub local_ssd_recovery_timeout: Option<Box<models::Duration>>,
    /// An opaque location hint used to place the instance close to other resources. This field is for use by internal tools that use the public API.
    #[serde(rename = "locationHint", skip_serializing_if = "Option::is_none")]
    pub location_hint: Option<String>,
    /// The minimum number of virtual CPUs this instance will consume when running on a sole-tenant node.
    #[serde(rename = "minNodeCpus", skip_serializing_if = "Option::is_none")]
    pub min_node_cpus: Option<i32>,
    /// A set of node affinity and anti-affinity configurations. Refer to Configuring node affinity for more information. Overrides reservationAffinity.
    #[serde(rename = "nodeAffinities", skip_serializing_if = "Option::is_none")]
    pub node_affinities: Option<Vec<models::SchedulingNodeAffinity>>,
    /// Defines the maintenance behavior for this instance. For standard instances, the default behavior is MIGRATE. For preemptible instances, the default and only possible behavior is TERMINATE. For more information, see Set VM host maintenance policy.
    #[serde(rename = "onHostMaintenance", skip_serializing_if = "Option::is_none")]
    pub on_host_maintenance: Option<OnHostMaintenance>,
    /// Defines whether the instance is preemptible. This can only be set during instance creation or while the instance is stopped and therefore, in a `TERMINATED` state. See Instance Life Cycle for more information on the possible instance states.
    #[serde(rename = "preemptible", skip_serializing_if = "Option::is_none")]
    pub preemptible: Option<bool>,
    /// Specifies the provisioning model of the instance.
    #[serde(rename = "provisioningModel", skip_serializing_if = "Option::is_none")]
    pub provisioning_model: Option<ProvisioningModel>,
}

impl Scheduling {
    /// Sets the scheduling options for an Instance.
    pub fn new() -> Scheduling {
        Scheduling {
            automatic_restart: None,
            instance_termination_action: None,
            local_ssd_recovery_timeout: None,
            location_hint: None,
            min_node_cpus: None,
            node_affinities: None,
            on_host_maintenance: None,
            preemptible: None,
            provisioning_model: None,
        }
    }
}
/// Specifies the termination action for the instance.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InstanceTerminationAction {
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "INSTANCE_TERMINATION_ACTION_UNSPECIFIED")]
    InstanceTerminationActionUnspecified,
    #[serde(rename = "STOP")]
    Stop,
}

impl Default for InstanceTerminationAction {
    fn default() -> InstanceTerminationAction {
        Self::Delete
    }
}
/// Defines the maintenance behavior for this instance. For standard instances, the default behavior is MIGRATE. For preemptible instances, the default and only possible behavior is TERMINATE. For more information, see Set VM host maintenance policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OnHostMaintenance {
    #[serde(rename = "MIGRATE")]
    Migrate,
    #[serde(rename = "TERMINATE")]
    Terminate,
}

impl Default for OnHostMaintenance {
    fn default() -> OnHostMaintenance {
        Self::Migrate
    }
}
/// Specifies the provisioning model of the instance.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProvisioningModel {
    #[serde(rename = "SPOT")]
    Spot,
    #[serde(rename = "STANDARD")]
    Standard,
}

impl Default for ProvisioningModel {
    fn default() -> ProvisioningModel {
        Self::Spot
    }
}
