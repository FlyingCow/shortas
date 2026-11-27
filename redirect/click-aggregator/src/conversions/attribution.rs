use chrono::{DateTime, Duration, Utc};
use ulid::Ulid;

use super::record::ConversionRecord;

/// How long an event waits for its click to be stored.
pub const CLICK_WAIT: Duration = Duration::minutes(15);
/// How long after its receipt an event is re-checked when its click was not
/// found in time. It is counted as unattributed meanwhile.
pub const LATE_HORIZON: Duration = Duration::hours(72);
/// Clocks of routers may differ by this much.
const CLOCK_DIFFERENCE: Duration = Duration::minutes(1);

pub const CLICK_NOT_FOUND: &str = "click_not_found";

#[derive(Debug, Clone, PartialEq)]
pub struct Click {
    pub workspace_id: String,
    pub route_id: String,
    pub created: DateTime<Utc>,
    pub country: String,
    pub device_family: String,
    pub os_family: String,
    pub user_agent_family: String,
    pub trackable: bool,
}

#[derive(Debug, PartialEq)]
pub enum Decision<'a> {
    Attributed(&'a Click),
    Unattributed(&'static str),
    Rejected(&'static str),
    Wait,
}

/// Matches an event to the click it names. `stored_at` is when the event was
/// stored, so a delayed stream does not shorten the wait.
pub fn decide<'a>(
    record: &ConversionRecord,
    click: Option<&'a Click>,
    stored_at: DateTime<Utc>,
    now: DateTime<Utc>,
) -> Decision<'a> {
    let Some(click) = click else {
        return if now - stored_at < CLICK_WAIT {
            Decision::Wait
        } else {
            Decision::Unattributed(CLICK_NOT_FOUND)
        };
    };
    if click.workspace_id != record.workspace_id {
        return Decision::Rejected("click_not_eligible");
    }
    if !click.trackable {
        return Decision::Unattributed("click_not_trackable");
    }
    let covered = record
        .route_filter
        .as_ref()
        .is_none_or(|routes| routes.contains(&click.route_id));
    if !covered {
        return Decision::Unattributed("route_not_covered");
    }
    // Clicks are stored in milliseconds.
    if record.occurred_at.timestamp_millis() < click.created.timestamp_millis() {
        return Decision::Unattributed("before_click");
    }
    if record.occurred_at - click.created > Duration::days(record.window_days.into()) {
        return Decision::Unattributed("outside_window");
    }
    Decision::Attributed(click)
}

/// Whether the event's click ID can still name a click: click IDs are ULIDs
/// made at the redirect, so they tell when their click happened.
pub fn may_name_a_click(record: &ConversionRecord) -> bool {
    let Some(Ok(id)) = record.click_id.as_deref().map(Ulid::from_string) else {
        return false;
    };
    let Some(clicked) = DateTime::from_timestamp_millis(id.timestamp_ms() as i64) else {
        return false;
    };
    let received = record.received_at;
    clicked <= received + CLOCK_DIFFERENCE
        && received - clicked <= Duration::days(record.window_days.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conversions::record::tests::record;

    fn click(record: &ConversionRecord) -> Click {
        Click {
            workspace_id: "w".into(),
            route_id: "r".into(),
            created: record.occurred_at - Duration::hours(1),
            country: "DE".into(),
            device_family: "Mac".into(),
            os_family: "Mac OS X".into(),
            user_agent_family: "Safari".into(),
            trackable: true,
        }
    }

    #[test]
    fn decides_by_the_click() {
        let mut r = record("e");
        let now = r.received_at;
        let base = click(&r);
        let decided = |r: &ConversionRecord, change: fn(&mut Click)| {
            let mut c = base.clone();
            change(&mut c);
            match decide(r, Some(&c), now, now) {
                Decision::Attributed(_) => "attributed",
                Decision::Unattributed(reason) | Decision::Rejected(reason) => reason,
                Decision::Wait => "wait",
            }
        };
        assert_eq!(decided(&r, |_| {}), "attributed");
        assert_eq!(decided(&r, |c| c.workspace_id = "other".into()), "click_not_eligible");
        assert_eq!(decided(&r, |c| c.trackable = false), "click_not_trackable");
        assert_eq!(decided(&r, |c| c.created += Duration::hours(2)), "before_click");
        assert_eq!(decided(&r, |c| c.created += Duration::hours(1)), "attributed");
        assert_eq!(decided(&r, |c| c.created -= Duration::days(30)), "outside_window");
        assert_eq!(decided(&r, |c| c.created -= Duration::days(29)), "attributed");
        r.route_filter = Some(vec!["r".into()]);
        assert_eq!(decided(&r, |_| {}), "attributed");
        r.route_filter = Some(vec!["x".into()]);
        assert_eq!(decided(&r, |_| {}), "route_not_covered");
        // Nothing about another workspace's click, not even its route
        assert_eq!(decided(&r, |c| c.workspace_id = "other".into()), "click_not_eligible");
    }

    #[test]
    fn waits_for_missing_clicks() {
        let r = record("e");
        let stored = r.received_at + Duration::hours(1);
        let at = |minutes| decide(&r, None, stored, stored + Duration::minutes(minutes));
        assert_eq!(at(0), Decision::Wait);
        assert_eq!(at(14), Decision::Wait);
        assert_eq!(at(15), Decision::Unattributed("click_not_found"));
    }

    #[test]
    fn only_ulids_from_the_window_may_name_a_click() {
        let mut r = record("e");
        let at = |offset: Duration| {
            let ms = (r.received_at + offset).timestamp_millis() as u64;
            Some(Ulid::from_parts(ms, 7).to_string())
        };
        let cases = [
            (Some("not-a-ulid".to_string()), false),
            (None, false),
            (at(Duration::minutes(2)), false),
            (at(Duration::seconds(30)), true),
            (at(Duration::hours(-1)), true),
            (at(Duration::days(-29)), true),
            (at(Duration::days(-31)), false),
        ];
        for (click_id, expected) in cases {
            r.click_id = click_id.clone();
            assert_eq!(may_name_a_click(&r), expected, "{click_id:?}");
        }
        r.click_id = at(Duration::hours(-1)).map(|id| id.to_lowercase());
        assert!(may_name_a_click(&r));
    }
}
