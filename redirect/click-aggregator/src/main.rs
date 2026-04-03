use std::time::Duration;

use anyhow::Result;
use std::result::Result::Ok;

use clap::Parser;
use click_aggregator::adapters::clickhouse::ClickhouseClickStreamStore;
use click_aggregator::adapters::ClickStreamStoreType;
use click_aggregator::core::aggs_pipe::AggsPipe;
use click_aggregator::core::metrics_server::start_metrics_server;
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

async fn init_sources(settings: Settings) -> Vec<ClickStreamSourceType> {
    let kafka_stream = KafkaHitStream;
    let fluvio_stream = FluvioHitStream::connect(settings.fluvio.click_stream).await;
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

    let pipe = AggsPipe::new(modules);

    let mut threads = App::run(init_sources(settings).await, pipe, token)
        .await
        .expect("Could not run app");

    while let Some(res) = threads.join_next().await {
        match res {
            Ok(_) => {
                tracing::info!("Task finished");
            }
            Err(err) => {
                tracing::error!("Task failed: {:?}", err);
                // Handle the error appropriately
            }
        }
    }

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

async fn metrics_subsystem(subsys: SubsystemHandle) -> Result<()> {
    let token: CancellationToken = CancellationToken::new();

    // Metrics server port - can be configured via env var
    let port = std::env::var("METRICS_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(9090);

    tokio::select! {
        _ = subsys.on_shutdown_requested() => {
            tracing::info!("Metrics server cancelled.");
            token.cancel();
        },
        result = start_metrics_server(port, token.clone()) => {
            if let Err(e) = result {
                tracing::error!("Metrics server error: {}", e);
            }
            subsys.request_shutdown();
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

    dotenv::from_filename("./click-aggregator/.env").ok();

    // Initialize tracing with Loki integration
    let loki_url = std::env::var("LOKI_URL").unwrap_or_else(|_| "http://loki:3100".to_string());

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("warn"));

    let loki_result = url::Url::parse(&loki_url)
        .ok()
        .and_then(|url| {
            tracing_loki::builder()
                .label("service", "click-aggregator")
                .ok()
                .and_then(|b| b.build_url(url).ok())
        });

    match loki_result {
        Some((loki_layer, loki_task)) => {
            tracing_subscriber::registry()
                .with(filter)
                .with(tracing_subscriber::fmt::layer())
                .with(loki_layer)
                .init();
            tokio::spawn(loki_task);
        }
        None => {
            tracing_subscriber::registry()
                .with(filter)
                .with(tracing_subscriber::fmt::layer())
                .init();
            eprintln!("click-aggregator: Failed to initialize Loki logging");
        }
    }

    // Setup and execute subsystem tree
    Toplevel::new(|s| async move {
        s.start(SubsystemBuilder::new("Aggregating", aggregating_subsystem));
        s.start(SubsystemBuilder::new("Metrics", metrics_subsystem));
    })
    .catch_signals()
    .handle_shutdown_requests(Duration::from_millis(1000))
    .await
    .map_err(Into::into)
}
