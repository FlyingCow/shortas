use anyhow::Result;
use chrono::{DateTime, Utc};

use super::{
    attribution::decide,
    store::{Batch, ConversionStore},
    RECORDS, WAITING,
};

const PAGE: u32 = 1_000;

/// Matches every waiting event once. Returns how many still wait. Safe to run
/// several times at once, and again after a failure.
pub async fn reconcile(store: &ConversionStore, now: DateTime<Utc>) -> Result<usize> {
    let mut after = String::new();
    let mut still_waiting = 0;
    loop {
        let page = store.waiting(&after, PAGE).await?;
        let Some(last) = page.last() else {
            break;
        };
        after = last.id.clone();
        let more = page.len() == PAGE as usize;

        let mut ids: Vec<&str> = page
            .iter()
            .filter_map(|w| w.record.as_ref()?.click_id.as_deref())
            .collect();
        ids.sort_unstable();
        ids.dedup();
        let clicks = store.clicks(&ids).await?;

        let mut batch = Batch::default();
        let mut outcomes = Vec::new();
        for waiting in &page {
            let decision = match &waiting.record {
                None => super::attribution::Decision::Wait,
                Some(record) => {
                    let click = record.click_id.as_ref().and_then(|id| clicks.get(id));
                    decide(record, click, waiting.stored_at, now)
                }
            };
            match batch.resolve(waiting, decision, now)? {
                "waiting" => still_waiting += 1,
                outcome => outcomes.push(outcome),
            }
        }
        store.insert(batch).await?;
        for outcome in outcomes {
            RECORDS.with_label_values(&[outcome]).inc();
        }
        if !more {
            break;
        }
    }
    WAITING.set(still_waiting as i64);
    Ok(still_waiting)
}
