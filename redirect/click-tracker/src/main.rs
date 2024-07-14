use anyhow::Result;

use clap::Parser;
use click_tracker::{
    core::tracking_pipe::BaseTrackingPipe, settings::Settings,
    tracking_pipe::default_tracking_pipe::DefaultTrackingPipe, AppBuilder,
};
use once_cell::sync::OnceCell;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    #[arg(short, long, default_value_t = String::from("production"), env("APP_RUN_MODE"))]
    pub run_mode: String,
    #[arg(short, long, default_value_t = String::from("./config"), env("APP_CONFIG_PATH"))]
    pub config_path: String,
}

static TRACKING_PIPE: OnceCell<DefaultTrackingPipe> = OnceCell::new();

#[inline]
pub fn get_trackin_pipe() -> &'static DefaultTrackingPipe {
    TRACKING_PIPE.get().unwrap()
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

    let app = AppBuilder::new(settings)
        .with_aws()
        .await
        .with_moka()
        .with_defaults()
        .with_uaparser()
        .with_geo_ip()
        .with_tracking_defaults()
        .with_default_modules()
        .build()
        .unwrap();

    app.start()
    .await?;

    let _ = TRACKING_PIPE.set(app);

    return Ok(());
}
