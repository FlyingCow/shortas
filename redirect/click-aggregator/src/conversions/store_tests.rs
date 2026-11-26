//! Needs ClickHouse with the migrations applied:
//! `CONVERSIONS_CLICKHOUSE_URL`, `_USER`, `_PASSWORD` and `_DATABASE`.
//! Run with `cargo test -p click-aggregator -- --ignored`.

use chrono::{DateTime, Duration, Utc};
use clickhouse::{Client, Compression};

use super::{
    consumer::store_batch,
    reconciler::reconcile,
    record::{tests::record, ConversionRecord},
    store::ConversionStore,
};
use crate::adapters::clickhouse::settings::ClickStreamStoreConfig;

struct Db {
    store: ConversionStore,
    client: Client,
    workspace: String,
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
        format!("{}-{click}", self.workspace)
    }

    async fn click(&self, click: &str, workspace: &str, created: DateTime<Utc>) {
        self.client
            .query(
                "INSERT INTO click_stream (id, workspace_id, route_id, created, country, \
                 device_family, os_family, user_agent_family, is_trackable) \
                 VALUES (?, ?, 'route', fromUnixTimestamp64Milli(toInt64(?)), 'DE', 'Mac', 'Mac OS X', 'Safari', 1)",
            )
            .bind(self.click_id(click))
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
        self.one(
            "SELECT toString(count()) FROM conversion_pending FINAL \
             WHERE workspace_id = ? AND resolved = 0",
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
    db.store(&[e.clone()]).await;
    assert_eq!(db.count().await, 0);
    db.reconcile(Utc::now()).await;
    assert_eq!(
        db.one(
            "SELECT concat(status, ' ', route_id, ' ', country, ' ', device_family, ' ', \
             os_family, ' ', user_agent_family, ' ', ifNull(toString(value), '')) \
             FROM conversion_events FINAL WHERE workspace_id = ?"
        )
        .await,
        "attributed route DE Mac Mac OS X Safari 49"
    );
    assert_eq!(db.waiting().await, "0");
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
async fn missing_clicks_are_given_up() {
    let db = Db::new();
    let e = db.event("never");
    db.store(&[e.clone()]).await;
    // Judged alone: the shared table holds other tests' waiting events, which
    // must not be given up here.
    let store = &db.store;
    let mut after = String::new();
    let waiting = loop {
        let page = store.waiting(&after, 1000).await.unwrap();
        after = page.last().expect("not stored").id.clone();
        if let Some(w) = page.into_iter().find(|w| w.id == e.id) {
            break w;
        }
    };
    let now = waiting.stored_at + Duration::minutes(15);
    let mut batch = super::store::Batch::default();
    let decision = super::attribution::decide(&e, None, waiting.stored_at, now);
    batch.resolve(&waiting, decision, now).unwrap();
    store.insert(batch).await.unwrap();
    assert_eq!(db.events().await, [format!("{} unattributed click_not_found", e.id)]);
    assert_eq!(db.waiting().await, "0");
}

#[tokio::test]
#[ignore]
async fn other_workspaces_clicks_are_rejected_and_reveal_nothing() {
    let db = Db::new();
    db.click("theirs", "other", Utc::now() - Duration::hours(1)).await;
    db.store(&[db.event("theirs")]).await;
    db.reconcile(Utc::now()).await;
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
    db.reconcile(Utc::now()).await;
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
