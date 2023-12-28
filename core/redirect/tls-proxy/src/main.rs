use tls_proxy::dynamodb::DynamoCryptoStore;
use tls_proxy::log::*;
use tls_proxy::moka::MokaCryptoCache;
use tls_proxy::CryptoManager;
use tls_proxy::{Keycert, Server};
use tracing::{/*debug, error, */ info, warn};

#[tokio::main]
async fn main() {
    init_logger();

    warn!("Starting Proxy");
    let _ = Keycert::new().key_from_path("./key.pem".to_string());

    let dynamo_crypto_store = DynamoCryptoStore::new();
    let moka_crypto_cache = MokaCryptoCache::new();
    let crypto_manager = CryptoManager::new(&dynamo_crypto_store, &moka_crypto_cache);
    let _server = Server::new(&crypto_manager);

    _server.run().await;
    _server.run_tls().await;

    info!("Exiting");
}
