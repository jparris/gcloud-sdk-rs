use serde::{Deserialize, Serialize}; /*
                                      * Service Control API
                                      *
                                      * Provides admission control and telemetry reporting for services integrated with Service Infrastructure.
                                      *
                                      * The version of the OpenAPI document: v2
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::servicecontrol_v2::models;

/// ResourceLocation : Location information about a resource.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceLocation {
    /// The locations of a resource after the execution of the operation. Requests to create or delete a location based resource must populate the 'current_locations' field and not the 'original_locations' field. For example: \"europe-west1-a\" \"us-east1\" \"nam3\"
    #[serde(rename = "currentLocations", skip_serializing_if = "Option::is_none")]
    pub current_locations: Option<Vec<String>>,
    /// The locations of a resource prior to the execution of the operation. Requests that mutate the resource's location must populate both the 'original_locations' as well as the 'current_locations' fields. For example: \"europe-west1-a\" \"us-east1\" \"nam3\"
    #[serde(rename = "originalLocations", skip_serializing_if = "Option::is_none")]
    pub original_locations: Option<Vec<String>>,
}

impl ResourceLocation {
    /// Location information about a resource.
    pub fn new() -> ResourceLocation {
        ResourceLocation {
            current_locations: None,
            original_locations: None,
        }
    }
}
