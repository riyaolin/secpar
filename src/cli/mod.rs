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
