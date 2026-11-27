//! Needs ClickHouse with the migrations applied:
//! `CONVERSIONS_CLICKHOUSE_URL`, `_USER`, `_PASSWORD` and `_DATABASE`.
//! Run with `cargo test -p click-aggregator -- --ignored`.

use std::{collections::HashMap, sync::Mutex};

use chrono::{DateTime, Duration, Utc};
use clickhouse::{Client, Compression};

use super::{
    attribution::LATE_HORIZON,
    consumer::store_batch,
    reconciler::{judge, late_step, recheck_late, reconcile, waiting_step, Step},
    record::{tests::record, ConversionRecord},
    store::{state, ConversionStore, Pending},
    RECORDS,
};
use crate::adapters::clickhouse::settings::ClickStreamStoreConfig;

struct Db {
    store: ConversionStore,
    client: Client,
    workspace: String,
    /// Click IDs by name: ULIDs, like the router's.
    clicks: Mutex<HashMap<String, String>>,
}

impl Db {
    fn new() -> Self {
        let var = |name: &str| std::env::var(format!("CONVERSIONS_CLICKHOUSE_{name}")).unwrap();
        let config = ClickStreamStoreConfig {
            url: var("URL"),
            max_rows: 1,
            period_millis: 1,
            period_bias: 0.0,
            database: var("DATABASE"),
            table: "click_stream".into(),
            user: Some(var("USER")),
            password: Some(var("PASSWORD")),
        };
        let client = Client::default()
            .with_compression(Compression::None)
            .with_url(&config.url)
            .with_database(&config.database)
            .with_user(var("USER"))
            .with_password(var("PASSWORD"));
        Self {
            store: ConversionStore::new(&config),
            client,
            workspace: ulid::Ulid::new().to_string(),
            clicks: Mutex::default(),
        }
    }

    /// A record of this test's workspace, with a click ID of its own.
    fn event(&self, click: &str) -> ConversionRecord {
        let mut r = record(&ulid::Ulid::new().to_string());
        r.workspace_id = self.workspace.clone();
        r.click_id = Some(self.click_id(click));
        r
    }

    fn click_id(&self, click: &str) -> String {
        let mut clicks = self.clicks.lock().unwrap();
        clicks
            .entry(click.into())
            .or_insert_with(|| ulid::Ulid::new().to_string())
            .clone()
    }

    async fn click(&self, click: &str, workspace: &str, created: DateTime<Utc>) {
        self.client
            .query(
                // The owner is this test's own: daily click rows are summed by
                // owner and route, so a shared owner would merge them away.
                "INSERT INTO click_stream (id, owner_id, workspace_id, route_id, created, country, \
                 device_family, os_family, user_agent_family, is_trackable) \
                 VALUES (?, ?, ?, 'route', fromUnixTimestamp64Milli(toInt64(?)), 'DE', 'Mac', 'Mac OS X', 'Safari', 1)",
            )
            .bind(self.click_id(click))
            .bind(&self.workspace)
            .bind(workspace)
            .bind(created.timestamp_millis())
            .execute()
            .await
            .unwrap();
    }

    async fn my_click(&self, click: &str) {
        self.click(click, &self.workspace, Utc::now() - Duration::hours(1)).await;
    }

    async fn store(&self, records: &[ConversionRecord]) {
        let payloads: Vec<_> = records.iter().map(|r| serde_json::to_vec(r).unwrap()).collect();
        store_batch(&self.store, payloads.iter().map(Vec::as_slice)).await.unwrap();
    }

    /// Two reconcilers at once. Other tests share the table, so several
    /// reconcilers run anyway.
    async fn reconcile(&self, now: DateTime<Utc>) {
        let (a, b) = tokio::join!(reconcile(&self.store, now), reconcile(&self.store, now));
        a.unwrap();
        b.unwrap();
    }

    async fn rows(&self, sql: &str) -> Vec<String> {
        self.client
            .query(sql)
            .bind(&self.workspace)
            .fetch_all()
            .await
            .unwrap()
    }

    async fn one(&self, sql: &str) -> String {
        self.rows(sql).await.join(",")
    }

    async fn events(&self) -> Vec<String> {
        self.rows(
            "SELECT concat(id, ' ', status, ' ', reason) FROM conversion_events FINAL \
             WHERE workspace_id = ? AND kind = 'event' ORDER BY id",
        )
        .await
    }

    async fn count(&self) -> usize {
        self.events().await.len()
    }

    async fn rejected(&self) -> String {
        self.one("SELECT reason FROM conversion_rejected FINAL WHERE workspace_id = ? ORDER BY id").await
    }

    async fn waiting(&self) -> String {
        self.in_state(state::WAITING).await
    }

    async fn late(&self) -> String {
        self.in_state(state::LATE).await
    }

    async fn in_state(&self, of: u8) -> String {
        self.one(&format!(
            "SELECT toString(count()) FROM conversion_pending FINAL \
             WHERE workspace_id = ? AND resolved = {of}"
        ))
        .await
    }

    /// Applies `step` to this event alone: the shared table holds other
    /// tests' events, which must not be judged at a made-up time.
    async fn judge_alone(&self, e: &ConversionRecord, of: u8, step: Step, now: DateTime<Utc>) {
        let pending = self.pending(e, of).await.expect("not pending");
        judge(&self.store, &[pending], now, step).await.unwrap();
    }

    async fn pending(&self, e: &ConversionRecord, of: u8) -> Option<Pending> {
        let mut after = String::new();
        loop {
            let page = self.store.pending(of, &after, 1000).await.unwrap();
            after = page.last()?.id.clone();
            if let Some(p) = page.into_iter().find(|p| p.id == e.id) {
                return Some(p);
            }
        }
    }

    /// The event's value and status, and its route when attributed.
    async fn event_row(&self) -> String {
        self.one(
            "SELECT concat(status, ' ', reason, ' ', route_id, ' ', ifNull(toString(value), '')) \
             FROM conversion_events FINAL WHERE workspace_id = ? AND kind = 'event'",
        )
        .await
    }

    async fn refunds(&self) -> String {
        let rows: Vec<String> = self
            .client
            .query(
                "SELECT concat(refund_id, ' ', toString(applied), ' ', invalid_reason) \
                 FROM conversion_refunds(workspace_id = ?, conversion_id = 'c') ORDER BY refund_id",
            )
            .bind(&self.workspace)
            .fetch_all()
            .await
            .unwrap();
        rows.iter().map(|r| r.trim()).collect::<Vec<_>>().join(",")
    }

    fn refund(&self, refund_id: &str, external_id: &str, value: &str) -> ConversionRecord {
        let mut r = self.event("none");
        r.kind = "refund".into();
        r.click_id = None;
        r.step = String::new();
        r.refund_id = Some(refund_id.into());
        r.external_id = Some(external_id.into());
        r.value = Some(value.into());
        r
    }

    fn order(&self, external_id: &str, value: &str) -> ConversionRecord {
        let mut r = self.event("none");
        r.click_id = None;
        r.counting = "Every".into();
        r.external_id = Some(external_id.into());
        r.value = Some(value.into());
        r
    }
}

#[tokio::test]
#[ignore]
async fn events_take_their_dimensions_from_the_click_and_are_not_clicks() {
    let db = Db::new();
    db.my_click("a").await;
    let clicks = "SELECT toString(count()) FROM click_stream WHERE workspace_id = ?";
    let daily = "SELECT toString(sum(total_clicks)) FROM click_stream_daily_mv WHERE workspace_id = ?";
    let before = (db.one(clicks).await, db.one(daily).await);
    let e = db.event("a");
    // Decided when read: the click is already stored.
    db.store(&[e.clone()]).await;
    assert_eq!(
        db.one(
            "SELECT concat(status, ' ', route_id, ' ', country, ' ', device_family, ' ', \
             os_family, ' ', user_agent_family, ' ', ifNull(toString(value), '')) \
             FROM conversion_events FINAL WHERE workspace_id = ?"
        )
        .await,
        "attributed route DE Mac Mac OS X Safari 49"
    );
    let pending = "SELECT toString(count()) FROM conversion_pending WHERE workspace_id = ?";
    assert_eq!(db.one(pending).await, "0");
    assert_eq!(before, (db.one(clicks).await, db.one(daily).await));
    assert_eq!(before.0, "1");
}

#[tokio::test]
#[ignore]
async fn unreadable_records_do_not_stop_the_others() {
    let db = Db::new();
    let order = serde_json::to_vec(&db.order("o", "1")).unwrap();
    let mut bad = db.order("bad", "1");
    bad.value = Some("1.00001".into());
    let bad = serde_json::to_vec(&bad).unwrap();
    let payloads: [&[u8]; 4] = [b"{", &bad, b"\xff", &order];
    store_batch(&db.store, payloads.into_iter()).await.unwrap();
    assert_eq!(db.count().await, 1);
}

#[tokio::test]
#[ignore]
async fn events_without_a_click_id_are_unattributed() {
    let db = Db::new();
    let e = db.order("o", "1");
    db.store(&[e.clone()]).await;
    assert_eq!(db.events().await, [format!("{} unattributed no_click_id", e.id)]);
}

#[tokio::test]
#[ignore]
async fn late_clicks_are_found_also_after_a_restart() {
    let db = Db::new();
    let e = db.event("late");
    db.store(&[e.clone()]).await;
    db.reconcile(Utc::now() + Duration::minutes(14)).await;
    assert_eq!((db.count().await, db.waiting().await), (0, "1".into()));
    db.my_click("late").await;
    // Nothing but the database is kept.
    let restarted = Db { workspace: db.workspace.clone(), ..Db::new() };
    restarted.reconcile(Utc::now() + Duration::minutes(14)).await;
    assert_eq!(db.events().await, [format!("{} attributed ", e.id)]);
    assert_eq!(db.waiting().await, "0");
}

#[tokio::test]
#[ignore]
async fn missing_clicks_are_counted_unattributed_and_stay_late() {
    let db = Db::new();
    let e = db.event("never");
    let mut forged = db.event("forged");
    forged.click_id = Some("not-a-click-id".into());
    db.store(&[e.clone(), forged.clone()]).await;
    let stored = db.pending(&e, state::WAITING).await.unwrap().stored_at;
    let now = stored + Duration::minutes(15);
    db.judge_alone(&e, state::WAITING, waiting_step, now).await;
    db.judge_alone(&forged, state::WAITING, waiting_step, now).await;
    let mut expected = [e.id.clone(), forged.id.clone()].map(|id| format!("{id} unattributed click_not_found"));
    expected.sort();
    assert_eq!(db.events().await, expected);
    // Only the event whose click ID can name a click stays late.
    assert_eq!((db.waiting().await, db.late().await), ("0".into(), "1".into()));
    assert!(db.pending(&e, state::LATE).await.is_some());
}

/// An event that went late in this test's workspace, with value 49.
async fn late_event(db: &Db, click: &str) -> ConversionRecord {
    let e = db.event(click);
    db.store(&[e.clone()]).await;
    let stored = db.pending(&e, state::WAITING).await.unwrap().stored_at;
    db.judge_alone(&e, state::WAITING, waiting_step, stored + Duration::minutes(15)).await;
    assert_eq!(db.event_row().await, "unattributed click_not_found  49");
    e
}

#[tokio::test]
#[ignore]
async fn clicks_delayed_by_a_backlog_attribute_late_events() {
    let db = Db::new();
    let e = late_event(&db, "delayed").await;
    let counted = RECORDS.with_label_values(&["late_attributed"]).get();

    // Nothing yet: it stays late.
    db.judge_alone(&e, state::LATE, late_step, Utc::now() + Duration::hours(1)).await;
    assert_eq!(db.late().await, "1");

    // The click is stored two hours late; the aggregator restarted meanwhile.
    db.click("delayed", &db.workspace, e.received_at - Duration::minutes(1)).await;
    let restarted = Db { workspace: db.workspace.clone(), ..Db::new() };
    restarted.judge_alone(&e, state::LATE, late_step, Utc::now() + Duration::hours(2)).await;
    assert_eq!(db.event_row().await, "attributed  route 49");
    assert_eq!(db.count().await, 1);
    assert_eq!(db.late().await, "0");
    assert!(RECORDS.with_label_values(&["late_attributed"]).get() > counted);
}

#[tokio::test]
#[ignore]
async fn both_passes_run_over_the_whole_table() {
    let db = Db::new();
    late_event(&db, "now").await;
    db.my_click("now").await;
    reconcile(&db.store, Utc::now()).await.unwrap();
    recheck_late(&db.store, Utc::now()).await.unwrap();
    assert_eq!(db.event_row().await, "attributed  route 49");
    let metrics: Vec<_> = prometheus::gather().into_iter().map(|m| m.get_name().to_string()).collect();
    assert!(metrics.iter().any(|m| m == "conversion_events_late"), "{metrics:?}");
}

#[tokio::test]
#[ignore]
async fn late_events_without_a_click_end_after_the_horizon() {
    let db = Db::new();
    let e = late_event(&db, "never").await;
    let end = e.received_at + LATE_HORIZON;
    db.judge_alone(&e, state::LATE, late_step, end - Duration::minutes(1)).await;
    assert_eq!(db.late().await, "1");
    db.judge_alone(&e, state::LATE, late_step, end).await;
    assert_eq!(db.late().await, "0");
    assert_eq!(db.event_row().await, "unattributed click_not_found  49");
}

#[tokio::test]
#[ignore]
async fn late_events_with_an_ineligible_click_end_unchanged() {
    let db = Db::new();
    let e = late_event(&db, "theirs").await;
    db.click("theirs", "other", e.received_at - Duration::minutes(1)).await;
    db.judge_alone(&e, state::LATE, late_step, Utc::now()).await;
    assert_eq!(db.late().await, "0");
    assert_eq!(db.event_row().await, "unattributed click_not_found  49");
    assert_eq!(db.rejected().await, "");
}

#[tokio::test]
#[ignore]
async fn other_workspaces_clicks_are_rejected_and_reveal_nothing() {
    let db = Db::new();
    db.click("theirs", "other", Utc::now() - Duration::hours(1)).await;
    db.store(&[db.event("theirs")]).await;
    assert_eq!(db.count().await, 0);
    assert_eq!(db.rejected().await, "click_not_eligible");
}

#[tokio::test]
#[ignore]
async fn window_and_filter_are_applied() {
    let db = Db::new();
    db.click("old", &db.workspace, Utc::now() - Duration::days(31)).await;
    db.click("future", &db.workspace, Utc::now() + Duration::hours(1)).await;
    db.my_click("filtered").await;
    let mut filtered = db.event("filtered");
    filtered.route_filter = Some(vec!["elsewhere".into()]);
    db.store(&[db.event("old"), db.event("future"), filtered]).await;
    let mut reasons = db
        .rows("SELECT concat(status, ' ', reason) FROM conversion_events FINAL WHERE workspace_id = ?")
        .await;
    reasons.sort();
    assert_eq!(
        reasons,
        ["unattributed before_click", "unattributed outside_window", "unattributed route_not_covered"]
    );
}

#[tokio::test]
#[ignore]
async fn once_per_click_keeps_the_earliest_accepted_event() {
    let db = Db::new();
    db.my_click("a").await;
    let first = db.event("a");
    let mut second = db.event("a");
    second.received_at = first.received_at + Duration::seconds(1);
    // The later one is stored and matched first.
    db.store(&[second.clone()]).await;
    db.reconcile(Utc::now()).await;
    assert_eq!(db.events().await, [format!("{} attributed ", second.id)]);
    db.store(&[first.clone()]).await;
    db.reconcile(Utc::now()).await;
    assert_eq!(db.events().await, [format!("{} attributed ", first.id)]);

    // Rejected events do not take the place of accepted ones.
    let mut inactive = db.event("a");
    inactive.received_at = first.received_at - Duration::seconds(1);
    inactive.rejection = Some("not_active".into());
    db.store(&[inactive]).await;
    db.reconcile(Utc::now()).await;
    assert_eq!(db.events().await, [format!("{} attributed ", first.id)]);
    assert_eq!(db.rejected().await, "not_active");

    // Another step counts for itself.
    let mut step = db.event("a");
    step.step = "cart".into();
    db.store(&[step]).await;
    db.reconcile(Utc::now()).await;
    assert_eq!(db.count().await, 2);
}

#[tokio::test]
#[ignore]
async fn every_conversion_counts_external_and_event_ids_once() {
    let db = Db::new();
    db.my_click("a").await;
    let event = |external: Option<&str>, event_id: &str, source: &str| {
        let mut r = db.event("a");
        r.counting = "Every".into();
        r.external_id = external.map(String::from);
        r.event_id = event_id.into();
        r.source = source.into();
        r
    };
    db.store(&[event(Some("o1"), "1", "browser"), event(Some("o2"), "2", "server")]).await;
    db.reconcile(Utc::now()).await;
    assert_eq!(db.count().await, 2);
    db.store(&[event(Some("o1"), "3", "server")]).await;
    db.reconcile(Utc::now()).await;
    assert_eq!(db.count().await, 2);
    db.store(&[event(None, "4", "browser"), event(None, "4", "browser")]).await;
    db.reconcile(Utc::now()).await;
    assert_eq!(db.count().await, 3);
}

#[tokio::test]
#[ignore]
async fn redelivered_records_change_nothing() {
    let db = Db::new();
    db.my_click("a").await;
    let records = [db.event("a"), db.order("o", "49"), db.refund("f", "o", "9")];
    for _ in 0..3 {
        db.store(&records).await;
        db.reconcile(Utc::now()).await;
    }
    assert_eq!(db.count().await, 2);
    assert_eq!(db.waiting().await, "0");
    assert_eq!(db.refunds().await, "f 9");
}

#[tokio::test]
#[ignore]
async fn refunds_reduce_revenue_up_to_the_original() {
    let db = Db::new();
    let order = db.order("o", "49");
    let mut partial = db.refund("f1", "o", "20");
    partial.received_at = order.received_at + Duration::seconds(1);
    let mut excess = db.refund("f2", "o", "40");
    excess.received_at = order.received_at + Duration::seconds(2);
    let mut other_currency = db.refund("f3", "o", "1");
    other_currency.currency = "USD".into();
    let unknown = db.refund("f4", "nope", "1");

    // Arriving before the order makes no difference.
    db.store(&[excess.clone(), partial.clone()]).await;
    assert_eq!(db.refunds().await, "f1 0 original_not_found,f2 0 original_not_found");
    db.store(&[order]).await;
    assert_eq!(db.refunds().await, "f1 20,f2 29 exceeds_original");

    let mut repeated = partial.clone();
    repeated.id = ulid::Ulid::new().to_string();
    repeated.value = Some("5".into());
    repeated.received_at += Duration::seconds(5);
    db.store(&[repeated, other_currency, unknown]).await;
    assert_eq!(
        db.refunds().await,
        "f1 20,f2 29 exceeds_original,f3 0 currency_mismatch,f4 0 original_not_found"
    );
    assert_eq!(db.count().await, 1);
}

#[tokio::test]
#[ignore]
async fn tests_do_not_replace_or_refund_live_events() {
    let db = Db::new();
    let mut test = db.order("o", "5");
    test.test = true;
    let mut refund = db.refund("f", "o", "5");
    refund.test = true;
    db.store(&[db.order("o", "49"), test, refund]).await;
    assert_eq!(db.count().await, 2);
    assert_eq!(db.refunds().await, "f 5");
}
