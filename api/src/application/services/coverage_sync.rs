//! Sends conversion coverage to the router in the background, and marks
//! activations finished once their routes have been sent.
//!
//! The work is the routes whose coverage differs from what the router was last
//! sent, so it survives restarts and outages. Writes that change coverage
//! wake the sync; otherwise it checks now and then, which retries after an
//! outage and picks up changes made through another instance.

use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{broadcast, Notify};
use tracing::warn;
use uuid::Uuid;

use super::{CoverageProgress, RouteService};
use crate::domain::traits::ConversionRepository;

/// Checks this often when not woken.
const IDLE_CHECK: Duration = Duration::from_secs(60);
/// Waits this long after the router failed.
const RETRY_AFTER: Duration = Duration::from_secs(10);

/// Wakes the coverage sync. Cheap to clone. Wakes are never lost: one that
/// comes while the sync is running makes it run again, and several then count
/// as one.
#[derive(Clone, Default)]
pub struct CoverageSignal(Arc<Notify>);

impl CoverageSignal {
    /// Coverage may have changed.
    pub fn notify(&self) {
        self.0.notify_one();
    }
}

pub struct CoverageSync {
    routes: Arc<RouteService>,
    conversions: Arc<dyn ConversionRepository>,
    signal: CoverageSignal,
}

impl CoverageSync {
    pub fn new(
        routes: Arc<RouteService>,
        conversions: Arc<dyn ConversionRepository>,
        signal: CoverageSignal,
    ) -> Self {
        Self { routes, conversions, signal }
    }

    /// Runs until shutdown.
    pub async fn run(&self, mut shutdown: broadcast::Receiver<()>) {
        let mut after = Uuid::nil();
        loop {
            let wait = match self.routes.send_pending_coverage(after).await {
                Ok(CoverageProgress::More(next)) => {
                    after = next;
                    None
                }
                // Routes before the starting point may be waiting too.
                Ok(CoverageProgress::Done) if !after.is_nil() => {
                    after = Uuid::nil();
                    None
                }
                Ok(CoverageProgress::Done) => Some(IDLE_CHECK),
                Ok(CoverageProgress::Stalled(next)) => {
                    after = next;
                    Some(RETRY_AFTER)
                }
                Err(e) => {
                    warn!("Failed to send conversion coverage: {}", e);
                    Some(RETRY_AFTER)
                }
            };
            if let Err(e) = self.conversions.finish_activations().await {
                warn!("Failed to finish conversion activations: {}", e);
            }

            match wait {
                None => {
                    if !matches!(shutdown.try_recv(), Err(broadcast::error::TryRecvError::Empty)) {
                        break;
                    }
                }
                Some(wait) => {
                    tokio::select! {
                        _ = shutdown.recv() => break,
                        _ = self.signal.0.notified() => {}
                        _ = tokio::time::sleep(wait) => {}
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::traits::{
        MockConversionRepository, MockDomainRepository, MockOutboxRepository,
        MockRouteRepository, MockWorkspaceRepository,
    };
    use crate::infrastructure::http_clients::ClickRouterClient;
    use crate::settings::ClickRouterSettings;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;

    /// Starts a sync whose route lookups answer from `pages` in turn, then
    /// with nothing. Returns the lookups made and how often activations
    /// were checked, as they happen.
    fn start(
        pages: Vec<Vec<Uuid>>,
    ) -> (CoverageSignal, broadcast::Sender<()>, Arc<Mutex<Vec<Uuid>>>, Arc<AtomicUsize>) {
        let lookups = Arc::new(Mutex::new(Vec::new()));
        let finished = Arc::new(AtomicUsize::new(0));

        let mut route_repo = MockRouteRepository::new();
        let pages = Mutex::new(pages.into_iter());
        let l = lookups.clone();
        route_repo.expect_pending_coverage().returning(move |after, _| {
            l.lock().unwrap().push(after);
            Ok(pages.lock().unwrap().next().unwrap_or_default())
        });
        // Routes without a domain are only marked sent.
        route_repo.expect_get_by_id().returning(|_| Ok(None));
        let routes = Arc::new(RouteService::new(
            Arc::new(route_repo),
            Arc::new(MockDomainRepository::new()),
            Arc::new(MockOutboxRepository::new()),
            Arc::new(
                ClickRouterClient::new(&ClickRouterSettings {
                    base_url: "http://127.0.0.1:1".into(),
                    timeout_ms: 100,
                })
                .unwrap(),
            ),
            Arc::new(MockWorkspaceRepository::new()),
        ));
        let mut conversions = MockConversionRepository::new();
        let f = finished.clone();
        conversions.expect_finish_activations().returning(move || {
            f.fetch_add(1, Ordering::SeqCst);
            Ok(0)
        });

        let signal = CoverageSignal::default();
        let sync = CoverageSync::new(routes, Arc::new(conversions), signal.clone());
        let (shutdown, rx) = broadcast::channel(1);
        tokio::spawn(async move { sync.run(rx).await });
        (signal, shutdown, lookups, finished)
    }

    async fn settle() {
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    #[tokio::test]
    async fn runs_at_start_then_only_when_woken() {
        let (signal, shutdown, lookups, finished) = start(vec![]);
        settle().await;
        assert_eq!(lookups.lock().unwrap().len(), 1);
        assert_eq!(finished.load(Ordering::SeqCst), 1);

        settle().await;
        assert_eq!(lookups.lock().unwrap().len(), 1, "idle until woken");

        signal.notify();
        settle().await;
        assert_eq!(lookups.lock().unwrap().len(), 2);
        assert_eq!(finished.load(Ordering::SeqCst), 2);

        for _ in 0..5 {
            signal.notify();
        }
        settle().await;
        let runs = lookups.lock().unwrap().len();
        assert!((3..=4).contains(&runs), "wakes in a row run once or twice, not {runs}");
        settle().await;
        assert_eq!(lookups.lock().unwrap().len(), runs, "idle again");

        shutdown.send(()).unwrap();
    }

    /// Then it checks from the start once more, for routes changed meanwhile.
    #[tokio::test]
    async fn full_pages_continue_without_waiting() {
        let mut page: Vec<Uuid> = (0..200).map(|_| Uuid::new_v4()).collect();
        page.sort();
        let last = *page.last().unwrap();
        let (_signal, shutdown, lookups, _) = start(vec![page]);
        settle().await;
        assert_eq!(*lookups.lock().unwrap(), vec![Uuid::nil(), last, Uuid::nil()]);
        shutdown.send(()).unwrap();
    }

    #[tokio::test]
    async fn stops_on_shutdown() {
        let (signal, shutdown, lookups, _) = start(vec![]);
        settle().await;
        shutdown.send(()).unwrap();
        settle().await;
        signal.notify();
        settle().await;
        assert_eq!(lookups.lock().unwrap().len(), 1);
    }
}
