use krator::ObjectStatus;
use kube_derive::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// TODO: Determine if this is helpful anywhere...
// Define an alias for the actual WalrusPod custom resource
// pub type WalrusPod = kube::CustomResource<WalrusPodSpec, WalrusPodStatus>;

#[derive(Clone, Debug, CustomResource, Serialize, Deserialize, Default, JsonSchema)]
// #[kube(group = "krator-rs.io", version = "v1", kind = "WalrusPod", namespaced)]
#[kube(
    group = "animals.io",
    version = "v1",
    kind = "Walrus",
    derive = "Default",
    status = "WalrusStatus",
    namespaced
)]
#[serde(rename_all = "camelCase")]
pub struct WalrusSpec {
    // TODO: I need to come back to these and get better fields in the spec as it
    //  relates to the states / phases defined below
    pub age: u32,
    pub name: String,
    pub tusks: bool,
    pub weight: f64,
}

// In the krator example they also setup a version of the `WalrusSpec` for the
// admission webhook config...

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub enum WalrusPhase {
    Roaming,
    Hungry,
    Asleep, /*
              WakingUp,
              Eating,
              Swimming,
              Basking,
              Playing,
              Socializing,
              Resting,
              Sleepy,
              Sleeping,
            */
}

// TODO: Extend the status as it relates to the states above. For instance
//  how long has the walrus been asleep, or been between eating, or spent
//  time playing or socializing VS sleeping
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct WalrusStatus {
    pub phase: Option<WalrusPhase>,
    pub message: Option<String>,
}

impl ObjectStatus for WalrusStatus {
    fn failed(e: &str) -> WalrusStatus {
        WalrusStatus {
            message: Some(format!("Error tracking walrus: {}.", e)),
            phase: None,
        }
    }

    fn json_patch(&self) -> serde_json::Value {
        // Generate a map containing only set fields.
        let mut status = serde_json::Map::new();

        if let Some(phase) = self.phase.clone() {
            status.insert("phase".to_string(), serde_json::json!(phase));
        };

        if let Some(message) = self.message.clone() {
            status.insert("message".to_string(), serde_json::Value::String(message));
        };

        // Create status patch with map.
        serde_json::json!({ "status": serde_json::Value::Object(status) })
    }
}
