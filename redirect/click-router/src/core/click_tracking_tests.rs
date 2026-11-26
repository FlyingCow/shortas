//! Requests through the whole flow, with the modules that decide on tracking.

use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use http::{HeaderMap, Method};
use serde_json::Value;

use crate::{
    adapters::{
        memory::{
            routes_store::InMemoryRoutesStore, user_settings_store::InMemoryUserSettingsStore,
        },
        moka::{
            routes_cache::MokaRoutesCache,
            settings::{RoutesCacheSettings, UserSettingsCacheSettings},
            user_settings_cache::MokaUserSettingsCache,
        },
        uaparser::{settings::UAParser, user_agent_detector::UAParserUserAgentDetector},
        HitRegistrarType, LocationDetectorType, RequestType, ResponseType, RoutesCacheType,
        RoutesStoreType, UserAgentDetectorType, UserSettingsCacheType, UserSettingsStoreType,
    },
    core::{
        flow_router::{FlowRouter, FlowRouterResult, RequestData, ResponseData},
        modules::{ConditionalModule, FlowModules, RedirectOnlyModule},
    },
    model::{
        expression::{Expression, UA},
        route::{ConditionalRouting, DestinationFormat, RoutingPolicy},
        user_settings::SKIP_TRACKING,
        Route, UserSettings,
    },
};

const FIREFOX: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:126.0) Gecko/20100101 Firefox/126.0";
const GOOGLEBOT: &str = "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)";
const DEST: &str = "https://a.com/p?x=1&x=2#top";

struct Setup {
    route: Route,
    extra: Vec<Route>,
    settings: UserSettings,
    settings_fail: bool,
    enabled: bool,
    method: Method,
    user_agent: &'static str,
}

impl Default for Setup {
    fn default() -> Self {
        Self {
            route: route("test", DEST),
            extra: vec![],
            settings: UserSettings::default(),
            settings_fail: false,
            enabled: true,
            method: Method::GET,
            user_agent: FIREFOX,
        }
    }
}

fn route(path: &str, dest: &str) -> Route {
    let mut route = Route {
        switch: "main".into(),
        link: format!("localhost%2F{path}"),
        dest: Some(dest.into()),
        ..Default::default()
    };
    route.properties.route_id = Some("route".into());
    route.properties.owner_id = Some("owner".into());
    route.properties.workspace_id = Some("workspace".into());
    route.properties.track_conversions = true;
    route
}

impl Setup {
    async fn router(&self, hits: Arc<Mutex<Vec<Value>>>) -> FlowRouter {
        let routes = InMemoryRoutesStore::new();
        for route in self.extra.iter().chain([&self.route]) {
            routes.insert(&route.switch, &route.link, route.clone()).await;
        }
        let settings = InMemoryUserSettingsStore::new();
        settings.insert("owner", self.settings.clone()).await;

        FlowRouter::default(
            RoutesCacheType::Moka(MokaRoutesCache::new(
                RoutesStoreType::InMemory(routes),
                RoutesCacheSettings {
                    max_capacity: 10,
                    time_to_live_minutes: 1,
                    time_to_idle_minutes: 1,
                },
            )),
            UserSettingsCacheType::Moka(MokaUserSettingsCache::new(
                if self.settings_fail {
                    UserSettingsStoreType::Failing
                } else {
                    UserSettingsStoreType::InMemory(settings)
                },
                UserSettingsCacheSettings {
                    max_capacity: 10,
                    time_to_live_minutes: 1,
                    time_to_idle_minutes: 1,
                },
            )),
            UserAgentDetectorType::UAParser(UAParserUserAgentDetector::new(&UAParser {
                yaml: "../data/ua-parser/regexes.yaml".into(),
            })),
            LocationDetectorType::None(),
            HitRegistrarType::Recording(hits),
            vec![
                FlowModules::Conditional(ConditionalModule::new()),
                FlowModules::RedirectOnly(RedirectOnlyModule::new()),
            ],
        )
        .with_click_ids(self.enabled)
    }

    async fn send(&self, router: &FlowRouter) -> String {
        let mut headers = HeaderMap::new();
        headers.insert("host", "localhost".parse().unwrap());
        headers.insert("user-agent", self.user_agent.parse().unwrap());
        let request = RequestType::Test(RequestData {
            uri: "/test".parse().unwrap(),
            method: self.method.clone(),
            headers,
            ..Default::default()
        });
        let response = ResponseType::Test(ResponseData::default());
        match router.handle(&request, &response).await.unwrap() {
            FlowRouterResult::Redirect(location, _) => location,
            other => panic!("expected a redirect, got {other}"),
        }
    }

    /// The redirect location and the recorded hit.
    async fn open(self) -> (String, Option<Value>) {
        let hits = Arc::new(Mutex::new(vec![]));
        let router = self.router(hits.clone()).await;
        let location = self.send(&router).await;
        let hit = hits.lock().unwrap().pop();
        (location, hit)
    }
}

fn assert_recorded_without_id(opened: (String, Option<Value>), dest: &str) {
    let (location, hit) = opened;
    let hit = hit.expect("a click");
    assert_eq!(location, dest);
    assert_eq!(hit["trackable"], false);
    assert_eq!(hit["data"]["Click"]["dest"], dest);
}

#[tokio::test]
async fn covered_route_gets_the_recorded_click_id() {
    let (location, hit) = Setup::default().open().await;
    let hit = hit.unwrap();
    let id = hit["id"].as_str().unwrap();

    assert!(ulid::Ulid::from_string(id).is_ok());
    assert_eq!(location, format!("https://a.com/p?x=1&x=2&shortas_cid={id}#top"));
    assert_eq!(hit["trackable"], true);
    assert_eq!(hit["data"]["Click"]["dest"], DEST);
}

#[tokio::test]
async fn existing_click_id_is_replaced() {
    let (location, hit) = Setup {
        route: route("test", "https://a.com/?shortas_cid=old&x=1"),
        ..Default::default()
    }
    .open()
    .await;
    let hit = hit.unwrap();
    let id = hit["id"].as_str().unwrap();
    assert_eq!(location, format!("https://a.com/?x=1&shortas_cid={id}"));
}

#[tokio::test]
async fn uncovered_and_opted_out_routes_are_unchanged() {
    let mut uncovered = route("test", DEST);
    uncovered.properties.track_conversions = false;
    let mut opted_out = route("test", DEST);
    opted_out.properties.conversion_opt_out = true;

    for route in [uncovered, opted_out] {
        let opened = Setup { route, ..Default::default() }.open().await;
        assert_recorded_without_id(opened, DEST);
    }
}

#[tokio::test]
async fn bots_native_destinations_and_the_setting_prevent_the_id() {
    let opened = Setup { user_agent: GOOGLEBOT, ..Default::default() }.open().await;
    assert_recorded_without_id(opened, DEST);

    let mut native = route("test", "myapp://open?x=1");
    native.dest_format = DestinationFormat::Native;
    let opened = Setup { route: native, ..Default::default() }.open().await;
    assert_recorded_without_id(opened, "myapp://open?x=1");

    let opened = Setup { enabled: false, ..Default::default() }.open().await;
    assert_recorded_without_id(opened, DEST);
}

#[tokio::test]
async fn unparseable_destination_is_unchanged() {
    for dest in ["not a url", "https://a.com/a b", "https://a.com:99999/"] {
        let opened = Setup { route: route("test", dest), ..Default::default() }.open().await;
        assert_recorded_without_id(opened, dest);
    }
}

#[tokio::test]
async fn requests_that_are_not_recorded_are_still_redirected() {
    let skip = UserSettings { skip: vec![SKIP_TRACKING.into()], ..Default::default() };
    let overflow = UserSettings { overflow: true, ..Default::default() };
    let mut ownerless = route("test", DEST);
    ownerless.properties.owner_id = None;

    for setup in [
        Setup { settings: skip, ..Default::default() },
        Setup { settings: overflow, ..Default::default() },
        Setup { method: Method::HEAD, ..Default::default() },
        Setup { route: ownerless, ..Default::default() },
        Setup { settings_fail: true, ..Default::default() },
    ] {
        let (location, hit) = setup.open().await;
        assert_eq!(location, DEST);
        assert!(hit.is_none());
    }
}

#[tokio::test]
async fn unrecorded_requests_with_debug_tracing_are_redirected() {
    let mut debug = route("test", DEST);
    debug.properties.allow_debug = true;
    let opened = Setup { route: debug, method: Method::HEAD, ..Default::default() };
    assert_eq!(opened.open().await.0, DEST);
}

#[tokio::test]
async fn conditional_destination_gets_the_id() {
    let mut conditional = route("test", "https://default.com");
    conditional.policy = RoutingPolicy::Conditional(vec![ConditionalRouting {
        key: "firefox".into(),
        condition: Expression {
            ua: Some(UA::EQ("Firefox".into())),
            ..Default::default()
        },
    }]);
    let mut target = route("test", "https://b.com/?y=1#f");
    target.switch = "firefox".into();
    // Coverage belongs to the link that was opened
    target.properties.track_conversions = false;

    let (location, hit) = Setup { route: conditional, extra: vec![target], ..Default::default() }
        .open()
        .await;
    let hit = hit.unwrap();
    let id = hit["id"].as_str().unwrap();
    assert_eq!(location, format!("https://b.com/?y=1&shortas_cid={id}#f"));
    assert_eq!(hit["data"]["Click"]["dest"], "https://b.com/?y=1#f");
}

#[tokio::test]
async fn ids_differ_across_instances() {
    let hits = Arc::new(Mutex::new(vec![]));
    let setup = Setup::default();
    for _ in 0..4 {
        let router = setup.router(hits.clone()).await;
        for _ in 0..2_000 {
            setup.send(&router).await;
        }
    }
    let hits = hits.lock().unwrap();
    let ids: HashSet<_> = hits.iter().map(|hit| hit["id"].as_str().unwrap()).collect();
    assert_eq!(ids.len(), 8_000);
}
