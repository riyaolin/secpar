use crate::errors::SecParError;
use crate::opt::ParCommand;
use crate::specs::ParameterStore;
use aws_sdk_ssm::{model::ParameterType, Client, Region};
use color_eyre::eyre::eyre;
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

/// get speficied parameter
pub async fn get_parameter(client: &Client, name: &str) -> Result<String, SecParError> {
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
            Err(SecParError::NotFound(name.to_string()))
        }
    }
}

pub async fn create_parameter(client: &Client, name: &str, value: &str) -> Result<(), SecParError> {
    match client
        .put_parameter()
        .overwrite(true)
        .r#type(ParameterType::SecureString)
        .name(name)
        .value(value)
        .send()
        .await
    {
        Ok(_output) => {
            debug!("created");
            Ok(())
        }
        Err(e) => {
            debug!("Error: {:?}", e.to_string());
            Err(SecParError::CreateFail(e.to_string()))
        }
    }
}

/// delete the specified parameter
pub async fn delete_parameter(client: &Client, name: &str) -> Result<(), SecParError> {
    match client.delete_parameter().name(name).send().await {
        Ok(output) => {
            info!("To be deleted parameter: {}", name);
            info!("{:?}", output);
            Ok(())
        }
        Err(e) => {
            debug!("Error: {:?}", e.to_string());
            Err(SecParError::NotFound(name.to_string()))
        }
    }
}

/// apply all the parameters in the given path
pub async fn apply_parameters(client: &Client, path: &std::path::Path) -> Result<(), SecParError> {
    let spec_content = ParameterStore::new(path)?;
    for parameter_pair in spec_content.parameters {
        if let Some(tuple) = parameter_pair.split_once(':') {
            info!("name: {}, value: {}", tuple.0, tuple.1);
            create_parameter(client, tuple.0, tuple.1).await?;
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
        ParCommand::Get { name } => match get_parameter(&client, &name).await {
            Ok(par_value) => {
                info!("Got parameter[{}] value:", name);
                info!("  {}", par_value);
            }
            Err(_) => {
                return Err(eyre!("Failed to retrieve parameter value"));
            }
        },
        ParCommand::Create { name, value } => {
            create_parameter(&client, name, value).await?;
        }
        ParCommand::Delete { name } => {
            delete_parameter(&client, name).await?;
        }
        ParCommand::Apply { path } => {
            apply_parameters(&client, path).await?;
        }
    }
    Ok(())
}
