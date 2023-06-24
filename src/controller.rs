use krator::Operator;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// TODO: When do I use either or...
use crate::controller::models::state::{Released, Tagged};
use models::pod::{Walrus, WalrusStatus};
use models::state::{SharedWalrusState, WalrusState};

mod models {
    include!("models/mod.rs");
}

pub struct WalrusTracker {
    shared: Arc<RwLock<SharedWalrusState>>,
}

impl WalrusTracker {
    /*
    #[cfg(feature = "admission-webhook")]
    fn new(client: &kube::Client) -> Self {
        let shared = Arc::new(RwLock::new(SharedWalrusState {
            friends: HashMap::new(),
            client: client.to_owned(),
        }));
        WalrusTracker { shared }
    }
    */

    // #[cfg(not(feature = "admission-webhook"))]
    pub fn new() -> Self {
        let shared = Arc::new(RwLock::new(SharedWalrusState {
            friends: HashMap::new(),
        }));
        WalrusTracker { shared }
    }
}

#[async_trait::async_trait]
impl Operator for WalrusTracker {
    type Manifest = Walrus;
    type Status = WalrusStatus;
    type InitialState = Tagged;
    type DeletedState = Released;
    type ObjectState = WalrusState;

    async fn initialize_object_state(
        &self,
        manifest: &Self::Manifest,
    ) -> anyhow::Result<Self::ObjectState> {
        let name = manifest.clone().metadata.clone().name.unwrap();
        // let name = manifest.meta().name.clone().unwrap();
        Ok(WalrusState {
            name,
            food: manifest.spec.weight / 10.0,
        })
    }

    async fn shared_state(&self) -> Arc<RwLock<SharedWalrusState>> {
        Arc::clone(&self.shared)
    }

    #[cfg(feature = "admission-webhook")]
    async fn admission_hook(
        &self,
        manifest: Self::Manifest,
    ) -> krator::admission::AdmissionResult<Self::Manifest> {
        use k8s_openapi::apimachinery::pkg::apis::meta::v1::Status;
        // All walrus names start with "M"
        let name = manifest.meta().name.clone().unwrap();
        // info!("Processing admission hook for walrus named {}", name);
        println!("Processing admission hook for walrus named {}", name);
        match name.chars().next() {
            Some('m') | Some('M') => krator::admission::AdmissionResult::Allow(manifest),
            _ => krator::admission::AdmissionResult::Deny(Status {
                code: Some(400),
                message: Some("Walruss may only have names starting with 'M'.".to_string()),
                status: Some("Failure".to_string()),
                ..Default::default()
            }),
        }
    }

    /*
      #[cfg(feature = "admission-webhook")]
      async fn admission_hook_tls(&self) -> anyhow::Result<krator::admission::AdmissionTls> {
          let client = self.shared.read().await.client.clone();
          let secret_name = Walrus::admission_webhook_secret_name();

          let opt = Opt::from_args();
          let secret = kube::Api::<Secret>::namespaced(client, &opt.webhook_namespace)
              .get(&secret_name)
              .await?;

          Ok(admission::AdmissionTls::from(&secret)?)
      }
    */
}
