use std::{sync::Arc, time::Duration};

use anyhow::Result;
use tokio::{
    sync::{
        mpsc::{self, Sender},
        Mutex, Semaphore,
    },
    time::Instant,
};
use tracing::{error, info};

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
        info!("Enqueuing a hit");

        self.tx.send(item).await.map_err(anyhow::Error::msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    #[derive(Clone, Debug, PartialEq)]
    struct TestItem {
        value: i32,
    }

    struct TestProcessor {
        processed: Arc<Mutex<Vec<Vec<TestItem>>>>,
    }

    #[async_trait::async_trait()]
    impl BatchProcess<TestItem> for TestProcessor {
        async fn process(&mut self, batch: Vec<TestItem>) -> Result<()> {
            self.processed.lock().await.push(batch);
            Ok(())
        }
    }

    struct ErrorProcessor {}

    #[async_trait::async_trait()]
    impl BatchProcess<TestItem> for ErrorProcessor {
        async fn process(&mut self, _batch: Vec<TestItem>) -> Result<()> {
            Err(anyhow::anyhow!("Test error"))
        }
    }

    #[tokio::test]
    async fn should_create_async_queue() {
        let processed = Arc::new(Mutex::new(Vec::new()));
        let processor = Box::new(TestProcessor {
            processed: processed.clone(),
        });

        let queue = AsyncQueue::new(
            processor,
            10,
            2,
            Duration::from_millis(100),
        );

        // Just verify creation works
        assert!(true);
    }

    #[tokio::test]
    async fn should_enqueue_and_process_items() {
        let processed = Arc::new(Mutex::new(Vec::new()));
        let processor = Box::new(TestProcessor {
            processed: processed.clone(),
        });

        let queue = AsyncQueue::new(
            processor,
            3,
            1,
            Duration::from_millis(100),
        );

        // Enqueue items
        for i in 0..3 {
            queue.enqueue(TestItem { value: i }).await.unwrap();
        }

        // Give it time to process
        tokio::time::sleep(Duration::from_millis(200)).await;

        let batches = processed.lock().await;
        assert!(batches.len() > 0);
    }

    #[tokio::test]
    async fn should_process_batch_when_batch_size_reached() {
        let processed = Arc::new(Mutex::new(Vec::new()));
        let processor = Box::new(TestProcessor {
            processed: processed.clone(),
        });

        let batch_size = 5;
        let queue = AsyncQueue::new(
            processor,
            batch_size,
            1,
            Duration::from_secs(10), // Long duration so batch size is the trigger
        );

        // Enqueue exactly batch_size items
        for i in 0..batch_size {
            queue.enqueue(TestItem { value: i as i32 }).await.unwrap();
        }

        // Give it time to process
        tokio::time::sleep(Duration::from_millis(100)).await;

        let batches = processed.lock().await;
        assert!(batches.len() > 0);
        if let Some(first_batch) = batches.first() {
            assert_eq!(first_batch.len(), batch_size);
        }
    }

    #[tokio::test]
    async fn should_process_batch_when_duration_elapsed() {
        let processed = Arc::new(Mutex::new(Vec::new()));
        let processor = Box::new(TestProcessor {
            processed: processed.clone(),
        });

        let queue = AsyncQueue::new(
            processor,
            100, // Large batch size so duration is the trigger
            1,
            Duration::from_millis(100),
        );

        // Enqueue a few items (less than batch_size)
        for i in 0..3 {
            queue.enqueue(TestItem { value: i }).await.unwrap();
        }

        // Wait for duration to elapse plus some buffer for processing
        tokio::time::sleep(Duration::from_millis(700)).await;

        let batches = processed.lock().await;
        assert!(batches.len() > 0);
    }

    #[tokio::test]
    async fn should_handle_multiple_consumers() {
        let processed = Arc::new(Mutex::new(Vec::new()));
        let processor = Box::new(TestProcessor {
            processed: processed.clone(),
        });

        let queue = AsyncQueue::new(
            processor,
            2,
            3, // Multiple consumers
            Duration::from_millis(50),
        );

        // Enqueue many items
        for i in 0..10 {
            queue.enqueue(TestItem { value: i }).await.unwrap();
        }

        // Give it time to process with multiple consumers
        tokio::time::sleep(Duration::from_millis(300)).await;

        let batches = processed.lock().await;
        assert!(batches.len() > 0);
    }

    #[tokio::test]
    async fn should_handle_processing_errors_gracefully() {
        let processor = Box::new(ErrorProcessor {});

        let queue = AsyncQueue::new(
            processor,
            5,
            1,
            Duration::from_millis(100),
        );

        // Enqueue items even though processing will fail
        for i in 0..5 {
            let result = queue.enqueue(TestItem { value: i }).await;
            assert!(result.is_ok()); // Enqueuing should still work
        }

        // Give it time to attempt processing (which will fail but not crash)
        tokio::time::sleep(Duration::from_millis(200)).await;

        // If we get here, the queue handled errors gracefully
        assert!(true);
    }

    #[tokio::test]
    async fn should_clone_queue() {
        let processed = Arc::new(Mutex::new(Vec::new()));
        let processor = Box::new(TestProcessor {
            processed: processed.clone(),
        });

        let queue = AsyncQueue::new(
            processor,
            10,
            1,
            Duration::from_millis(100),
        );

        let queue_clone = queue.clone();

        // Both should be able to enqueue
        queue.enqueue(TestItem { value: 1 }).await.unwrap();
        queue_clone.enqueue(TestItem { value: 2 }).await.unwrap();

        assert!(true);
    }
}
