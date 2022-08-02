use crate::errors::SecParError;
use crate::opt::SecCommand;
use aws_sdk_secretsmanager::{Client, Region};
use color_eyre::Report;
use tracing::{debug, info};

/// list all secrets
pub async fn list_secrets() -> Result<(), SecParError> {
    let region_provider = Region::new("us-east-1");
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);
    match client.list_secrets().send().await {
        Ok(output) => {
            info!("Got secrets:");
            let secrets = output.secret_list().unwrap_or_default();
            for secret in secrets {
                info!("  {}", secret.name().unwrap_or("No name!"));
            }
            Ok(())
        }
        Err(e) => {
            debug!("Error: {:?}", e.to_string());
            Err(SecParError::NotFound(e.to_string()))
        }
    }
}

/// get a secret from secret manager
pub async fn retrieve_secret(name: &str) -> Result<String, SecParError> {
    let region_provider = Region::new("us-east-1");
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);
    match client.get_secret_value().secret_id(name).send().await {
        Ok(output) => {
            debug!("Value: {}", output.secret_string().unwrap());
            Ok(output.secret_string().unwrap().to_owned())
        }
        Err(e) => {
            debug!("Error: {:?}", e.to_string());
            Err(SecParError::NotFound(e.to_string()))
        }
    }
}

/// create a secret in secret manager
pub async fn save_secret(name: &str, secret: &str) -> Result<String, SecParError> {
    let region_provider = Region::new("us-east-1");
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);
    match client
        .create_secret()
        .name(name)
        .secret_string(secret)
        .send()
        .await
    {
        Ok(output) => {
            debug!("Value: {}", output.arn().unwrap());
            Ok(output.arn().unwrap().to_owned())
        }
        Err(e) => {
            debug!("Error: {:?}", e.to_string());
            Err(SecParError::CreateFail(e.to_string()))
        }
    }
}

pub async fn process_sec_command(command: &SecCommand) -> Result<(), Report> {
    match command {
        SecCommand::List {} => {
            info!("List All Secrets...");
            list_secrets().await?;
        }
        SecCommand::Get { name } => {}
        SecCommand::Create { name, secret } => {}
        SecCommand::Delete { name } => {}
    }
    Ok(())
}
