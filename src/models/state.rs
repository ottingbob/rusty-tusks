use std::sync::Arc;
use krator::{Manifest, ObjectState, State, Transition, TransitionTo};
use krator_derive::*;
use tokio::sync::RwLock;
use std::collections::{HashMap, HashSet};
use rand::Rng;
use rand::seq::IteratorRandom;
use tracing::info;

use super::pod::{WalrusPhase, WalrusStatus, Walrus};

pub struct WalrusState {
    pub name: String,
    pub food: f64,
}

#[async_trait::async_trait]
impl ObjectState for WalrusState {
    // The `Manifest` will be generated from the derive
    // from the `WalrusSpec`
    type Manifest = Walrus;
    type Status = WalrusStatus;
    type SharedState = SharedWalrusState;
    async fn async_drop(self, shared: &mut Self::SharedState) {
        shared.friends.remove(&self.name);
    }
}

#[derive(Debug, Default)]
// Walrus was tagged for tracking.
pub struct Tagged;

#[async_trait::async_trait]
impl State<WalrusState> for Tagged {
    async fn next(
        self: Box<Self>,
        shared: Arc<RwLock<SharedWalrusState>>,
        state: &mut WalrusState,
        _manifest: Manifest<Walrus>,
    ) -> Transition<WalrusState> {
        info!("Found new walrus named {}!", state.name);
        shared
            .write()
            .await
            .friends
            .insert(state.name.clone(), HashSet::new());
        Transition::next(self, Roam)
    }

    async fn status(
        &self,
        _state: &mut WalrusState,
        _manifest: &Walrus,
    ) -> anyhow::Result<WalrusStatus> {
        Ok(WalrusStatus {
            phase: Some(WalrusPhase::Roaming),
            message: None,
        })
    }
}

// Explicitly implement TransitionTo
impl TransitionTo<Roam> for Tagged {}

// Derive TransitionTo
#[derive(Debug, Default, TransitionTo)]
// Specify valid next states.
#[transition_to(Eat)]
/// Walrus is roaming the wilderness.
struct Roam;

async fn make_friend(name: &str, shared: &Arc<RwLock<SharedWalrusState>>) -> Option<String> {
    let mut walruses = shared.write().await;
    let mut rng = rand::thread_rng();
    let other_walruses =
        walruses
          .friends
          .keys()
          .map(|s| s.to_owned())
          .choose_multiple(&mut rng, walruses.friends.len());
    for other_walrus in other_walruses {
        if name == other_walrus {
            continue;
        }

        let friends = walruses.friends.get_mut(&other_walrus).unwrap();
        if !friends.contains(name) {
            friends.insert(name.to_string());
            return Some(other_walrus.to_string());
        }
    }
    return None;
}

#[async_trait::async_trait]
impl State<WalrusState> for Roam {
    async fn next(
        self: Box<Self>,
        shared: Arc<RwLock<SharedWalrusState>>,
        state: &mut WalrusState,
        _manifest: Manifest<Walrus>,
    ) -> Transition<WalrusState> {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;

            state.food -= 5.0;
            if state.food <= 10.0 {
                return Transition::next(self, Eat);
            }

            let r: f64 = rand::thread_rng().gen();
            if r < 0.05 {
                if let Some(other_walrus) = make_friend(&state.name, &shared).await {
                    println!("{} made friends with {}!", state.name, other_walrus);
                }
            }
        }
    }

    async fn status(
        &self,
        _state: &mut WalrusState,
        _manifest: &Walrus,
    ) -> anyhow::Result<WalrusStatus> {
        Ok(WalrusStatus {
            phase: Some(WalrusPhase::Roaming),
            message: Some("Repppoooo!".to_string()),
        })
    }
}

#[derive(Debug, Default, TransitionTo)]
#[transition_to(Sleep)]
/// Walrus is eating.
struct Eat;

#[async_trait::async_trait]
impl State<WalrusState> for Eat {
    async fn next(
        self: Box<Self>,
        _shared: Arc<RwLock<SharedWalrusState>>,
        state: &mut WalrusState,
        manifest: Manifest<Walrus>,
    ) -> Transition<WalrusState> {
        let walrus = manifest.latest();
        state.food = walrus.spec.weight / 10.0;
        tokio::time::sleep(std::time::Duration::from_secs((state.food / 10.0) as u64)).await;
        Transition::next(self, Sleep)
    }

    async fn status(
        &self,
        _state: &mut WalrusState,
        _manifest: &Walrus,
    ) -> anyhow::Result<WalrusStatus> {
        Ok(WalrusStatus {
            phase: Some(WalrusPhase::Hungry),
            message: Some("*crunch*".to_string()),
        })
    }
}

#[derive(Debug, Default, TransitionTo)]
#[transition_to(Roam)]
/// Walrus is sleeping.
struct Sleep;

#[async_trait::async_trait]
impl State<WalrusState> for Sleep {
    async fn next(
        self: Box<Self>,
        _shared: Arc<RwLock<SharedWalrusState>>,
        _state: &mut WalrusState,
        _manifest: Manifest<Walrus>,
    ) -> Transition<WalrusState> {
        tokio::time::sleep(std::time::Duration::from_secs(20)).await;
        Transition::next(self, Roam)
    }

    async fn status(
        &self,
        _state: &mut WalrusState,
        _manifest: &Walrus,
    ) -> anyhow::Result<WalrusStatus> {
        Ok(WalrusStatus {
            phase: Some(WalrusPhase::Asleep),
            message: Some("zzzzzz".to_string()),
        })
    }
}

#[derive(Debug, Default)]
/// Walrus was released from our care.
pub struct Released;

#[async_trait::async_trait]
impl State<WalrusState> for Released {
    async fn next(
        self: Box<Self>,
        _shared: Arc<RwLock<SharedWalrusState>>,
        _state: &mut WalrusState,
        _manifest: Manifest<Walrus>,
    ) -> Transition<WalrusState> {
        info!("Walrus tagged for release!");
        Transition::Complete(Ok(()))
    }

    async fn status(
        &self,
        state: &mut WalrusState,
        _manifest: &Walrus,
    ) -> anyhow::Result<WalrusStatus> {
        Ok(WalrusStatus {
            phase: None,
            message: Some(format!("Bye, {}!", state.name)),
        })
    }
}

pub struct SharedWalrusState {
    pub friends: HashMap<String, HashSet<String>>,
}
