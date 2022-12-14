use std::option::Option;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// top level commands
pub enum Command {
    /// Secrets Manager Command
    #[structopt(name = "sec")]
    Secret(SecCommand),
    /// Parameter Store Command
    #[structopt(name = "par")]
    Parameter(ParCommand),
}

/// subcommands for secrets manager command
#[derive(Debug, StructOpt)]
pub enum SecCommand {
    /// list existing secrets
    List {},
    /// get specific secret by name
    Get {
        #[structopt(long)]
        /// the secret name
        name: String,
    },
    /// describe specific secret by name
    Describe {
        #[structopt(long)]
        /// the secret name
        name: String,
    },
    /// create a secret
    Create {
        #[structopt(long)]
        /// the secret name
        name: String,
        #[structopt(long)]
        /// the secret value
        secret: String,
    },
    /// delete specific secret by name
    Delete {
        #[structopt(long)]
        /// the secret name
        name: String,
    },
}

#[derive(Debug, StructOpt)]
pub enum ParCommand {
    /// list existing parameters
    List {},
    /// get specific parameter by name
    Get {
        #[structopt(long)]
        /// the secret name
        name: String,
    },
    /// create a parameter
    Create {
        #[structopt(long)]
        /// the parameter name
        name: String,
        #[structopt(long)]
        /// the parameter value
        value: String,
    },
    /// delete specific parameter by name
    Delete {
        #[structopt(long)]
        /// the parameter name
        name: String,
    },
    /// apply all the parameters in the spec file
    Apply {
        #[structopt(long, default_value = "./templates/parameter_store_template.yaml")]
        path: std::path::PathBuf,
    },
}
