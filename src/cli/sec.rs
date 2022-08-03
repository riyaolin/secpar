use crate::errors::SecParError;
use crate::opt::SecCommand;
use aws_sdk_secretsmanager::{Client, Region};
use color_eyre::eyre::eyre;
use color_eyre::Report;
use tracing::{debug, info};

/// list all secrets
pub async fn list_secrets(client: &Client) -> Result<(), SecParError> {
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

/// describe a secret in details:
/// https://docs.rs/aws-sdk-secretsmanager/0.16.0/aws_sdk_secretsmanager/output/struct.DescribeSecretOutput.html
pub async fn describe_secret(client: &Client, name: &str) -> Result<(), SecParError> {
    match client.describe_secret().secret_id(name).send().await {
        Ok(output) => {
            info!("Secret[{}] Details: ", name);
            info!("{:?}", output);
            Ok(())
        }
        Err(e) => {
            debug!("Error: {:?}", e.to_string());
            Err(SecParError::NotFound(e.to_string()))
        }
    }
}

/// delete the specified secret
pub async fn delete_secret(client: &Client, name: &str) -> Result<(), SecParError> {
    match client.delete_secret().secret_id(name).send().await {
        Ok(output) => {
            info!("To be deleted secret: {}", name);
            info!("{:?}", output);
            Ok(())
        }
        Err(e) => {
            debug!("Error: {:?}", e.to_string());
            Err(SecParError::NotFound(e.to_string()))
        }
    }
}

/// get a secret from secret manager
pub async fn retrieve_secret(client: &Client, name: &str) -> Result<String, SecParError> {
    match client.get_secret_value().secret_id(name).send().await {
        Ok(output) => {
            //debug!("Value: {}", output.secret_string().unwrap());
            Ok(output.secret_string().unwrap().to_owned())
        }
        Err(e) => {
            debug!("Error: {:?}", e.to_string());
            Err(SecParError::NotFound(e.to_string()))
        }
    }
}

/// create a secret in secret manager
pub async fn save_secret(client: &Client, name: &str, secret: &str) -> Result<String, SecParError> {
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

/// secret subcommand processing
pub async fn process_sec_command(command: &SecCommand) -> Result<(), Report> {
    let region_provider = Region::new("us-east-1");
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);
    match command {
        SecCommand::List {} => {
            info!("List All Secrets...");
            list_secrets(&client).await?;
        }
        SecCommand::Get { name } => match retrieve_secret(&client, &name).await {
            Ok(secret) => {
                info!("Got Secret:");
                info!("  {}", secret);
            }
            Err(_) => {
                return Err(eyre!("Failed to retrieve secret"));
            }
        },
        SecCommand::Describe { name } => {
            describe_secret(&client, name).await?;
        }
        SecCommand::Create { name, secret } => {
            save_secret(&client, &name, &secret).await?;
        }
        SecCommand::Delete { name } => {
            delete_secret(&client, name).await?;
        }
    }
    Ok(())
}
