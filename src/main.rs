use clap::Parser;
use secpar::opt::Cli;
use tracing_subscriber::FmtSubscriber;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

/// Entrance to the CLI commands
#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // Default to ERROR so normal output is clean. Set RUST_LOG=debug for
    // verbose tracing (e.g. RUST_LOG=secpar=debug secpar sec list).
    let filter = EnvFilter::from_default_env().add_directive(LevelFilter::ERROR.into());

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let cli = Cli::parse();

    secpar::run(&cli).await?;

    Ok(())
}
