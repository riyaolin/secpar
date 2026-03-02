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
