use crate::errors::SecParError;
use crate::opt::ParCommand;
use aws_sdk_ssm::{Client, Region};
use color_eyre::eyre::eyre;
use color_eyre::Report;
use tokio_stream::StreamExt;
use tracing::{debug, info};

/// list all parameters
async fn list_parameters(client: &Client) -> Result<(), SecParError> {
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

/// get speficied parameter
async fn get_parameter(client: &Client, name: &str) -> Result<String, SecParError> {
    match client
        .get_parameter()
        .name(name)
        .with_decryption(true)
        .send()
        .await
    {
        Ok(output) => Ok(output.parameter().unwrap().value.clone().unwrap()),
        Err(e) => {
            debug!("Error: {:?}", e.to_string());
            Err(SecParError::NotFound(e.to_string()))
        }
    }
}

pub async fn process_par_command(command: &ParCommand) -> Result<(), Report> {
    let region_provider = Region::new("us-east-1");
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);
    match command {
        ParCommand::List {} => {
            list_parameters(&client).await?;
        }
        ParCommand::Get { name } => match get_parameter(&client, &name).await {
            Ok(par_value) => {
                info!("Got parameter[{}] value:", name);
                info!("  {}", par_value);
            }
            Err(_) => {
                return Err(eyre!("Failed to retrieve parameter value"));
            }
        },
    }
    Ok(())
}
