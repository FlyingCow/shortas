//! Matches pending events to their clicks: waiting events every few seconds,
//! late ones less often.

use anyhow::Result;
use chrono::{DateTime, Utc};

use super::{
    attribution::{decide, may_name_a_click, Click, Decision, CLICK_NOT_FOUND, LATE_HORIZON},
    store::{state, Batch, ConversionStore, Pending},
    LATE, RECORDS, WAITING,
};

const PAGE: u32 = 1_000;

/// What becomes of one pending event: `None` while it stays as it is,
/// otherwise the outcome to count.
pub type Step =
    fn(&Pending, Option<&Click>, DateTime<Utc>, &mut Batch) -> Result<Option<&'static str>>;

/// Matches every waiting event once. Returns how many still wait. Safe to run
/// several times at once, and again after a failure.
pub async fn reconcile(store: &ConversionStore, now: DateTime<Utc>) -> Result<usize> {
    let waiting = run(store, state::WAITING, now, waiting_step).await?;
    WAITING.set(waiting as i64);
    Ok(waiting)
}

/// Re-checks every late event once. Returns how many are still late.
pub async fn recheck_late(store: &ConversionStore, now: DateTime<Utc>) -> Result<usize> {
    let late = run(store, state::LATE, now, late_step).await?;
    LATE.set(late as i64);
    Ok(late)
}

async fn run(store: &ConversionStore, of: u8, now: DateTime<Utc>, step: Step) -> Result<usize> {
    let mut after = String::new();
    let mut remaining = 0;
    loop {
        let page = store.pending(of, &after, PAGE).await?;
        let Some(last) = page.last() else {
            break;
        };
        after = last.id.clone();
        remaining += judge(store, &page, now, step).await?;
        if page.len() < PAGE as usize {
            break;
        }
    }
    Ok(remaining)
}

/// Applies `step` to `page` with the clicks it names. Returns how many stay.
pub async fn judge(
    store: &ConversionStore,
    page: &[Pending],
    now: DateTime<Utc>,
    step: Step,
) -> Result<usize> {
    let mut ids: Vec<&str> = page
        .iter()
        .filter_map(|p| p.record.as_ref()?.click_id.as_deref())
        .collect();
    ids.sort_unstable();
    ids.dedup();
    let clicks = store.clicks(&ids).await?;

    let mut batch = Batch::default();
    let mut outcomes = Vec::new();
    let mut remaining = 0;
    for pending in page {
        let click = pending
            .record
            .as_ref()
            .and_then(|r| clicks.get(r.click_id.as_ref()?));
        match step(pending, click, now, &mut batch)? {
            None => remaining += 1,
            Some(outcome) => outcomes.push(outcome),
        }
    }
    store.insert(batch).await?;
    for outcome in outcomes {
        RECORDS.with_label_values(&[outcome]).inc();
    }
    Ok(remaining)
}

/// A waiting event is decided once its click is stored, or when the wait is
/// over. An event whose click may still come then becomes late.
pub fn waiting_step(
    pending: &Pending,
    click: Option<&Click>,
    now: DateTime<Utc>,
    batch: &mut Batch,
) -> Result<Option<&'static str>> {
    let Some(record) = &pending.record else {
        batch.mark(pending, state::DONE, now);
        return Ok(Some("unreadable"));
    };
    let decision = decide(record, click, pending.stored_at, now);
    if decision == Decision::Wait {
        return Ok(None);
    }
    let late = decision == Decision::Unattributed(CLICK_NOT_FOUND) && may_name_a_click(record);
    let outcome = batch.decided(record, decision)?;
    batch.mark(pending, if late { state::LATE } else { state::DONE }, now);
    Ok(Some(outcome))
}

/// A late event becomes attributed when its click is found and eligible. Any
/// other verdict, or the end of the horizon, leaves it unattributed for good.
pub fn late_step(
    pending: &Pending,
    click: Option<&Click>,
    now: DateTime<Utc>,
    batch: &mut Batch,
) -> Result<Option<&'static str>> {
    let Some(record) = &pending.record else {
        batch.mark(pending, state::DONE, now);
        return Ok(Some("unreadable"));
    };
    let outcome = match click {
        Some(click) => match decide(record, Some(click), pending.stored_at, now) {
            decision @ Decision::Attributed(_) => {
                batch.decided(record, decision)?;
                "late_attributed"
            }
            _ => "late_ended",
        },
        None if now - record.received_at >= LATE_HORIZON => "late_ended",
        None => return Ok(None),
    };
    batch.mark(pending, state::DONE, now);
    Ok(Some(outcome))
}
