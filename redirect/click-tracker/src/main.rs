use aws_sdk_kinesis as kinesis;

#[::tokio::main]
async fn main() -> Result<(), kinesis::Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_kinesis::Client::new(&config);

    // ... make some calls with the client

    Ok(())
}