use server::*;

#[tokio::main]
async fn main() {

    env_logger::try_init().unwrap();

    let tls_handler = TlsConnectionHandler::new(
        CryptoManager::new(
            DynamoCryptoStore::new(), 
            InMemoryCryptoCache::new())
    );

    let handler = ConnectionHandler::new();

    ServerBuilder::new(handler, tls_handler)
        .bind(([127, 0, 0, 1], 1337).into())
        .bind(([127, 0, 0, 1], 1338).into())
        .bind_tls(([127, 0, 0, 1], 4434).into())
        .build()
        .run().await;

}
