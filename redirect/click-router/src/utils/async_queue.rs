use std::{sync::Arc, time::Duration};

use anyhow::Result;
use tokio::{
    sync::{
        mpsc::{self, Sender},
        Mutex, Semaphore,
    },
    time::Instant,
};
use tracing::{error, warn};

const IDLE_TIMEOUT: u64 = 500;

#[async_trait::async_trait()]
pub trait BatchProcess<T>: Send + Sync {
    async fn process(&mut self, batch: Vec<T>) -> Result<()>;
}

#[derive(Clone, Debug)]
pub struct AsyncQueue<T: 'static> {
    tx: Sender<T>,
}

impl<T: Send + Sync> AsyncQueue<T> {
    pub fn new(
        processor: Box<dyn BatchProcess<T> + Send + Sync>,
        batch_size: usize,
        consumers: usize,
        duration: Duration,
    ) -> Self {
        let (tx, mut rx) = mpsc::channel((consumers) * batch_size);
        let processor = Arc::new(Mutex::new(processor));

        let _join_handle = tokio::spawn(async move {
            let permits = Arc::new(Semaphore::new(consumers));
            let mut batch = Vec::with_capacity(consumers * batch_size);
            let sleep = tokio::time::sleep(duration);

            tokio::pin!(sleep);
            sleep.as_mut().reset(Instant::now() + duration);

            loop {
                let recv_res = rx.try_recv();

                if let Some(message) = recv_res.ok() {
                    batch.push(message);
                } else {
                    if batch.len() > 0 { 
                        tokio::time::sleep(Duration::from_millis(IDLE_TIMEOUT)).await;
                    }
                    else  {
                        let message = rx.recv().await.unwrap();
                        batch.push(message);
                    }
                }

                if batch.len() >= batch_size || (batch.len() > 0 && sleep.is_elapsed()) {
                    let mut drain_count = batch_size;

                    if sleep.is_elapsed() {
                        drain_count = batch.len();
                    }

                    let items = batch.drain(..drain_count).collect();

                    let processor = processor.clone();
                    let permits = permits.clone();

                    let permit = permits.acquire_owned().await.unwrap();
                    tokio::spawn(async move {
                        let result = processor.lock().await.process(items).await;

                        if let Err(error) = result {
                            error!("{}", error);
                        }

                        drop(permit);
                    });

                    sleep.as_mut().reset(Instant::now() + duration);
                }
            }
        });

        Self { tx }
    }

    pub async fn enqueue(&self, item: T) -> Result<()> {
        warn!("enqueue");

        self.tx.send(item).await.map_err(anyhow::Error::msg)
    }
}
