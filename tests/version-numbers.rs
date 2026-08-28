#[test]
fn test_readme_deps_updated() {
    version_sync::assert_markdown_deps_updated!("README.md");
}

#[test]
fn test_readme_mentions_version() {
    version_sync::assert_contains_substring!("README.md", "Version {version}");
}

#[test]
fn test_html_root_url() {
    version_sync::assert_html_root_url_updated!("src/lib.rs");
}

/// Three-way version invariant: the crate version's major, the
/// `current_gads_version` alias's version in src/lib.rs, and the README's
/// "Google Ads API vNN" mention must all agree. A drifted sed or manual
/// Cargo bump fails here loudly.
#[test]
fn test_version_three_way_consistency() {
    let cargo_version = env!("CARGO_PKG_VERSION");
    let cargo_major: u32 = env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap();

    let lib_rs = std::fs::read_to_string("src/lib.rs").expect("src/lib.rs readable");

    // The alias re-export line: pub use google::ads::googleads::vNN as current_gads_version;
    let alias_line = lib_rs
        .lines()
        .find(|l| l.contains(" as current_gads_version;"))
        .expect("current_gads_version alias line not found in src/lib.rs");
    let alias_lhs = alias_line
        .split(" as current_gads_version;")
        .next()
        .unwrap_or("");
    let alias_segment = alias_lhs.rsplit("::").next().unwrap_or("").trim();
    let alias_major: u32 = alias_segment
        .strip_prefix('v')
        .and_then(|v| v.parse().ok())
        .expect("alias line does not carry a vNN module version");
    assert_eq!(
        cargo_major, alias_major,
        "alias in src/lib.rs is v{alias_major} but Cargo.toml major is {cargo_major}; \
         repoint the alias line (the single hand-edit anchor) or fix the Cargo version"
    );

    // README's stated Google Ads API version: "Google Ads API vMAJOR.MINOR".
    let readme = std::fs::read_to_string("README.md").expect("README.md readable");
    let api_mention = readme
        .lines()
        .find(|l| l.contains("Google Ads API v"))
        .expect("no 'Google Ads API vNN' mention in README.md");
    let api_major: u32 = api_mention
        .split("Google Ads API v")
        .nth(1)
        .and_then(|rest| rest.split('.').next())
        .and_then(|v| v.parse().ok())
        .expect("README API mention does not start with a numeric major");
    assert_eq!(
        cargo_major, api_major,
        "README says Google Ads API v{api_major} but Cargo.toml major is {cargo_major}; \
         run utils/update.sh to keep them in sync"
    );

    // Sanity: README also states the full crate version.
    assert!(
        readme.contains(cargo_version),
        "README does not mention current crate version {cargo_version}"
    );
}
