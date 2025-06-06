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

/// Project : Represents a Project resource. A project is used to organize resources in a Google Cloud Platform environment. For more information, read about the Resource Hierarchy.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    /// [Output Only] The Cloud Armor tier for this project. It can be one of the following values: CA_STANDARD, CA_ENTERPRISE_PAYGO. If this field is not specified, it is assumed to be CA_STANDARD.
    #[serde(rename = "cloudArmorTier", skip_serializing_if = "Option::is_none")]
    pub cloud_armor_tier: Option<CloudArmorTier>,
    #[serde(
        rename = "commonInstanceMetadata",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_instance_metadata: Option<Box<models::Metadata>>,
    /// [Output Only] Creation timestamp in RFC3339 text format.
    #[serde(rename = "creationTimestamp", skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// This signifies the default network tier used for configuring resources of the project and can only take the following values: PREMIUM, STANDARD. Initially the default network tier is PREMIUM.
    #[serde(rename = "defaultNetworkTier", skip_serializing_if = "Option::is_none")]
    pub default_network_tier: Option<DefaultNetworkTier>,
    /// [Output Only] Default service account used by VMs running in this project.
    #[serde(
        rename = "defaultServiceAccount",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_service_account: Option<String>,
    /// An optional textual description of the resource.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Restricted features enabled for use on this project.
    #[serde(rename = "enabledFeatures", skip_serializing_if = "Option::is_none")]
    pub enabled_features: Option<Vec<String>>,
    /// [Output Only] The unique identifier for the resource. This identifier is defined by the server. This is *not* the project ID, and is just a unique ID used by Compute Engine to identify resources.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// [Output Only] Type of the resource. Always compute#project for projects.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The project ID. For example: my-example-project. Use the project ID to make requests to Compute Engine.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// [Output Only] Quotas assigned to this project.
    #[serde(rename = "quotas", skip_serializing_if = "Option::is_none")]
    pub quotas: Option<Vec<models::Quota>>,
    /// [Output Only] Server-defined URL for the resource.
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    #[serde(
        rename = "usageExportLocation",
        skip_serializing_if = "Option::is_none"
    )]
    pub usage_export_location: Option<Box<models::UsageExportLocation>>,
    /// [Output Only] Default internal DNS setting used by VMs running in this project.
    #[serde(rename = "vmDnsSetting", skip_serializing_if = "Option::is_none")]
    pub vm_dns_setting: Option<VmDnsSetting>,
    /// [Output Only] The role this project has in a shared VPC configuration. Currently, only projects with the host role, which is specified by the value HOST, are differentiated.
    #[serde(rename = "xpnProjectStatus", skip_serializing_if = "Option::is_none")]
    pub xpn_project_status: Option<XpnProjectStatus>,
}

impl Project {
    /// Represents a Project resource. A project is used to organize resources in a Google Cloud Platform environment. For more information, read about the Resource Hierarchy.
    pub fn new() -> Project {
        Project {
            cloud_armor_tier: None,
            common_instance_metadata: None,
            creation_timestamp: None,
            default_network_tier: None,
            default_service_account: None,
            description: None,
            enabled_features: None,
            id: None,
            kind: None,
            name: None,
            quotas: None,
            self_link: None,
            usage_export_location: None,
            vm_dns_setting: None,
            xpn_project_status: None,
        }
    }
}
/// [Output Only] The Cloud Armor tier for this project. It can be one of the following values: CA_STANDARD, CA_ENTERPRISE_PAYGO. If this field is not specified, it is assumed to be CA_STANDARD.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CloudArmorTier {
    #[serde(rename = "CA_ENTERPRISE_ANNUAL")]
    EnterpriseAnnual,
    #[serde(rename = "CA_ENTERPRISE_PAYGO")]
    EnterprisePaygo,
    #[serde(rename = "CA_STANDARD")]
    Standard,
}

impl Default for CloudArmorTier {
    fn default() -> CloudArmorTier {
        Self::EnterpriseAnnual
    }
}
/// This signifies the default network tier used for configuring resources of the project and can only take the following values: PREMIUM, STANDARD. Initially the default network tier is PREMIUM.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DefaultNetworkTier {
    #[serde(rename = "FIXED_STANDARD")]
    FixedStandard,
    #[serde(rename = "PREMIUM")]
    Premium,
    #[serde(rename = "STANDARD")]
    Standard,
    #[serde(rename = "STANDARD_OVERRIDES_FIXED_STANDARD")]
    StandardOverridesFixedStandard,
}

impl Default for DefaultNetworkTier {
    fn default() -> DefaultNetworkTier {
        Self::FixedStandard
    }
}
/// [Output Only] Default internal DNS setting used by VMs running in this project.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VmDnsSetting {
    #[serde(rename = "GLOBAL_DEFAULT")]
    GlobalDefault,
    #[serde(rename = "UNSPECIFIED_VM_DNS_SETTING")]
    UnspecifiedVmDnsSetting,
    #[serde(rename = "ZONAL_DEFAULT")]
    ZonalDefault,
    #[serde(rename = "ZONAL_ONLY")]
    ZonalOnly,
}

impl Default for VmDnsSetting {
    fn default() -> VmDnsSetting {
        Self::GlobalDefault
    }
}
/// [Output Only] The role this project has in a shared VPC configuration. Currently, only projects with the host role, which is specified by the value HOST, are differentiated.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum XpnProjectStatus {
    #[serde(rename = "HOST")]
    Host,
    #[serde(rename = "UNSPECIFIED_XPN_PROJECT_STATUS")]
    UnspecifiedXpnProjectStatus,
}

impl Default for XpnProjectStatus {
    fn default() -> XpnProjectStatus {
        Self::Host
    }
}
