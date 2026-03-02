use crate::opt::GlobalOpts;
use aws_types::region::Region;

/// par cli
pub mod par;
/// sec cli
pub mod sec;

/// Build shared AWS config using global CLI options.
pub async fn load_shared_config(opts: &GlobalOpts) -> aws_types::SdkConfig {
    let region = Region::new(opts.region.clone());
    let mut loader = aws_config::from_env().region(region);
    if let Some(profile) = opts.profile.as_deref() {
        loader = loader.profile_name(profile);
    }
    loader.load().await
}

/// Returns a compact human-readable description of the active AWS environment.
///
/// Includes region, optional profile, and the endpoint URL when
/// `AWS_ENDPOINT_URL` is set (e.g. a LocalStack instance).
///
/// # Examples
///
/// ```
/// use secpar::cli::env_context;
/// use secpar::opt::GlobalOpts;
///
/// let opts = GlobalOpts { region: "us-east-1".into(), profile: None };
/// assert_eq!(env_context(&opts), "🌍 us-east-1");
///
/// let opts_profile = GlobalOpts { region: "eu-west-1".into(), profile: Some("staging".into()) };
/// assert_eq!(env_context(&opts_profile), "🌍 eu-west-1  👤 staging");
/// ```
pub fn env_context(opts: &GlobalOpts) -> String {
    let mut parts = vec![format!("🌍 {}", opts.region)];
    if let Some(profile) = &opts.profile {
        parts.push(format!("👤 {profile}"));
    }
    if let Ok(endpoint) = std::env::var("AWS_ENDPOINT_URL") {
        parts.push(format!("🔌 {endpoint}"));
    }
    parts.join("  ")
}

/// Prints a full bordered panel showing all active AWS environment fields.
///
/// Displays region, and optionally profile and endpoint when set.
/// Called by the `secpar env` command.
///
/// # Arguments
///
/// * `opts` - Global CLI options supplying region and profile.
///
/// # Examples
///
/// ```no_run
/// use secpar::cli::show_env;
/// use secpar::opt::GlobalOpts;
///
/// let opts = GlobalOpts { region: "us-east-1".into(), profile: None };
/// show_env(&opts);
/// // ┌────────────────────────────┐
/// // │      AWS Environment       │
/// // ├────────────────────────────┤
/// // │  Region   :  🌍 us-east-1  │
/// // └────────────────────────────┘
/// ```
pub fn show_env(opts: &GlobalOpts) {
    let rows: Vec<(&str, String)> = {
        let mut v = vec![("Region ", format!("🌍 {}", opts.region))];
        if let Some(profile) = &opts.profile {
            v.push(("Profile", format!("👤 {profile}")));
        }
        if let Ok(endpoint) = std::env::var("AWS_ENDPOINT_URL") {
            v.push(("Endpoint", format!("🔌 {endpoint}")));
        }
        v
    };

    let value_width = rows
        .iter()
        .map(|(_, v)| v.chars().count())
        .max()
        .unwrap_or(0);
    let label_width = rows
        .iter()
        .map(|(l, _)| l.chars().count())
        .max()
        .unwrap_or(0);
    // inner: "  <label>  :  <value>  "
    let inner_width = 2 + label_width + 2 + 1 + 2 + value_width + 2;
    let border = "─".repeat(inner_width);

    println!("┌{border}┐");
    println!("│{:^inner_width$}│", "  AWS Environment  ");
    println!("├{border}┤");
    for (label, value) in &rows {
        let line = format!("  {label:<label_width$}  :  {value}  ");
        // pad to inner_width (value may contain multi-byte emoji; pad by char count)
        let pad = inner_width.saturating_sub(line.chars().count());
        println!("│{line}{:pad$}│", "");
    }
    println!("└{border}┘");
}
