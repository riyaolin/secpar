use clap::Parser;
use secpar::opt::Cli;
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

/// Entrance to the CLI commands
#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let filter = EnvFilter::from_default_env()
        // all modules with `cli::` prefix will set to debug level
        .add_directive("secpar::cli::=debug".parse()?)
        .add_directive(LevelFilter::INFO.into());

    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Starting secpar...");

    let cli = Cli::parse();

    secpar::run(&cli).await?;

    info!("CLI finished.");

    Ok(())
}
