use std::net::SocketAddr;
use std::num::NonZeroUsize;

use crate::adapters::spi::aws::app_builder_extension::AwsBuilder;
use crate::adapters::spi::hyper::app_builder_extension::HyperBuilder;
use crate::adapters::spi::moka::app_builder_extension::MokaBuilder;
use crate::core::default::app_builder_extension::DefaultsBuilder;
use crate::core::{BaseConnectionHandler, BaseTlsConnectionHandler};

use crate::server::{Server, ServerOptions, Socket};
use crate::settings::Settings;

pub struct ServerBuilder<C, T>
where
    C: BaseConnectionHandler + Send + Clone + 'static,
    T: BaseTlsConnectionHandler + Send + Clone + 'static,
{
    threads: usize,
    listen_os_signals: bool,
    exit: bool,
    sockets: Vec<Socket>,
    handler: C,
    tls_handler: T,
}

impl<C, T> ServerBuilder<C, T>
where
    C: BaseConnectionHandler + Send + Clone + 'static,
    T: BaseTlsConnectionHandler + Send + Clone + 'static,
{
    pub fn new(handler: C, tls_handler: T) -> Self {
        ServerBuilder {
            threads: std::thread::available_parallelism().map_or(2, NonZeroUsize::get),
            listen_os_signals: true,
            exit: false,
            sockets: Vec::new(),
            handler: handler,
            tls_handler: tls_handler,
        }
    }

    pub fn build(self) -> Server<C, T> {
        if self.sockets.is_empty() {
            panic!("Server should have at least one bound socket");
        }

        Server::new(
            self.sockets,
            self.handler,
            self.tls_handler,
            ServerOptions {
                threads: self.threads,
                exit: self.exit,
                listen_os_signals: self.listen_os_signals,
            },
        )
    }
}

pub struct AppBuilder
{    
    threads: usize,
    listen_os_signals: bool,
    exit: bool,
    sockets: Vec<Socket>,
}

impl AppBuilder
where
{
    pub fn bind(mut self, address: SocketAddr) -> Self {
        self.sockets.push(Socket::Unsecured(address));
        self
    }

    pub fn bind_tls(mut self, address: SocketAddr) -> Self {
        self.sockets.push(Socket::Secured(address));
        self
    }

    pub fn workers(mut self, num: usize) -> Self {
        assert_ne!(num, 0, "workers must be greater than 0");
        self.threads = num;
        self
    }

    /// Disable OS signal handling.
    pub fn disable_signals(mut self) -> Self {
        self.listen_os_signals = false;
        self
    }

    pub fn system_exit(mut self) -> Self {
        self.exit = true;
        self
    }

    pub fn default(settings: Settings) -> Self {
        AppBuilder{
            threads: std::thread::available_parallelism().map_or(2, NonZeroUsize::get),
            listen_os_signals: true,
            exit: false,
            sockets: Vec::new(),
        }
    } 
    
    pub async fn run(self) {
        let settings = Settings::new().unwrap();

        let aws_builder = AwsBuilder::new(settings.aws).await;
        let moka_builder = MokaBuilder::new(settings.moka).await;
        let defaults_builder =
            DefaultsBuilder::new(aws_builder.dynamo.crypto_store, moka_builder.crypto_cache).await;

        let hyper = HyperBuilder::new(defaults_builder.crypto_manager).await;

        let mut  server_builder = ServerBuilder::new(hyper.unsecure_handler, hyper.secure_handler);

        server_builder.exit = self.exit;
        server_builder.sockets = self.sockets;
        server_builder.threads = self.threads;
        server_builder.listen_os_signals = self.listen_os_signals;

        server_builder.build().run().await;
    }
}
