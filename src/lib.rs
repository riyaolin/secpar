use color_eyre::Report;
/// CLI commands
pub mod cli;
/// a collection of custom errors
pub mod errors;
/// command line options
pub mod opt;

/// Structopts
use crate::opt::Command;

/// the entry point of this cli tool
pub async fn run(cmd: &Command) -> Result<(), Report> {
    match &cmd {
        Command::Secret(sec_cmd) => {
            cli::sec::process_sec_command(sec_cmd).await?;
        }
        Command::Parameter(par_cmd) => {
            cli::par::process_par_command(par_cmd).await?;
        }
    }
    Ok(())
}
