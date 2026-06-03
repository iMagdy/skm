use std::process::Command;

#[test]
fn live_search_react_uses_default_api_endpoint() {
    if std::env::var("KTESIO_RUN_LIVE_SEARCH_E2E").ok().as_deref() != Some("1") {
        eprintln!("skipping live API e2e; set KTESIO_RUN_LIVE_SEARCH_E2E=1 to run");
        return;
    }

    let output = Command::new(env!("CARGO_BIN_EXE_kt"))
        .env_remove("KTESIO_SEARCH_API_URL")
        .env("KTESIO_NO_UPDATE_CHECK", "1")
        .args(["search", "react", "--json"])
        .output()
        .expect("run kt search");

    assert!(
        output.status.success(),
        "kt search failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let json: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("search stdout should be JSON");
    let results = json
        .as_array()
        .expect("kt search --json should print an array");

    if let Some(first) = results.first() {
        assert!(first.get("id").and_then(|value| value.as_str()).is_some());
        assert!(first.get("name").and_then(|value| value.as_str()).is_some());
        assert!(first
            .get("source")
            .and_then(|value| value.as_str())
            .is_some());
        assert!(first
            .get("skill")
            .and_then(|value| value.as_str())
            .is_some());
    }
}
