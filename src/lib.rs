//! # secpar
//!
//! A CLI tool that wraps the AWS Rust SDK for **Secrets Manager** (`sec`) and
//! **Parameter Store** (`par`), adding interactive selection menus, spinners,
//! confirmation prompts, and formatted tables.
//!
//! ## Authentication
//!
//! Credentials are resolved in the following order (standard AWS credential
//! chain):
//!
//! 1. Environment variables (`AWS_ACCESS_KEY_ID` / `AWS_SECRET_ACCESS_KEY`)
//! 2. AWS credentials file (`~/.aws/credentials`)
//! 3. AWS config file (`~/.aws/config`)
//! 4. IAM instance / task / IRSA roles (EC2, ECS, EKS)
//!
//! ## Global options
//!
//! | Flag | Env var | Default |
//! |------|---------|---------|
//! | `--region` | `AWS_REGION` | `us-east-1` |
//! | `--profile` | `AWS_PROFILE` | *(none)* |
//!
//! ## CLI examples
//!
//! ```text
//! # Show active AWS environment
//! secpar env
//!
//! # Secrets Manager
//! secpar sec list
//! secpar sec get --name my-secret
//! secpar sec get                        # interactive selection
//! secpar sec describe --name my-secret
//! secpar sec create --name my-secret --secret '{"key":"value"}'
//! secpar sec create --name my-secret --secret '{"key":"value"}' --yes  # skip confirmation
//! secpar sec delete --name my-secret
//! secpar sec delete                     # interactive selection + confirmation
//! secpar sec apply --path ./templates/secrets_template.yaml
//!
//! # Parameter Store
//! secpar par list
//! secpar par get --name /my/param
//! secpar par get                        # interactive selection
//! secpar par create --name /my/param --value secret-value
//! secpar par delete --name /my/param
//! secpar par delete                     # interactive selection + confirmation
//! secpar par apply --path ./templates/parameter_store_template.yaml
//!
//! # Region / profile override
//! secpar --region eu-west-1 --profile staging env
//! secpar --region eu-west-1 --profile staging sec list
//! ```

use color_eyre::Result;
/// CLI commands
pub mod cli;
/// a collection of custom errors
pub mod errors;
/// command line options
pub mod opt;
/// spec file
pub mod specs;
/// shared UX primitives (spinners, prompts, tables)
pub mod ui;
/// util methods
pub mod util;

use crate::opt::{Cli, Command};

/// The entry point of this CLI tool.
pub async fn run(cli: &Cli) -> Result<()> {
    match &cli.command {
        Command::Secret { command } => {
            cli::sec::process_sec_command(command, &cli.global).await?;
        }
        Command::Parameter { command } => {
            cli::par::process_par_command(command, &cli.global).await?;
        }
        Command::Env => {
            cli::show_env(&cli.global);
        }
    }
    Ok(())
}
