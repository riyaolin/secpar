use clap::{Args, Parser, Subcommand};

/// secpar CLI
#[derive(Debug, Parser)]
#[command(name = "secpar", version, about)]
pub struct Cli {
    #[command(flatten)]
    pub global: GlobalOpts,
    #[command(subcommand)]
    pub command: Command,
}

/// Global options shared across commands
#[derive(Debug, Args)]
pub struct GlobalOpts {
    /// AWS region (overrides AWS_REGION/AWS_DEFAULT_REGION)
    #[arg(long, env = "AWS_REGION", default_value = "us-east-1")]
    pub region: String,
    /// AWS profile name
    #[arg(long, env = "AWS_PROFILE")]
    pub profile: Option<String>,
}

/// top level commands
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Secrets Manager Command
    #[command(name = "sec")]
    Secret {
        #[command(subcommand)]
        command: SecCommand,
    },
    /// Parameter Store Command
    #[command(name = "par")]
    Parameter {
        #[command(subcommand)]
        command: ParCommand,
    },
}

/// subcommands for secrets manager command
#[derive(Debug, Subcommand)]
pub enum SecCommand {
    /// list existing secrets
    List {},
    /// get specific secret by name (omit --name to select interactively)
    Get {
        /// the secret name
        #[arg(long)]
        name: Option<String>,
    },
    /// describe specific secret by name (omit --name to select interactively)
    Describe {
        /// the secret name
        #[arg(long)]
        name: Option<String>,
    },
    /// create a secret
    Create {
        /// the secret name
        #[arg(long)]
        name: String,
        /// the secret value
        #[arg(long)]
        secret: String,
    },
    /// delete specific secret by name (omit --name to select interactively)
    Delete {
        /// the secret name
        #[arg(long)]
        name: Option<String>,
        /// skip the 7-day recovery window and delete immediately
        #[arg(long, default_value_t = false)]
        force: bool,
    },
}

#[derive(Debug, Subcommand)]
pub enum ParCommand {
    /// list existing parameters
    List {},
    /// get specific parameter by name (omit --name to select interactively)
    Get {
        /// the parameter name
        #[arg(long)]
        name: Option<String>,
    },
    /// create a parameter
    Create {
        /// the parameter name
        #[arg(long)]
        name: String,
        /// the parameter value
        #[arg(long)]
        value: String,
    },
    /// delete specific parameter by name (omit --name to select interactively)
    Delete {
        /// the parameter name
        #[arg(long)]
        name: Option<String>,
    },
    /// apply all the parameters in the spec file
    Apply {
        #[arg(long, default_value = "./templates/parameter_store_template.yaml")]
        path: std::path::PathBuf,
    },
}
