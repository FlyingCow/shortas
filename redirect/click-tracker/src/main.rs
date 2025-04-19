use std::thread;

use anyhow::Result;

use clap::Parser;
use signal_hook::{consts::SIGINT, iterator::Signals};

use click_tracker::{core::tracking_pipe::BaseTrackingPipe, settings::Settings, AppBuilder};
use tokio_util::sync::CancellationToken;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    #[arg(short, long, default_value_t = String::from("production"), env("APP_RUN_MODE"))]
    pub run_mode: String,
    #[arg(short, long, default_value_t = String::from("./config"), env("APP_CONFIG_PATH"))]
    pub config_path: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    dotenv::from_filename("./click-tracker/.env").ok();

    let args = Args::parse();

    let settings = Settings::new(
        Some(args.run_mode.as_str()),
        Some(args.config_path.as_str()),
    )
    .unwrap();

    let mut app = AppBuilder::new(settings)
        .with_aws()
        .await
        .with_fluvio()
        .await
        .with_moka()
        .with_defaults()
        .with_uaparser()
        .with_geo_ip()
        .with_redis()
        .with_tracking_defaults()
        .with_default_modules()
        .build()
        .unwrap();

    let cancel: CancellationToken = CancellationToken::new();

    let handler = app.start(cancel.clone());

    let mut signals = Signals::new(&[SIGINT])?;
    thread::spawn(move || {
        for sig in signals.forever() {
            cancel.cancel();
            println!("Received signal {:?}", sig);
        }
    });

    return handler.await;
}
