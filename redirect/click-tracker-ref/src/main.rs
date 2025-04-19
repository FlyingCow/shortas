use anyhow::Result;

use clap::Parser;
use click_tracker_ref::{
    App, FluvioHitStream, KafkaHitStream, Settings,
    adapters::{
        ClickAggsRegistrarType, HitStreamSourceType, fluvio::FluvioClickAggsRegistrar,
        kafka::KafkaClickAggsRegistrar,
    },
    core::{
        pipe::modules::clicks::{ClickModules, aggregate::AggregateModule, init::InitModule},
        tracking_pipe::TrackingPipe,
    },
};

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    #[arg(short, long, default_value_t = String::from("production"), env("APP_RUN_MODE"))]
    pub run_mode: String,
    #[arg(short, long, default_value_t = String::from("./config"), env("APP_CONFIG_PATH"))]
    pub config_path: String,
}

async fn init_modules(settings: Settings) -> Vec<ClickModules> {
    let init = InitModule;
    let aggregate = AggregateModule::new(ClickAggsRegistrarType::Fluvio(
        FluvioClickAggsRegistrar::new(settings.fluvio.click_aggs).await,
    ));

    vec![ClickModules::Init(init), ClickModules::Aggregate(aggregate)]
}

fn init_sources(settings: Settings) -> Vec<HitStreamSourceType> {
    let kafka_stream = KafkaHitStream;
    let fluvio_stream = FluvioHitStream {
        settings: settings.fluvio.hit_stream,
    };
    vec![
        HitStreamSourceType::Fluvio(fluvio_stream),
        HitStreamSourceType::Kafka(kafka_stream),
    ]
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

    let pipe = TrackingPipe::builder()
        .with_stream_sources(init_sources(settings.clone()))
        .with_modules(init_modules(settings.clone()).await)
        .build();

    let app = App::builder().with_pipe(pipe).build();

    //starting the app
    let handler = app.run().await?;

    //waiting for the app to finish
    handler.await?;

    Ok(())
}
