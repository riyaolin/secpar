use secpar::opt::Command;
use structopt::StructOpt;
use tracing::info;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
/// entrance to the CLI commands
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let filter = EnvFilter::from_default_env()
        .add_directive("secpar::cli::=debug".parse()?) // all modules with `cli::` prefix will set to debug level
        .add_directive(LevelFilter::INFO.into());

    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Starting secpar...");

    let cmd: Command = Command::from_args();

    secpar::run(&cmd).await?;

    info!("CLI finished.");

    Ok(())
}
