use anyhow::Result;

use click_tracker_ref::{App, FluvioHitStream, KafkaHitStream, adapters::HitStreamSourceType};

#[tokio::main]
async fn main() -> Result<()> {
    let kafka_stream = KafkaHitStream;
    let fluvio_stream = FluvioHitStream;

    let app = App::builder()
        .with_stream_sources(vec![
            HitStreamSourceType::Fluvio(fluvio_stream),
            HitStreamSourceType::Kafka(kafka_stream),
        ])
        .build();

    //starting the app
    let handler = app.run().await?;

    //waiting for the app to finish
    handler.await?;

    Ok(())
}
