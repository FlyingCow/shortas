use chrono::{DateTime, Duration, Utc};

use super::record::ConversionRecord;

/// How long an event waits for its click to be stored.
pub const CLICK_WAIT: Duration = Duration::minutes(15);

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
            Decision::Unattributed("click_not_found")
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
}
