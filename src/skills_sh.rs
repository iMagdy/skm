use std::collections::HashMap;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::error::SearchFailed;

const DEFAULT_SEARCH_API_URL: &str = "https://api.ktesio.dev/search-skills";
const MAX_ATTEMPTS: usize = 3;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SkillSearchResult {
    pub id: String,
    pub name: String,
    pub source: String,
    pub skill: String,
    pub repo: Option<String>,
    pub installs: u64,
    pub url: Option<String>,
    pub install_target: Option<String>,
    pub installable: bool,
    #[serde(default)]
    pub stars: Option<u64>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[cfg(not(tarpaulin_include))]
pub fn search<F>(
    query: &str,
    limit: usize,
    mut notify: F,
) -> Result<Vec<SkillSearchResult>, Box<dyn std::error::Error>>
where
    F: FnMut(String),
{
    let base_url = std::env::var("KTESIO_SEARCH_API_URL")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| DEFAULT_SEARCH_API_URL.to_string());
    let transport = UreqTransport::new();
    let sleeper = ThreadSleeper;
    search_with_transport(&transport, &sleeper, &base_url, query, limit, &mut notify)
}

#[cfg(tarpaulin_include)]
pub fn search<F>(
    _query: &str,
    _limit: usize,
    _notify: F,
) -> Result<Vec<SkillSearchResult>, Box<dyn std::error::Error>>
where
    F: FnMut(String),
{
    Err(SearchFailed {
        message: "Live Ktesio search API is disabled during coverage runs".to_string(),
    }
    .into())
}

fn search_with_transport<T, S, F>(
    transport: &T,
    sleeper: &S,
    base_url: &str,
    query: &str,
    limit: usize,
    notify: &mut F,
) -> Result<Vec<SkillSearchResult>, Box<dyn std::error::Error>>
where
    T: HttpTransport,
    S: Sleeper,
    F: FnMut(String),
{
    let query = query.trim();
    if query.len() < 2 {
        return Err(SearchFailed {
            message: "Search query must be at least 2 characters".to_string(),
        }
        .into());
    }

    let limit = normalized_limit(limit);
    let url = search_url(base_url, query, limit)?;

    for attempt in 1..=MAX_ATTEMPTS {
        match transport.get(&url) {
            Ok(response) if response.status == 200 => return parse_search_response(&response.body),
            Ok(response) if is_retryable_status(response.status) => {
                if attempt == MAX_ATTEMPTS {
                    return Err(retry_exhausted(response.status, &response.headers).into());
                }

                let delay = retry_delay(&response.headers, response.status, attempt);
                notify(retry_message(response.status, delay, attempt + 1));
                sleeper.sleep(delay);
            }
            Ok(response) => return Err(non_retryable_status(response).into()),
            Err(error) if error.retryable => {
                if attempt == MAX_ATTEMPTS {
                    return Err(SearchFailed {
                        message: format!(
                            "Ktesio search API could not be reached after 3 attempts. Last error: {}",
                            error.message
                        ),
                    }
                    .into());
                }

                let delay = backoff_delay(attempt);
                notify(format!(
                    "Ktesio search API connection failed; retrying in {}s (attempt {}/{MAX_ATTEMPTS}).",
                    display_seconds(delay),
                    attempt + 1
                ));
                sleeper.sleep(delay);
            }
            Err(error) => {
                return Err(SearchFailed {
                    message: format!("Ktesio search API failed: {}", error.message),
                }
                .into())
            }
        }
    }

    Err(SearchFailed {
        message: "Ktesio search API failed unexpectedly".to_string(),
    }
    .into())
}

fn search_url(base_url: &str, query: &str, limit: usize) -> Result<String, SearchFailed> {
    let mut separator = "?";
    if base_url.contains('?') {
        separator = "&";
    }
    let encoded = urlencoding::encode(query);
    Ok(format!(
        "{}{}q={encoded}&limit={limit}",
        base_url.trim_end_matches('&'),
        separator
    ))
}

fn normalized_limit(limit: usize) -> usize {
    limit.clamp(1, 100)
}

fn parse_search_response(body: &str) -> Result<Vec<SkillSearchResult>, Box<dyn std::error::Error>> {
    if let Ok(response) = serde_json::from_str::<SearchApiResponse>(body) {
        return Ok(response.data);
    }

    if let Ok(results) = serde_json::from_str::<Vec<SkillSearchResult>>(body) {
        return Ok(results);
    }

    Err(SearchFailed {
        message: "Ktesio search API returned an unexpected search response".to_string(),
    }
    .into())
}

fn is_retryable_status(status: u16) -> bool {
    status == 429 || status >= 500
}

fn retry_delay(headers: &HashMap<String, String>, status: u16, attempt: usize) -> Duration {
    if status == 429 {
        if let Some(seconds) = header_seconds(headers, "retry-after") {
            return Duration::from_secs(seconds.max(1));
        }
        if let Some(seconds) = header_seconds(headers, "x-ratelimit-reset") {
            return Duration::from_secs(seconds.max(1));
        }
    }

    backoff_delay(attempt)
}

fn header_seconds(headers: &HashMap<String, String>, name: &str) -> Option<u64> {
    headers
        .get(name)
        .and_then(|value| value.trim().parse::<u64>().ok())
}

fn backoff_delay(attempt: usize) -> Duration {
    let seconds = 1_u64
        .checked_shl((attempt.saturating_sub(1)) as u32)
        .unwrap_or(8);
    let jitter_ms = ((attempt as u64) * 137) % 250;
    Duration::from_secs(seconds.min(8)) + Duration::from_millis(jitter_ms)
}

fn retry_message(status: u16, delay: Duration, next_attempt: usize) -> String {
    match status {
        429 => format!(
            "Ktesio search API rate limit reached; retrying in {}s (attempt {next_attempt}/{MAX_ATTEMPTS}).",
            display_seconds(delay)
        ),
        503 => format!(
            "Ktesio search API is temporarily unavailable; retrying in {}s (attempt {next_attempt}/{MAX_ATTEMPTS}).",
            display_seconds(delay)
        ),
        _ => format!(
            "Ktesio search API failed temporarily; retrying in {}s (attempt {next_attempt}/{MAX_ATTEMPTS}).",
            display_seconds(delay)
        ),
    }
}

fn display_seconds(duration: Duration) -> u64 {
    duration.as_secs().max(1)
}

fn retry_exhausted(status: u16, headers: &HashMap<String, String>) -> SearchFailed {
    let retry_hint = retry_delay(headers, status, MAX_ATTEMPTS);
    match status {
        429 => SearchFailed {
            message: format!(
                "Ktesio search API rate limit reached after 3 attempts. Please retry in about {}s.",
                display_seconds(retry_hint)
            ),
        },
        503 => SearchFailed {
            message:
                "Ktesio search API is temporarily unavailable after 3 attempts. Please retry later."
                    .to_string(),
        },
        _ => SearchFailed {
            message: "Ktesio search API failed after 3 attempts. Please retry later.".to_string(),
        },
    }
}

fn non_retryable_status(response: HttpResponse) -> SearchFailed {
    match response.status {
        400 => SearchFailed {
            message:
                "Ktesio search API rejected the search query. Try a different query or lower limit."
                    .to_string(),
        },
        403 => SearchFailed {
            message: "Ktesio search API rejected this client. Please update kt and retry."
                .to_string(),
        },
        404 => SearchFailed {
            message: "Ktesio search API endpoint was not found. Please update kt.".to_string(),
        },
        status => SearchFailed {
            message: format!("Ktesio search API failed with HTTP status {status}."),
        },
    }
}

#[derive(Debug, Clone)]
struct HttpResponse {
    status: u16,
    headers: HashMap<String, String>,
    body: String,
}

#[derive(Debug, Clone)]
struct HttpTransportError {
    message: String,
    retryable: bool,
}

trait HttpTransport {
    fn get(&self, url: &str) -> Result<HttpResponse, HttpTransportError>;
}

trait Sleeper {
    fn sleep(&self, duration: Duration);
}

#[cfg(not(tarpaulin_include))]
struct ThreadSleeper;

#[cfg(not(tarpaulin_include))]
impl Sleeper for ThreadSleeper {
    fn sleep(&self, duration: Duration) {
        std::thread::sleep(duration);
    }
}

#[cfg(not(tarpaulin_include))]
struct UreqTransport {
    agent: ureq::Agent,
}

#[cfg(not(tarpaulin_include))]
impl UreqTransport {
    fn new() -> Self {
        Self {
            agent: ureq::AgentBuilder::new()
                .timeout(Duration::from_secs(15))
                .build(),
        }
    }
}

#[cfg(not(tarpaulin_include))]
impl HttpTransport for UreqTransport {
    fn get(&self, url: &str) -> Result<HttpResponse, HttpTransportError> {
        let request = self
            .agent
            .get(url)
            .set("Accept", "application/json")
            .set("User-Agent", client_user_agent())
            .set("X-Ktesio-Client", client_header_value());

        match request.call() {
            Ok(response) => response_to_http(response),
            Err(ureq::Error::Status(_, response)) => response_to_http(response),
            Err(ureq::Error::Transport(error)) => Err(HttpTransportError {
                message: friendly_transport_error(&error.to_string()),
                retryable: true,
            }),
        }
    }
}

fn client_user_agent() -> &'static str {
    concat!("ktesio/", env!("CARGO_PKG_VERSION"))
}

fn client_header_value() -> &'static str {
    "kt-cli"
}

fn friendly_transport_error(message: &str) -> String {
    let lower = message.to_ascii_lowercase();
    if lower.contains("dns") || lower.contains("resolve") || lower.contains("lookup") {
        "DNS lookup failed".to_string()
    } else if lower.contains("timed out") || lower.contains("timeout") {
        "connection timed out".to_string()
    } else if lower.contains("connection refused") {
        "connection refused".to_string()
    } else {
        "temporary network error".to_string()
    }
}

#[cfg(not(tarpaulin_include))]
fn response_to_http(response: ureq::Response) -> Result<HttpResponse, HttpTransportError> {
    let status = response.status();
    let headers = interesting_headers(&response);
    let body = response.into_string().map_err(|error| HttpTransportError {
        message: error.to_string(),
        retryable: false,
    })?;

    Ok(HttpResponse {
        status,
        headers,
        body,
    })
}

#[cfg(not(tarpaulin_include))]
fn interesting_headers(response: &ureq::Response) -> HashMap<String, String> {
    ["Retry-After", "X-RateLimit-Reset"]
        .into_iter()
        .filter_map(|name| {
            response
                .header(name)
                .map(|value| (name.to_ascii_lowercase(), value.to_string()))
        })
        .collect()
}

#[derive(Debug, Deserialize)]
struct SearchApiResponse {
    data: Vec<SkillSearchResult>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};

    struct FakeTransport {
        responses: RefCell<Vec<Result<HttpResponse, HttpTransportError>>>,
        attempts: Cell<usize>,
        urls: RefCell<Vec<String>>,
    }

    impl FakeTransport {
        fn new(responses: Vec<Result<HttpResponse, HttpTransportError>>) -> Self {
            Self {
                responses: RefCell::new(responses),
                attempts: Cell::new(0),
                urls: RefCell::new(Vec::new()),
            }
        }
    }

    impl HttpTransport for FakeTransport {
        fn get(&self, url: &str) -> Result<HttpResponse, HttpTransportError> {
            self.attempts.set(self.attempts.get() + 1);
            self.urls.borrow_mut().push(url.to_string());
            self.responses.borrow_mut().remove(0)
        }
    }

    struct FakeSleeper {
        delays: RefCell<Vec<Duration>>,
    }

    impl FakeSleeper {
        fn new() -> Self {
            Self {
                delays: RefCell::new(Vec::new()),
            }
        }
    }

    impl Sleeper for FakeSleeper {
        fn sleep(&self, duration: Duration) {
            self.delays.borrow_mut().push(duration);
        }
    }

    fn response(status: u16, body: &str) -> HttpResponse {
        HttpResponse {
            status,
            headers: HashMap::new(),
            body: body.to_string(),
        }
    }

    fn response_with_header(status: u16, name: &str, value: &str) -> HttpResponse {
        let mut headers = HashMap::new();
        headers.insert(name.to_string(), value.to_string());
        HttpResponse {
            status,
            headers,
            body: "{}".to_string(),
        }
    }

    fn skill(name: &str) -> SkillSearchResult {
        SkillSearchResult {
            id: format!("example/skills/{name}"),
            name: name.to_string(),
            source: "example/skills".to_string(),
            skill: name.to_string(),
            repo: Some("https://github.com/example/skills.git".to_string()),
            installs: 42,
            url: Some(format!("https://skillsmp.com/skills/{name}")),
            install_target: Some(format!("example/skills/{name}")),
            installable: true,
            stars: Some(7),
            description: Some("Example skill".to_string()),
            updated_at: Some("2026-06-03T00:00:00Z".to_string()),
        }
    }

    #[test]
    fn test_search_uses_ktesio_api_envelope() {
        let body = serde_json::json!({
            "data": [skill("react")],
            "meta": {
                "provider": "skillsmp",
                "cache": "miss",
                "query": "react",
                "limit": 5,
                "page": 1,
                "count": 1
            }
        })
        .to_string();
        let transport = FakeTransport::new(vec![Ok(response(200, &body))]);
        let sleeper = FakeSleeper::new();
        let mut messages = Vec::new();

        let results = search_with_transport(
            &transport,
            &sleeper,
            DEFAULT_SEARCH_API_URL,
            "react",
            5,
            &mut |message| messages.push(message),
        )
        .unwrap();

        assert_eq!(results, vec![skill("react")]);
        assert!(transport.urls.borrow()[0].starts_with(DEFAULT_SEARCH_API_URL));
        assert!(transport.urls.borrow()[0].contains("q=react"));
        assert!(transport.urls.borrow()[0].contains("limit=5"));
        assert!(messages.is_empty());
    }

    #[test]
    fn test_required_client_headers_are_defined() {
        assert!(client_user_agent().starts_with("ktesio/"));
        assert_eq!(client_header_value(), "kt-cli");
    }

    #[test]
    fn test_search_supports_plain_array_for_compatibility() {
        let body = serde_json::to_string(&vec![skill("react")]).unwrap();
        let transport = FakeTransport::new(vec![Ok(response(200, &body))]);
        let sleeper = FakeSleeper::new();
        let mut messages = Vec::new();

        let results = search_with_transport(
            &transport,
            &sleeper,
            DEFAULT_SEARCH_API_URL,
            "react",
            5,
            &mut |message| messages.push(message),
        )
        .unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].skill, "react");
    }

    #[test]
    fn test_rate_limit_retries_three_total_attempts_with_retry_after() {
        let success = r#"{"data":[],"meta":{"provider":"cache","cache":"miss","query":"tests","limit":10,"page":1,"count":0}}"#;
        let transport = FakeTransport::new(vec![
            Ok(response_with_header(429, "retry-after", "3")),
            Ok(response_with_header(429, "x-ratelimit-reset", "4")),
            Ok(response(200, success)),
        ]);
        let sleeper = FakeSleeper::new();
        let mut messages = Vec::new();

        let results = search_with_transport(
            &transport,
            &sleeper,
            DEFAULT_SEARCH_API_URL,
            "tests",
            10,
            &mut |message| messages.push(message),
        )
        .unwrap();

        assert!(results.is_empty());
        assert_eq!(transport.attempts.get(), 3);
        assert_eq!(sleeper.delays.borrow()[0], Duration::from_secs(3));
        assert_eq!(sleeper.delays.borrow()[1], Duration::from_secs(4));
        assert!(messages[0].contains("rate limit reached"));
    }

    #[test]
    fn test_retryable_transport_error_retries() {
        let success = r#"{"data":[],"meta":{"provider":"cache","cache":"miss","query":"tests","limit":10,"page":1,"count":0}}"#;
        let transport = FakeTransport::new(vec![
            Err(HttpTransportError {
                message: "timeout".to_string(),
                retryable: true,
            }),
            Ok(response(200, success)),
        ]);
        let sleeper = FakeSleeper::new();
        let mut messages = Vec::new();

        let results = search_with_transport(
            &transport,
            &sleeper,
            DEFAULT_SEARCH_API_URL,
            "tests",
            10,
            &mut |message| messages.push(message),
        )
        .unwrap();

        assert!(results.is_empty());
        assert_eq!(transport.attempts.get(), 2);
        assert_eq!(sleeper.delays.borrow().len(), 1);
        assert!(messages[0].contains("connection failed"));
    }

    #[test]
    fn test_non_retryable_status_fails_once() {
        let transport = FakeTransport::new(vec![Ok(response(403, "{}"))]);
        let sleeper = FakeSleeper::new();
        let mut messages = Vec::new();

        let result = search_with_transport(
            &transport,
            &sleeper,
            DEFAULT_SEARCH_API_URL,
            "tests",
            10,
            &mut |message| messages.push(message),
        );

        assert!(result.is_err());
        assert_eq!(transport.attempts.get(), 1);
        assert!(sleeper.delays.borrow().is_empty());
        assert!(messages.is_empty());
    }

    #[test]
    fn test_malformed_success_response_does_not_retry() {
        let transport = FakeTransport::new(vec![Ok(response(200, "not json"))]);
        let sleeper = FakeSleeper::new();
        let mut messages = Vec::new();

        let result = search_with_transport(
            &transport,
            &sleeper,
            DEFAULT_SEARCH_API_URL,
            "tests",
            10,
            &mut |message| messages.push(message),
        );

        assert!(result.is_err());
        assert_eq!(transport.attempts.get(), 1);
        assert!(result.unwrap_err().to_string().contains("unexpected"));
    }

    #[test]
    fn test_short_query_fails_without_request() {
        let transport = FakeTransport::new(Vec::new());
        let sleeper = FakeSleeper::new();
        let mut messages = Vec::new();

        let result = search_with_transport(
            &transport,
            &sleeper,
            DEFAULT_SEARCH_API_URL,
            " x ",
            10,
            &mut |message| messages.push(message),
        );

        assert!(result.is_err());
        assert_eq!(transport.attempts.get(), 0);
        assert!(messages.is_empty());
    }

    #[test]
    fn test_limit_is_capped_and_base_url_with_query_is_supported() {
        let transport = FakeTransport::new(vec![Ok(response(
            200,
            r#"{"data":[],"meta":{"provider":"cache","cache":"miss","query":"tests","limit":100,"page":1,"count":0}}"#,
        ))]);
        let sleeper = FakeSleeper::new();
        let mut messages = Vec::new();

        search_with_transport(
            &transport,
            &sleeper,
            "https://example.test/search-skills?debug=1",
            "tests",
            500,
            &mut |message| messages.push(message),
        )
        .unwrap();

        assert!(transport.urls.borrow()[0].contains("debug=1&q=tests&limit=100"));
    }

    #[test]
    fn test_service_unavailable_exhaustion_uses_clean_error() {
        let transport = FakeTransport::new(vec![
            Ok(response(503, "{}")),
            Ok(response(503, "{}")),
            Ok(response(503, "{}")),
        ]);
        let sleeper = FakeSleeper::new();
        let mut messages = Vec::new();

        let result = search_with_transport(
            &transport,
            &sleeper,
            DEFAULT_SEARCH_API_URL,
            "tests",
            10,
            &mut |message| messages.push(message),
        );

        assert!(result.is_err());
        assert_eq!(transport.attempts.get(), 3);
        assert_eq!(sleeper.delays.borrow().len(), 2);
        assert!(messages[0].contains("temporarily unavailable"));
        assert!(result.unwrap_err().to_string().contains("retry later"));
    }

    #[test]
    fn test_retryable_transport_error_exhaustion_mentions_last_error() {
        let transport = FakeTransport::new(vec![
            Err(HttpTransportError {
                message: "timeout".to_string(),
                retryable: true,
            }),
            Err(HttpTransportError {
                message: "timeout".to_string(),
                retryable: true,
            }),
            Err(HttpTransportError {
                message: "connection refused".to_string(),
                retryable: true,
            }),
        ]);
        let sleeper = FakeSleeper::new();
        let mut messages = Vec::new();

        let result = search_with_transport(
            &transport,
            &sleeper,
            DEFAULT_SEARCH_API_URL,
            "tests",
            10,
            &mut |message| messages.push(message),
        );

        assert!(result.is_err());
        assert_eq!(transport.attempts.get(), 3);
        assert_eq!(messages.len(), 2);
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("connection refused"));
    }

    #[test]
    fn test_non_retryable_transport_error_fails_once() {
        let transport = FakeTransport::new(vec![Err(HttpTransportError {
            message: "bad response body".to_string(),
            retryable: false,
        })]);
        let sleeper = FakeSleeper::new();
        let mut messages = Vec::new();

        let result = search_with_transport(
            &transport,
            &sleeper,
            DEFAULT_SEARCH_API_URL,
            "tests",
            10,
            &mut |message| messages.push(message),
        );

        assert!(result.is_err());
        assert_eq!(transport.attempts.get(), 1);
        assert!(sleeper.delays.borrow().is_empty());
        assert!(messages.is_empty());
    }

    #[test]
    fn test_retry_delay_and_messages_cover_fallbacks() {
        let mut headers = HashMap::new();
        headers.insert("retry-after".to_string(), "not-a-number".to_string());

        let delay = retry_delay(&headers, 429, 2);

        assert!(delay >= Duration::from_secs(2));
        assert!(retry_message(429, Duration::from_secs(12), 2).contains("rate limit"));
        assert!(retry_message(503, Duration::from_secs(2), 3).contains("temporarily unavailable"));
        assert!(retry_message(500, Duration::from_secs(2), 3).contains("failed temporarily"));
        assert_eq!(display_seconds(Duration::from_millis(5)), 1);
    }

    #[test]
    fn test_retry_exhausted_status_messages() {
        let mut headers = HashMap::new();
        headers.insert("retry-after".to_string(), "7".to_string());

        assert!(retry_exhausted(429, &headers)
            .to_string()
            .contains("retry in about 7s"));
        assert!(retry_exhausted(503, &HashMap::new())
            .to_string()
            .contains("temporarily unavailable"));
        assert!(retry_exhausted(500, &HashMap::new())
            .to_string()
            .contains("retry later"));
    }

    #[test]
    fn test_non_retryable_status_messages() {
        let statuses = [
            (400, "rejected the search query"),
            (403, "rejected this client"),
            (404, "endpoint was not found"),
            (418, "HTTP status 418"),
        ];

        for (status, expected) in statuses {
            let error = non_retryable_status(response(status, "{}"));
            assert!(
                error.to_string().contains(expected),
                "{} did not contain {}",
                error,
                expected
            );
        }
    }

    #[test]
    fn test_friendly_transport_error_variants() {
        assert_eq!(
            friendly_transport_error("DNS lookup failed"),
            "DNS lookup failed"
        );
        assert_eq!(
            friendly_transport_error("operation timed out"),
            "connection timed out"
        );
        assert_eq!(
            friendly_transport_error("connection refused by peer"),
            "connection refused"
        );
        assert_eq!(
            friendly_transport_error("unexpected reset"),
            "temporary network error"
        );
    }

    #[cfg(tarpaulin_include)]
    #[test]
    fn test_live_search_is_disabled_for_coverage() {
        let result = search("tests", 10, |_| {});

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("disabled during coverage runs"));
    }
}
