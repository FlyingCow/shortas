use anyhow::Result;
use chrono::{DateTime, Utc};
use cookie::{Cookie, CookieJar};
use http::{
    header::IntoHeaderName, uri::Scheme, Extensions, HeaderMap, HeaderValue, Method, StatusCode,
    Uri, Version,
};
use indexmap::IndexMap;
use multimap::MultiMap;
use rand;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use std::{
    self,
    collections::HashMap,
    fmt::{self, Display, Formatter, Result as FmtResult},
    net::SocketAddr,
};

use crate::{
    adapters::{
        HitRegistrarType, LocationDetectorType, RequestType, ResponseType, RoutesCacheType,
        UserAgentDetectorType, UserSettingsCacheType,
    },
    model::{
        hit::{Click, HitRoute},
        Hit, Route, UserSettings,
    },
};

use super::{
    flow_module::{FlowModule, FlowStepContinuation},
    hits_register::HitRegistrar,
    host::{HostExtractor, HostInfo},
    ip::{IPExtractor, IPInfo},
    language::{Language, LanguageExtractor},
    location::{Country, LocationDetector},
    metrics::{FlowRouterMetrics, Timer},
    modules::FlowModules,
    protocol::{ProtoInfo, ProtocolExtractor},
    routes::RoutesManager,
    trace::{HitTrace, TraceCollector},
    user_agent::{Device, UserAgent, UserAgentDetector, OS},
    user_agent_string::UserAgentStringExtractor,
    user_settings::UserSettingsManager,
    InitOnce,
};

/// Represents the type of HTTP redirect to be performed
///
/// This enum defines whether a redirect should be permanent (301) or temporary (302).
/// The choice affects SEO and browser caching behavior.
#[derive(Clone, Debug)]
pub enum RedirectType {
    /// HTTP 301 - Moved Permanently
    /// Indicates that the resource has been permanently moved to a new location
    Permanent,
    /// HTTP 302 - Found (Temporary Redirect)  
    /// Indicates that the resource is temporarily available at a different location
    Temporary,
}

/// Represents the possible outcomes of processing a request through the flow router
///
/// This enum encapsulates all the different ways a request can be handled,
/// from simple responses to redirects, proxied requests, and retargeting scenarios.
#[derive(Clone, Debug)]
pub enum FlowRouterResult {
    /// Return an empty response with the specified HTTP status code
    Empty(StatusCode),
    /// Return a JSON response with content and HTTP status code
    Json(String, StatusCode),
    /// Return a plain text response with content and HTTP status code
    PlainText(String, StatusCode),
    /// Return an image response with binary data, content type, and HTTP status code
    Image(Vec<u8>, String, StatusCode),
    /// Proxy the request to another URI and return the response with the specified status code
    Proxied(Uri, StatusCode),
    /// Perform an HTTP redirect to the specified URI with the given redirect type
    Redirect(Uri, RedirectType),
    /// Handle retargeting with a primary URI and fallback URIs
    Retargeting(Uri, Vec<Uri>),
    /// Indicates an error occurred during processing
    Error,
}

impl Display for FlowRouterResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Represents the different stages in the request processing flow
///
/// The flow router processes requests through a series of well-defined steps,
/// each handling a specific aspect of the request lifecycle.
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum FlowStep {
    /// Initial state before processing begins
    #[default]
    Initial,
    /// Starting phase - initial request validation and setup
    Start,
    /// URL extraction phase - parse and analyze the incoming URL
    UrlExtract,
    /// Registration phase - log the request and gather analytics data
    Register,
    /// Result building phase - construct the appropriate response
    BuildResult,
    /// Final phase - cleanup and response finalization
    End,
}

impl Display for FlowStep {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

/// Represents different types of data that can be stored in the flow router context
///
/// This enum allows the flow router to store various types of contextual information
/// that can be used by different modules during request processing.
#[derive(Clone, Debug)]
pub enum FlowRouterData {
    /// Boolean value for flags and binary states
    Bool(bool),
    /// Numeric value for counters, percentages, and calculations
    Number(f64),
    /// Static string reference for labels and identifiers
    String(String),
}

impl FlowRouterData {
    /// Checks if this data is a boolean with the specified value
    ///
    /// # Arguments
    /// * `value` - The boolean value to compare against
    ///
    /// # Returns
    /// * `true` if this is a Bool variant with the matching value, `false` otherwise
    pub fn is_bool(&self, value: bool) -> bool {
        if let FlowRouterData::Bool(bool_value) = &self {
            return *bool_value == value;
        }

        false
    }

    /// Checks if this data is a string with the specified value (case-insensitive)
    ///
    /// # Arguments
    /// * `value` - The string value to compare against
    ///
    /// # Returns
    /// * `true` if this is a String variant with the matching value, `false` otherwise
    pub fn is_string(&self, value: &str) -> bool {
        if let FlowRouterData::String(str_value) = &self {
            return value.eq_ignore_ascii_case(str_value);
        }

        false
    }

    /// Checks if this data is a number with the specified value
    ///
    /// # Arguments
    /// * `value` - The numeric value to compare against
    ///
    /// # Returns
    /// * `true` if this is a Number variant with the matching value, `false` otherwise
    pub fn is_num(&self, value: f64) -> bool {
        if let FlowRouterData::Number(num_value) = &self {
            return *num_value == value;
        }

        false
    }
}

impl<'a> FlowRouterContext<'a> {
    pub fn is_data_true(&self, bool_key: &'static str) -> bool {
        let data_value = self.data.get(&bool_key);

        if let Some(i) = data_value {
            return i.is_bool(true);
        }

        false
    }

    ///
    /// Adds a bool value to the context's data
    ///
    pub fn add_bool(&mut self, bool_key: &'static str, value: bool) {
        let _ = &self.data.insert(bool_key, FlowRouterData::Bool(value));
    }

    ///
    /// Adds a string value to the context's data
    ///
    pub fn add_string(&mut self, bool_key: &'static str, value: String) {
        let _ = &self.data.insert(bool_key, FlowRouterData::String(value));
    }

    pub fn get_string(&mut self, key: &'static str) -> Option<String> {
        let value = &self.data.get(key);

        if let Some(value) = value {
            if let FlowRouterData::String(str_value) = value {
                return Some(str_value.clone());
            }
        }

        None
    }

    ///
    /// Adds a num value to the context's data
    ///
    pub fn add_num(&mut self, bool_key: &'static str, value: f64) {
        let _ = &self.data.insert(bool_key, FlowRouterData::Number(value));
    }
}

/// Represents the incoming route information extracted from a request
///
/// This structure contains all the essential components of an incoming URL
/// that are needed for routing decisions and processing.
#[derive(Clone, Debug)]
pub struct FlowInRoute {
    /// The URL scheme (http, https, etc.)
    pub scheme: String,
    /// The hostname or domain name
    pub host: String,
    /// The port number
    pub port: u16,
    /// The URL path component
    pub path: String,
    /// The query string parameters
    pub query: String,
}

impl FlowInRoute {
    pub fn new(scheme: String, host: String, port: u16, path: String, query: String) -> Self {
        Self {
            scheme,
            host,
            port,
            path,
            query,
        }
    }
}

/// The main context structure that holds all information about a request being processed
///
/// This structure serves as the central data container that flows through all stages
/// of request processing, accumulating information and state as it progresses.
pub struct FlowRouterContext<'a> {
    /// Unique identifier for this request
    pub id: String,
    /// UTC timestamp when the request was received
    pub utc: DateTime<Utc>,
    /// Key-value storage for arbitrary data used by modules
    pub data: HashMap<&'a str, FlowRouterData>,
    /// Lazily-initialized client operating system information
    pub client_os: InitOnce<Option<OS>>,
    /// Lazily-initialized client user agent information
    pub client_ua: InitOnce<Option<UserAgent>>,
    /// Lazily-initialized client device information
    pub client_device: InitOnce<Option<Device>>,
    /// Lazily-initialized client country/location information
    pub client_country: InitOnce<Option<Country>>,
    /// Current step in the processing flow
    pub current_step: FlowStep,
    /// Extracted host information from the request
    pub host: Option<HostInfo>,
    /// Client IP address information
    pub client_ip: Option<IPInfo>,
    /// Raw user agent string from the request headers
    pub user_agent: Option<String>,
    /// Parsed client language preferences
    pub client_langs: Option<Vec<Language>>,
    /// Protocol information (HTTP/HTTPS)
    pub protocol: Option<ProtoInfo>,
    /// The output route determined for this request (Arc to avoid cloning)
    pub out_route: Option<Arc<Route>>,
    /// The main route configuration that matched this request (Arc to avoid cloning)
    pub main_route: Option<Arc<Route>>,
    /// Parsed incoming route information
    pub in_route: FlowInRoute,
    /// Reference to the original request
    pub request: &'a RequestType<'a>,
    /// Reference to the response being built
    pub response: &'a ResponseType<'a>,
    /// The final result of processing this request
    pub result: Option<FlowRouterResult>,
    /// Debug trace collector (only active when allow_debug=true)
    pub trace: Option<TraceCollector>,
    /// Finalized trace data for passing to hit registration
    pub hit_trace: Option<HitTrace>,
}

impl<'a> FlowRouterContext<'a> {
    pub fn new(
        in_route: FlowInRoute,
        request: &'a RequestType<'a>,
        response: &'a ResponseType<'a>,
    ) -> Self {
        // Optimized ID generation with pre-allocation
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(); // Use seconds instead of nanos to avoid u128
        let random = rand::random::<u32>();

        let mut id = String::with_capacity(24);
        use std::fmt::Write;
        let _ = write!(id, "{}_{}", timestamp, random);

        Self {
            id,
            utc: Utc::now(),
            data: HashMap::new(),
            client_os: InitOnce::default(None),
            client_ua: InitOnce::default(None),
            client_device: InitOnce::default(None),
            client_country: InitOnce::default(None),
            current_step: FlowStep::Initial,
            in_route,
            user_agent: None,
            client_ip: None,
            client_langs: None,
            host: None,
            protocol: None,
            out_route: None,
            main_route: None,
            result: None,
            request,
            response,
            trace: None,
            hit_trace: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PerConnHandler {
    pub local_addr: SocketAddr,
    pub remote_addr: SocketAddr,
    pub server_name: String,
    pub tls_info: Option<TlsInfo>,
}

#[derive(Clone, Debug, Default)]
pub struct ResponseData {
    /// The HTTP status code.WebTransportSession
    pub status_code: Option<StatusCode>,
    /// The HTTP headers.
    pub headers: HeaderMap,
    /// The HTTP version.
    pub version: Version,
    /// The HTTP cookies.
    pub cookies: CookieJar,
    /// Used to store extra data derived from the underlying protocol.
    pub extensions: Extensions,
}

#[derive(Clone, Debug, Default)]
pub struct RequestData {
    // The requested URL.
    pub uri: Uri,

    // The request headers.
    pub headers: HeaderMap,

    pub extensions: Extensions,

    // The request method.
    pub method: Method,

    pub cookies: CookieJar,

    pub params: IndexMap<String, String>,

    // accept: Option<Vec<Mime>>,
    pub queries: MultiMap<String, String>,

    /// The version of the HTTP protocol used.
    pub version: Version,

    pub scheme: Option<Scheme>,

    pub local_addr: Option<SocketAddr>,

    pub remote_addr: Option<SocketAddr>,

    pub tls_info: Option<TlsInfo>,
}

pub trait Request {
    fn uri(&self) -> &Uri;
    fn headers(&self) -> &HeaderMap;
    fn method(&self) -> &Method;
    fn scheme(&self) -> &Scheme;
    fn params(&self) -> &IndexMap<String, String>;
    fn queries(&self) -> &MultiMap<String, String>;
    fn remote_addr(&self) -> Option<SocketAddr>;
    fn cookies(&self) -> &CookieJar;
    /// Get `Cookie` from cookies.
    fn cookie<T>(&self, name: T) -> Option<&Cookie<'static>>
    where
        T: AsRef<str>;
}

pub trait Response {
    fn add_header<N, V>(&mut self, name: N, value: V, overwrite: bool) -> Result<()>
    where
        N: IntoHeaderName,
        V: TryInto<HeaderValue>;

    /// Get cookies reference.
    fn cookies(&self) -> &CookieJar;

    /// Get mutable cookies reference.
    fn cookies_mut(&mut self) -> &mut CookieJar;

    /// Helper function for get cookie.
    fn cookie<T>(&self, name: T) -> Option<&Cookie<'static>>
    where
        T: AsRef<str>;

    /// Helper function for add cookie.
    fn add_cookie(&mut self, cookie: Cookie<'static>);
}

#[derive(Clone, Debug)]
pub struct TlsInfo {
    pub sni_hostname: Option<String>,
    pub alpn_protocol: Option<String>,
    pub has_certificate: bool,
}

const MAIN_SWITCH: &'static str = "main";

/// The main flow router that orchestrates request processing through various stages
///
/// This is the central component that manages the entire request lifecycle,
/// coordinating between different extractors, detectors, and modules to process
/// incoming requests and generate appropriate responses.
pub struct FlowRouter {
    /// Manages route configurations and lookups
    routes_manager: RoutesManager,
    /// Manages user-specific settings and preferences
    settings_manager: UserSettingsManager,
    /// Handles hit registration and analytics logging
    hit_registrar: HitRegistrarType,
    /// Extracts host information from requests
    host_extractor: HostExtractor,
    /// Determines protocol information (HTTP/HTTPS)
    protocol_extractor: ProtocolExtractor,
    /// Extracts client IP address information
    ip_extractor: IPExtractor,
    /// Extracts user agent strings from request headers
    user_agent_string_extractor: UserAgentStringExtractor,
    /// Extracts language preferences from request headers
    language_extractor: LanguageExtractor,
    /// Detects user agent details (browser, OS, device)
    user_agent_detector: UserAgentDetectorType,
    /// Detects geographic location from IP addresses
    location_detector: LocationDetectorType,
    /// Collection of processing modules that handle specific logic
    modules: Vec<FlowModules>,
    /// Metrics collection for monitoring and observability
    metrics: FlowRouterMetrics,
    /// Atomic counter for generating unique request IDs
    request_counter: AtomicU64,
}

impl FlowRouter {
    /// Creates a new FlowRouter with default configuration
    ///
    /// # Arguments
    /// * `routes_cache` - Cache implementation for route storage
    /// * `user_settings_cache` - Cache implementation for user settings
    /// * `user_agent_detector` - Service for detecting user agent details
    /// * `location_detector` - Service for detecting geographic location
    /// * `hit_registrar` - Service for registering analytics hits
    /// * `modules` - Collection of processing modules to use
    ///
    /// # Returns
    /// * A new FlowRouter instance configured with the provided components
    pub fn default(
        routes_cache: RoutesCacheType,
        user_settings_cache: UserSettingsCacheType,
        user_agent_detector: UserAgentDetectorType,
        location_detector: LocationDetectorType,
        hit_registrar: HitRegistrarType,
        modules: Vec<FlowModules>,
    ) -> Self {
        FlowRouter {
            routes_manager: RoutesManager::new(routes_cache),
            settings_manager: UserSettingsManager::new(user_settings_cache),
            hit_registrar,
            host_extractor: HostExtractor::new(),
            protocol_extractor: ProtocolExtractor::new(),
            ip_extractor: IPExtractor::new(),
            user_agent_string_extractor: UserAgentStringExtractor::new(),
            language_extractor: LanguageExtractor::new(),
            user_agent_detector,
            location_detector,
            modules,
            metrics: FlowRouterMetrics::new().expect("Failed to initialize metrics"),
            request_counter: AtomicU64::new(0),
        }
    }

    /// Create a FlowRouter with custom metrics
    pub fn with_metrics(
        routes_cache: RoutesCacheType,
        user_settings_cache: UserSettingsCacheType,
        user_agent_detector: UserAgentDetectorType,
        location_detector: LocationDetectorType,
        hit_registrar: HitRegistrarType,
        modules: Vec<FlowModules>,
        metrics: FlowRouterMetrics,
    ) -> Self {
        FlowRouter {
            routes_manager: RoutesManager::new(routes_cache),
            settings_manager: UserSettingsManager::new(user_settings_cache),
            hit_registrar,
            host_extractor: HostExtractor::new(),
            protocol_extractor: ProtocolExtractor::new(),
            ip_extractor: IPExtractor::new(),
            user_agent_string_extractor: UserAgentStringExtractor::new(),
            language_extractor: LanguageExtractor::new(),
            user_agent_detector,
            location_detector,
            modules,
            metrics,
            request_counter: AtomicU64::new(0),
        }
    }

    /// Get a reference to the hit registrar
    pub fn hit_registrar(&self) -> &HitRegistrarType {
        &self.hit_registrar
    }

    /// Get a reference to the metrics
    pub fn metrics(&self) -> &FlowRouterMetrics {
        &self.metrics
    }

    /// Generate an efficient request ID with minimal allocations
    fn generate_request_id(&self) -> String {
        // Use a simple counter-based ID instead of ULID for better performance
        // Format: timestamp_counter (e.g., "1696234567_123")
        let counter = self.request_counter.fetch_add(1, Ordering::Relaxed);
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Pre-allocate capacity to avoid reallocations
        // timestamp (10 digits) + '_' (1) + counter (10 digits max) = ~21 bytes
        let mut id = String::with_capacity(24);

        // Use write! macro which is faster than format! for this use case
        use std::fmt::Write;
        let _ = write!(id, "{}_{}", timestamp, counter);

        id
    }

    /// Processes a request through the iterative flow state machine
    ///
    /// This method implements an optimized iterative approach to request processing,
    /// avoiding the overhead of recursive function calls. It processes the request
    /// through each flow step until completion.
    ///
    /// # Arguments
    /// * `context` - Mutable reference to the flow router context
    ///
    /// # Returns
    /// * `Result<()>` - Ok if processing completes successfully
    ///
    /// # Errors
    /// * Returns an error if any processing step fails
    pub async fn process_flow(&self, context: &mut FlowRouterContext<'_>) -> Result<()> {
        let flow_timer = Timer::new();
        self.metrics.iterative_flow_usage.inc();

        let trace_enabled = context.trace.is_some();

        let mut current_step = FlowStep::Start;

        loop {
            context.current_step = current_step;

            // Start span for this stage if tracing is enabled
            if let Some(ref mut trace) = context.trace {
                trace.start_span(&format!("{:?}", current_step));
            }

            let stage_timer = if trace_enabled {
                Some(Timer::new())
            } else {
                None
            };

            let should_continue = match current_step {
                FlowStep::Start => self.handle_start_iterative(context).await?,
                FlowStep::UrlExtract => self.handle_url_extract_iterative(context).await?,
                FlowStep::Register => self.handle_register_iterative(context).await?,
                FlowStep::BuildResult => self.handle_build_result_iterative(context).await?,
                FlowStep::End => {
                    self.handle_end_iterative(context).await?;
                    // End span and record metrics for End stage
                    if let Some(ref mut trace) = context.trace {
                        trace.end_span();
                    }
                    if let Some(timer) = stage_timer {
                        self.metrics
                            .debug_stage_duration
                            .with_label_values(&["End"])
                            .observe(timer.elapsed_seconds());
                    }
                    break; // End the flow
                }
                _ => return Err(anyhow::anyhow!("Invalid flow step: {:?}", current_step)),
            };

            // End span and record debug metrics for this stage
            if let Some(ref mut trace) = context.trace {
                trace.end_span();
            }
            if let Some(timer) = stage_timer {
                self.metrics
                    .debug_stage_duration
                    .with_label_values(&[&format!("{:?}", current_step)])
                    .observe(timer.elapsed_seconds());
            }

            // Move to next step if modules didn't break the flow
            current_step = match (current_step, should_continue) {
                (FlowStep::Start, true) => FlowStep::UrlExtract,
                (FlowStep::UrlExtract, true) => FlowStep::Register,
                (FlowStep::Register, true) => FlowStep::BuildResult,
                (FlowStep::BuildResult, true) => FlowStep::End,
                (_, false) => break, // Flow was interrupted by a module
                (FlowStep::Initial, true) => return Err(anyhow::anyhow!("Invalid initial state")),
                (FlowStep::End, true) => break, // Should not reach here due to explicit break above
            };
        }

        // Record flow processing time
        flow_timer.observe_duration_seconds(&self.metrics.flow_processing_duration);

        // Finalize trace if enabled and record debug total duration
        if trace_enabled {
            self.metrics
                .debug_total_duration
                .observe(flow_timer.elapsed_seconds());

            // Take the trace collector and finalize it
            if let Some(trace) = context.trace.take() {
                context.hit_trace = Some(trace.finalize());
            }
        }

        Ok(())
    }

    /// Handles the Start phase of request processing (iterative version)
    ///
    /// Executes all registered modules for the Start phase and determines
    /// whether processing should continue to the next phase.
    ///
    /// # Arguments
    /// * `context` - Mutable reference to the flow router context
    ///
    /// # Returns
    /// * `Result<bool>` - Ok(true) to continue to next step, Ok(false) to stop processing
    async fn handle_start_iterative(&self, context: &mut FlowRouterContext<'_>) -> Result<bool> {
        for module in &self.modules {
            let result = module.handle_start(context, &self).await?;

            if result == FlowStepContinuation::Break {
                return Ok(false); // Don't continue to next step
            }
        }

        Ok(true) // Continue to next step
    }

    /// Handles the URL Extract phase of request processing (iterative version)
    ///
    /// Executes all registered modules for the URL extraction phase and determines
    /// whether processing should continue to the next phase.
    ///
    /// # Arguments
    /// * `context` - Mutable reference to the flow router context
    ///
    /// # Returns
    /// * `Result<bool>` - Ok(true) to continue to next step, Ok(false) to stop processing
    async fn handle_url_extract_iterative(
        &self,
        context: &mut FlowRouterContext<'_>,
    ) -> Result<bool> {
        for module in &self.modules {
            let result = module.handle_url_extract(context, &self).await?;

            if result == FlowStepContinuation::Break {
                return Ok(false); // Don't continue to next step
            }
        }

        Ok(true) // Continue to next step
    }

    /// Handles the Register phase of request processing (iterative version)
    ///
    /// Executes all registered modules for the registration phase, logs analytics data,
    /// and determines whether processing should continue to the next phase.
    ///
    /// # Arguments
    /// * `context` - Mutable reference to the flow router context
    ///
    /// # Returns
    /// * `Result<bool>` - Ok(true) to continue to next step, Ok(false) to stop processing
    async fn handle_register_iterative(&self, context: &mut FlowRouterContext<'_>) -> Result<bool> {
        for module in &self.modules {
            let result = module.handle_register(context, &self).await?;

            if result == FlowStepContinuation::Break {
                return Ok(false); // Don't continue to next step
            }
        }

        let trace_enabled = context.trace.is_some();
        let queue_timer = if trace_enabled {
            Some(Timer::new())
        } else {
            None
        };

        // Finalize trace before hit registration so it's included in the hit
        let hit_trace = if let Some(trace) = context.trace.take() {
            Some(trace.finalize())
        } else {
            None
        };

        let click = Click::new(
            context
                .out_route
                .as_ref()
                .unwrap()
                .dest
                .as_ref()
                .unwrap()
                .as_str(),
        );

        let hit = Hit::click_with_trace(
            &context.id,
            context.utc,
            context.user_agent.as_deref(),
            context.client_ip.as_ref().map(|ip| ip.address),
            &click,
            HitRoute::from_route(&context.main_route),
            hit_trace.clone(),
        );

        let hit_result = self.hit_registrar.register(&hit).await;

        // Record hit queue duration for debug routes
        if let Some(timer) = queue_timer {
            self.metrics
                .debug_hit_queue_duration
                .observe(timer.elapsed_seconds());
        }

        // Store the hit trace in context for potential later use
        context.hit_trace = hit_trace;

        if hit_result.is_ok() {
            self.metrics.hits_registered.inc();
        }

        hit_result?;

        Ok(true) // Continue to next step
    }

    /// Iterative version that returns whether to continue to next step
    async fn handle_build_result_iterative(
        &self,
        context: &mut FlowRouterContext<'_>,
    ) -> Result<bool> {
        for module in &self.modules {
            let result = module.handle_build_result(context, &self).await?;

            if result == FlowStepContinuation::Break {
                return Ok(false); // Don't continue to next step
            }
        }

        let result = match &context.out_route {
            Some(route) => {
                let destination = &route
                    .dest
                    .as_ref()
                    .unwrap_or(&String::from("http://test.com"))
                    .to_string();

                FlowRouterResult::Redirect(destination.parse().unwrap(), RedirectType::Temporary)
            }
            None => FlowRouterResult::Empty(StatusCode::NOT_FOUND),
        };

        context.result = Some(result);

        Ok(true) // Continue to next step
    }

    /// Iterative version for the end step
    async fn handle_end_iterative(&self, context: &mut FlowRouterContext<'_>) -> Result<()> {
        for module in &self.modules {
            let result = module.handle_end(context, &self).await?;

            if result == FlowStepContinuation::Break {
                return Ok(());
            }
        }

        Ok(())
    }

    pub async fn get_user_settings(&self, user_id: &str) -> Result<Option<UserSettings>> {
        let user_settings = self.settings_manager.get_user_settings(user_id).await?;

        Ok(user_settings)
    }

    pub async fn get_main_route(
        &self,
        path: &str,
        context: &FlowRouterContext<'_>,
    ) -> Result<Option<Route>> {
        let route = self
            .routes_manager
            .get_route(MAIN_SWITCH, context.in_route.host.as_str(), path)
            .await?;

        Ok(route)
    }

    pub async fn get_route(
        &self,
        switch: &str,
        context: &FlowRouterContext<'_>,
    ) -> Result<Option<Route>> {
        let route = self
            .routes_manager
            .get_route(
                switch,
                context.in_route.host.as_str(),
                context.in_route.path.as_str(),
            )
            .await?;

        Ok(route)
    }

    pub async fn get_route_by_switch(
        &self,
        switch: &str,
        path: &str,
    ) -> Result<Option<Route>> {
        self.routes_manager.get_route_by_switch(switch, path).await
    }

    async fn start<'a>(
        &self,
        req: &'a RequestType<'a>,
        res: &'a ResponseType<'a>,
    ) -> Result<FlowRouterContext<'a>> {
        let mut context = self.build_context(req, res);

        for module in &self.modules {
            let result = module.init(&mut context, &self).await?;

            if result == FlowStepContinuation::Break {
                return Ok(context);
            }
        }

        if let None = context.main_route {
            // Wrap route in Arc to avoid expensive cloning
            if let Some(route) = self.get_route(MAIN_SWITCH, &context).await? {
                let route_arc = Arc::new(route);
                context.main_route = Some(route_arc.clone());
                context.out_route = Some(route_arc);
            }
        } else {
            context.out_route = context.main_route.clone();
        }

        let _ = &self.replace_debug_data(&mut context);

        // Initialize debug tracing if allow_debug is enabled for this route
        if self.allow_debug(&mut context) {
            self.metrics.debug_requests_total.inc();
            context.trace = Some(TraceCollector::new(&context.id));
        }

        self.process_flow(&mut context).await?;

        Ok(context)
    }

    fn build_route_uri(&self, request: &RequestType) -> FlowInRoute {
        let path = &request.uri().path()[1..];

        let host_info = self.host_extractor.detect(&request, false).unwrap();

        let query = request.uri().query().unwrap_or_default();

        let scheme = request.uri().scheme().unwrap_or(&Scheme::HTTP).to_string();

        let in_route = FlowInRoute {
            host: host_info.host,
            port: host_info.port,
            path: path.to_ascii_lowercase(),
            query: query.to_ascii_lowercase(),
            scheme: scheme.to_ascii_lowercase(),
        };

        in_route
    }

    fn allow_debug(&self, context: &mut FlowRouterContext) -> bool {
        if context.main_route.is_none() {
            return false;
        }

        let route = context.main_route.as_ref().unwrap();

        return route.properties.allow_debug;
    }

    pub fn load_country(&self, context: &mut FlowRouterContext) {
        if context.client_country.has_value() {
            return;
        }

        if context.client_ip.is_none() {
            context.client_country.init_with(None);
            return;
        }

        let country = self
            .location_detector
            .detect_country(&context.client_ip.as_ref().unwrap().address);

        context.client_country.init_with(country);
    }

    pub fn load_os(&self, context: &mut FlowRouterContext) {
        if context.client_os.has_value() {
            return;
        }

        if context.user_agent.is_none() {
            context.client_os.init_with(None);
            return;
        }

        let os = self
            .user_agent_detector
            .parse_os(context.user_agent.as_ref().unwrap());

        context.client_os.init_with(Some(os));
    }

    pub fn load_ua(&self, context: &mut FlowRouterContext) {
        if context.client_ua.has_value() {
            return;
        }

        if context.user_agent.is_none() {
            context.client_ua.init_with(None);
            return;
        }

        let ua = self
            .user_agent_detector
            .parse_user_agent(context.user_agent.as_ref().unwrap());

        context.client_ua.init_with(Some(ua));
    }

    pub fn load_device(&self, context: &mut FlowRouterContext) {
        if context.client_device.has_value() {
            return;
        }

        if context.user_agent.is_none() {
            context.client_device.init_with(None);
            return;
        }

        let device = self
            .user_agent_detector
            .parse_device(context.user_agent.as_ref().unwrap());

        context.client_device.init_with(Some(device));
    }

    fn replace_debug_data(&self, context: &mut FlowRouterContext) {
        if !self.allow_debug(context) {
            return;
        }

        context.host = self.host_extractor.detect(&context.request, true);
        context.protocol = self.protocol_extractor.detect(context.request, true);
        context.client_ip = self.ip_extractor.detect(&context.request, true);
        context.user_agent = self
            .user_agent_string_extractor
            .detect(&context.request, true);
        context.client_langs = self.language_extractor.detect(&context.request, true);
    }

    fn build_context<'a>(
        &self,
        req: &'a RequestType<'a>,
        res: &'a ResponseType<'a>,
    ) -> FlowRouterContext<'a> {
        let mut context = FlowRouterContext {
            id: self.generate_request_id(),
            utc: Utc::now(),
            data: HashMap::new(),
            client_os: InitOnce::default(None),
            client_ua: InitOnce::default(None),
            client_device: InitOnce::default(None),
            client_country: InitOnce::default(None),
            current_step: FlowStep::Initial,
            in_route: self.build_route_uri(req),
            user_agent: None,
            client_ip: None,
            client_langs: None,
            host: None,
            protocol: None,
            out_route: None,
            main_route: None,
            result: None,
            request: req,
            response: res,
            trace: None,
            hit_trace: None,
        };

        context.host = self.host_extractor.detect(&context.request, false);
        context.protocol = self.protocol_extractor.detect(&context.request, false);
        context.client_ip = self.ip_extractor.detect(&context.request, false);
        context.user_agent = self
            .user_agent_string_extractor
            .detect(&context.request, false);
        context.client_langs = self.language_extractor.detect(&context.request, false);

        context
    }

    /// Main entry point for handling HTTP requests through the flow router
    ///
    /// This method orchestrates the entire request processing pipeline,
    /// from initial parsing through final result generation.
    ///
    /// # Arguments
    /// * `req` - The incoming HTTP request to process
    /// * `res` - The HTTP response being constructed
    ///
    /// # Returns
    /// * `Result<FlowRouterResult>` - The result of processing the request
    ///
    /// # Errors
    /// * Returns an error if any stage of processing fails
    pub async fn handle<'a>(
        &self,
        req: &'a RequestType<'a>,
        res: &'a ResponseType<'a>,
    ) -> Result<FlowRouterResult> {
        let request_timer = Timer::new();
        self.metrics.requests_total.inc();
        self.metrics.active_requests.inc();

        let result = async {
            let context = self.start(req, res).await?;
            Ok(context.result.unwrap())
        }
        .await;

        self.metrics.active_requests.dec();

        match &result {
            Ok(_) => {
                self.metrics.requests_success.inc();
            }
            Err(_) => {
                self.metrics.requests_error.inc();
            }
        }

        // Record total request processing time
        request_timer.observe_duration_seconds(&self.metrics.request_duration);

        // Estimate memory allocations (rough approximation)
        // This is a simplified metric - in production you might use more sophisticated memory tracking
        let estimated_allocations = 15.0; // Base allocations for context, strings, etc.
        self.metrics
            .memory_allocations_per_request
            .observe(estimated_allocations);

        result
    }
}
