use std::pin::Pin;
use std::task::{Context, Poll};
use std::{convert::Infallible, net::SocketAddr};
use std::sync::Arc;

use futures_util::future::join_all;
use futures_util::Future;
use http::{Request, Response, StatusCode};

use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex
};
use tower::Service;

use crate::core::BaseConnectionHandler;
use crate::core::BaseTlsConnectionHandler;

#[derive(Clone, Copy)]
pub enum Socket {
    Unsecured(SocketAddr),
    Secured(SocketAddr),
}

pub struct ServerOptions {
    pub threads: usize,
    pub listen_os_signals: bool,
    pub exit: bool,
}

pub struct Server<C, T>
where
    C: BaseConnectionHandler + Send + Clone + 'static,
    T: BaseTlsConnectionHandler + Send + Clone + 'static,
{
    handler: C,
    tls_handler: T,
    threads: usize,
    listen_os_signals: bool,
    exit: bool,
    sockets: Vec<Socket>,
}

impl<C, T> Server<C, T>
where
    C: BaseConnectionHandler + Send + Sync + Clone,
    T: BaseTlsConnectionHandler + Send + Sync + Clone,
{
    pub fn new(sockets: Vec<Socket>, unsecure_handler: C, secure_handler: T, options: ServerOptions) -> Self {
        Self {
            handler: unsecure_handler,
            tls_handler: secure_handler,
            threads: options.threads,
            listen_os_signals: options.listen_os_signals,
            exit: options.exit,
            sockets: sockets,
        }
    }

    fn get_unsecure_handler(&self) -> Arc<C> {
        Arc::new(self.handler.clone())
    }

    fn get_secure_handler(&self) -> Arc<T> {
        Arc::new(self.tls_handler.clone())
    }

    async fn listen_unsecured(&self, addr: SocketAddr) {
        let listener = TcpListener::bind(addr).await.unwrap();
        let unsecure_handler = self.get_unsecure_handler();

        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let unsecure_handler = unsecure_handler.clone();

            tokio::spawn(async move {
                let handler = unsecure_handler;
                handler.handle(stream).await;
            });
        }
    }

    async fn listen_secured(&self, addr: SocketAddr) {
        let listener = TcpListener::bind(addr).await.unwrap();
        let secure_handler = self.get_secure_handler();

        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let secure_handler = secure_handler.clone();

            tokio::spawn(async move {
                let handler = secure_handler;
                handler.handle(stream).await
            });
        }
    }
    
    async fn listen(&self, socket: Socket){
        match socket {
            Socket::Secured(addr) => self.listen_secured(addr).await,
            Socket::Unsecured(addr) => self.listen_unsecured(addr).await,
        }
    }

    pub async fn run(&self) {

        let futures = self.sockets.iter().map(|s| self.listen(s.clone()));

        join_all(futures).await;
    }

}