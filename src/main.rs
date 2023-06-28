use kube::api::ListParams;
use kube::CustomResourceExt;
use structopt::StructOpt;
use tracing::{debug, info};

use rusty_tusks::controller::WalrusTracker;

// pub mod models;
use rusty_tusks::models::pod::Walrus;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "walrus",
    about = "An example Operator for `Walrus` custom resources."
)]
struct Opt {
    /// Configure logger to emit JSON output.
    #[structopt(long)]
    json: bool,

    /// output walrus crd manifest
    #[structopt(long)]
    output_crd: bool,
}

// fn init_logger(opt: &Opt) -> anyhow::Result<Option<opentelemetry_jaeger::Uninstall>> {
fn init_logger(opt: &Opt) {
    // This isn't very DRY, but all of these combinations have different types,
    // and Boxing them doesn't seem to work.
    /*
    let guard = if opt.json {
        let _subscriber = tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
            // .json()
            .finish();
    } else {
      */
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .pretty()
        .finish();

    tracing::subscriber::set_global_default(subscriber);
    // None
    // };
    // Ok(guard)
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    let _guard = init_logger(&opt);

    if opt.output_crd {
        println!("{}", serde_yaml::to_string(&Walrus::crd()).unwrap());
        return Ok(());
    }

    let kubeconfig = kube::Config::infer().await?;
    debug!("Setting up with kubeconfig: {:?}", kubeconfig);

    info!("Starting Walrus Operator");
    let tracker = WalrusTracker::new();
    // Only track walrus in IUCN Glacier location
    let params = ListParams::default().labels("iucn.co/location=glacier");

    use krator::{ControllerBuilder, Manager};
    let mut manager = Manager::new(&kubeconfig);
    let controller = ControllerBuilder::new(tracker).with_params(params);
    manager.register_controller(controller);

    info!("Starting Walrus Tracker and Manager");
    manager.start().await;

    Ok(())
}
