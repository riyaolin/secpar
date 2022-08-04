use crate::errors::SecParError;
use crate::opt::ParCommand;
use aws_sdk_ssm::{Client, Region};
use color_eyre::Report;
use tokio_stream::StreamExt;
use tracing::{debug, info};

/// list all parameters
pub async fn list_parameters(client: &Client) -> Result<(), SecParError> {
    let mut pars_stream = client.describe_parameters().into_paginator().send();
    let mut p_count = 1;
    while let Some(par_output) = pars_stream.next().await {
        debug!("New Parameter Page!");
        for parameter in par_output.unwrap().parameters().unwrap_or_default() {
            info!(
                "Parameter[{}|{}]: {}",
                p_count,
                parameter.r#type().unwrap().as_str(),
                parameter.name().unwrap_or("No name!")
            );
            p_count += 1;
        }
    }
    Ok(())
}

pub async fn process_par_command(command: &ParCommand) -> Result<(), Report> {
    let region_provider = Region::new("us-east-1");
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);
    match command {
        ParCommand::List {} => {
            list_parameters(&client).await?;
        }
        ParCommand::Get { name: _ } => {
            todo!();
        }
    }
    Ok(())
}
