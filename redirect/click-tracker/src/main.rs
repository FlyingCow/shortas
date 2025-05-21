use std::time::Duration;

use anyhow::{Ok, Result};

use clap::Parser;
use click_tracker::{
    App, FluvioHitStream, KafkaHitStream, Settings,
    adapters::{
        ClickAggsRegistrarType, HitStreamSourceType, LocationDetectorType, SessionDetectorType,
        UserAgentDetectorType, fluvio::FluvioClickAggsRegistrar,
        geo_ip::geo_ip_location_detector::GeoIPLocationDetector,
        redis::session_detector::RedisSessionDetector,
        uaparser::user_agent_detector::UAParserUserAgentDetector,
    },
    core::{
        pipe::modules::clicks::{
            ClickModules, aggregate::AggregateModule, init::InitModule,
            location::EnrichLocationModule, session::EnrichSessionModule,
            user_agent::EnrichUserAgentModule,
        },
        tracking_pipe::TrackingPipe,
    },
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

async fn init_modules(settings: &Settings, token: CancellationToken) -> Vec<ClickModules> {
    let init = InitModule;

    let aggregate = AggregateModule::new(ClickAggsRegistrarType::Fluvio(
        FluvioClickAggsRegistrar::new(&settings.fluvio.click_aggs, token).await,
    ));

    let location = EnrichLocationModule::new(LocationDetectorType::GeoIP(
        GeoIPLocationDetector::new(&settings.geo_ip),
    ));

    let session = EnrichSessionModule::new(SessionDetectorType::Redis(RedisSessionDetector::new(
        &settings.redis,
    )));

    let user_agent = EnrichUserAgentModule::new(UserAgentDetectorType::UAParser(
        UAParserUserAgentDetector::new(settings.uaparser.yaml.as_str()),
    ));

    vec![
        ClickModules::Init(init),
        ClickModules::Location(location),
        ClickModules::Session(session),
        ClickModules::UserAgent(user_agent),
        ClickModules::Aggregate(aggregate),
    ]
}

fn init_sources(settings: Settings) -> Vec<HitStreamSourceType> {
    let kafka_stream = KafkaHitStream;
    let fluvio_stream = FluvioHitStream::new(settings.fluvio.hit_stream);
    vec![
        HitStreamSourceType::Fluvio(fluvio_stream),
        HitStreamSourceType::Kafka(kafka_stream),
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

    let pipe = TrackingPipe::builder()
        .with_stream_sources(init_sources(settings))
        .with_modules(modules)
        .build();

    let app = App::builder().with_pipe(pipe).build();

    //starting the app
    let handler = app.run(token).await?;

    handler.await.map_err(anyhow::Error::msg)?;

    Ok(())
}

async fn tracking_subsystem(subsys: SubsystemHandle) -> Result<()> {
    let token: CancellationToken = CancellationToken::new();

    tokio::select! {
        _ = subsys.on_shutdown_requested() => {
            tracing::info!("Tracking cancelled.");
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

    dotenv::from_filename("./click-tracker/.env").ok();

    // Setup and execute subsystem tree
    Toplevel::new(|s| async move {
        s.start(SubsystemBuilder::new("Tracking", tracking_subsystem));
    })
    .catch_signals()
    .handle_shutdown_requests(Duration::from_millis(1000))
    .await
    .map_err(Into::into)
}
