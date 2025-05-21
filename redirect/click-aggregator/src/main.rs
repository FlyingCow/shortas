use std::time::Duration;

use anyhow::{Ok, Result};

use clap::Parser;
use click_aggregator::adapters::clickhouse::ClickhouseClickStreamStore;
use click_aggregator::adapters::ClickStreamStoreType;
use click_aggregator::core::aggs_pipe::AggsPipe;
use click_aggregator::core::pipe::modules::clicks::store::StoreModule;
use click_aggregator::core::pipe::modules::clicks::AggsModules;
use click_aggregator::{
    adapters::ClickStreamSourceType, core::pipe::modules::clicks::init::InitModule, App,
    FluvioHitStream, KafkaHitStream, Settings,
};
use tokio_graceful_shutdown::{SubsystemBuilder, SubsystemHandle, Toplevel};
use tokio_util::sync::CancellationToken;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    #[arg(short, long, default_value_t = String::from("production"), env("APP_RUN_MODE"))]
    pub run_mode: String,
    #[arg(short, long, default_value_t = String::from("./config"), env("APP_CONFIG_PATH"))]
    pub config_path: String,
}

async fn init_modules(settings: &Settings, token: CancellationToken) -> Vec<AggsModules> {
    let init = InitModule;
    let store = StoreModule::new(ClickStreamStoreType::Clickhouse(
        ClickhouseClickStreamStore::new(settings.clickhouse.click_stream_store.clone(), token)
            .await
            .expect("Can not load clickhouse click store"),
    ));

    vec![AggsModules::Init(init), AggsModules::Store(store)]
}

fn init_sources(settings: Settings) -> Vec<ClickStreamSourceType> {
    let kafka_stream = KafkaHitStream;
    let fluvio_stream = FluvioHitStream::new(settings.fluvio.click_stream);
    vec![
        ClickStreamSourceType::Fluvio(fluvio_stream),
        ClickStreamSourceType::Kafka(kafka_stream),
    ]
}

async fn start(token: CancellationToken) -> Result<()> {
    let args = Args::parse();

    let settings = Settings::new(
        Some(args.run_mode.as_str()),
        Some(args.config_path.as_str()),
    )
    .expect("Can not load settings toml.");

    let modules = init_modules(&settings, token.clone()).await;

    let pipe = AggsPipe::builder()
        .with_stream_sources(init_sources(settings))
        .with_modules(modules)
        .build();

    let app = App::builder().with_pipe(pipe).build();

    //starting the app
    let handler = app.run(token).await?;

    handler.await.map_err(anyhow::Error::msg)?;

    Ok(())
}

async fn aggregating_subsystem(subsys: SubsystemHandle) -> Result<()> {
    let token: CancellationToken = CancellationToken::new();

    tokio::select! {
        _ = subsys.on_shutdown_requested() => {
            tracing::info!("Aggregating cancelled.");
            token.cancel();
        },
        _ = start(token.clone()) => {
            subsys.request_shutdown();
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    dotenv::from_filename("./click-aggregator/.env").ok();

    // Setup and execute subsystem tree
    Toplevel::new(|s| async move {
        s.start(SubsystemBuilder::new("Aggregating", aggregating_subsystem));
    })
    .catch_signals()
    .handle_shutdown_requests(Duration::from_millis(1000))
    .await
    .map_err(Into::into)
}
