use crate::errors::SecParError;
use crate::opt::{GlobalOpts, SecCommand};
use crate::ui::{build_secrets_table, confirm_delete, new_spinner, select_from_list};
use aws_sdk_secretsmanager::Client;
use color_eyre::Result;
use tracing::debug;

/// Returns a list of all secrets as `(name, arn, last_changed)` tuples.
///
/// # Arguments
///
/// * `client` - An initialised Secrets Manager [`Client`].
///
/// # Returns
///
/// A `Vec` of `(name, arn, last_changed)` tuples where each field falls back
/// to `"unknown"` / `""` when the AWS response omits it.
///
/// # Errors
///
/// Returns [`SecParError::AwsSdk`] if the AWS `list_secrets` call fails.
///
/// # Examples
///
/// ```no_run
/// use aws_sdk_secretsmanager::Client;
/// use secpar::cli::sec::list_secrets;
/// # async fn run(client: &Client) -> Result<(), secpar::errors::SecParError> {
/// let rows = list_secrets(client).await?;
/// for (name, arn, last_changed) in &rows {
///     println!("{name}  {arn}  {last_changed}");
/// }
/// # Ok(())
/// # }
/// ```
pub async fn list_secrets(client: &Client) -> Result<Vec<(String, String, String)>, SecParError> {
    match client.list_secrets().send().await {
        Ok(output) => {
            let rows = output
                .secret_list()
                .iter()
                .map(|s| {
                    let name = s.name().unwrap_or("unknown").to_owned();
                    let arn = s.arn().unwrap_or("").to_owned();
                    let last_changed = s
                        .last_changed_date()
                        .map(|d| d.to_string())
                        .unwrap_or_default();
                    (name, arn, last_changed)
                })
                .collect();
            Ok(rows)
        }
        Err(e) => {
            debug!("Error: {:?}", e.to_string());
            Err(SecParError::AwsSdk(e.to_string()))
        }
    }
}

/// Describes a secret and prints its full details.
///
/// # Arguments
///
/// * `client` - An initialised Secrets Manager [`Client`].
/// * `name` - The secret name or ARN.
///
/// # Errors
///
/// Returns [`SecParError::AwsSdk`] if the AWS `describe_secret` call fails.
///
/// # Examples
///
/// ```no_run
/// use aws_sdk_secretsmanager::Client;
/// use secpar::cli::sec::describe_secret;
/// # async fn run(client: &Client) -> Result<(), secpar::errors::SecParError> {
/// describe_secret(client, "my-secret").await?;
/// # Ok(())
/// # }
/// ```
pub async fn describe_secret(client: &Client, name: &str) -> Result<(), SecParError> {
    match client.describe_secret().secret_id(name).send().await {
        Ok(output) => {
            println!("{output:?}");
            Ok(())
        }
        Err(e) => {
            debug!("Error: {:?}", e.to_string());
            Err(SecParError::AwsSdk(e.to_string()))
        }
    }
}

/// Deletes the specified secret.
///
/// # Arguments
///
/// * `client` - An initialised Secrets Manager [`Client`].
/// * `name` - The secret name or ARN.
///
/// # Errors
///
/// Returns [`SecParError::AwsSdk`] if the AWS `delete_secret` call fails.
///
/// # Examples
///
/// ```no_run
/// use aws_sdk_secretsmanager::Client;
/// use secpar::cli::sec::delete_secret;
/// # async fn run(client: &Client) -> Result<(), secpar::errors::SecParError> {
/// delete_secret(client, "my-secret").await?;
/// # Ok(())
/// # }
/// ```
pub async fn delete_secret(client: &Client, name: &str) -> Result<(), SecParError> {
    match client.delete_secret().secret_id(name).send().await {
        Ok(_) => Ok(()),
        Err(e) => {
            debug!("Error: {:?}", e.to_string());
            Err(SecParError::AwsSdk(e.to_string()))
        }
    }
}

/// Retrieves the plaintext value of a secret.
///
/// # Arguments
///
/// * `client` - An initialised Secrets Manager [`Client`].
/// * `name` - The secret name or ARN.
///
/// # Returns
///
/// The `secret_string` field from the AWS response as an owned `String`.
///
/// # Errors
///
/// Returns [`SecParError::AwsSdk`] on API failure or
/// [`SecParError::MissingValue`] when `secret_string` is absent.
///
/// # Examples
///
/// ```no_run
/// use aws_sdk_secretsmanager::Client;
/// use secpar::cli::sec::retrieve_secret;
/// # async fn run(client: &Client) -> Result<(), secpar::errors::SecParError> {
/// let value = retrieve_secret(client, "my-secret").await?;
/// println!("{value}");
/// # Ok(())
/// # }
/// ```
pub async fn retrieve_secret(client: &Client, name: &str) -> Result<String, SecParError> {
    match client.get_secret_value().secret_id(name).send().await {
        Ok(output) => match output.secret_string() {
            Some(secret) => Ok(secret.to_owned()),
            None => Err(SecParError::MissingValue(format!(
                "secret value is empty for {name}"
            ))),
        },
        Err(e) => {
            debug!("Error: {:?}", e.to_string());
            Err(SecParError::AwsSdk(e.to_string()))
        }
    }
}

/// Creates a new secret with the given name and value.
///
/// # Arguments
///
/// * `client` - An initialised Secrets Manager [`Client`].
/// * `name` - The secret name.
/// * `secret` - The secret value (string).
///
/// # Returns
///
/// The ARN of the newly created secret.
///
/// # Errors
///
/// Returns [`SecParError::AwsSdk`] on API failure or
/// [`SecParError::MissingValue`] when the response omits the ARN.
///
/// # Examples
///
/// ```no_run
/// use aws_sdk_secretsmanager::Client;
/// use secpar::cli::sec::save_secret;
/// # async fn run(client: &Client) -> Result<(), secpar::errors::SecParError> {
/// let arn = save_secret(client, "my-secret", r#"{"key":"value"}"#).await?;
/// println!("Created: {arn}");
/// # Ok(())
/// # }
/// ```
pub async fn save_secret(client: &Client, name: &str, secret: &str) -> Result<String, SecParError> {
    match client
        .create_secret()
        .name(name)
        .secret_string(secret)
        .send()
        .await
    {
        Ok(output) => match output.arn() {
            Some(arn) => Ok(arn.to_owned()),
            None => Err(SecParError::MissingValue(format!(
                "secret ARN is missing for {name}"
            ))),
        },
        Err(e) => {
            debug!("Error: {:?}", e.to_string());
            Err(SecParError::AwsSdk(e.to_string()))
        }
    }
}

/// Resolves a secret name: returns `name` directly when provided, or presents
/// an interactive selection menu populated from `list_secrets`.
async fn resolve_secret_name(client: &Client, name: Option<&str>) -> Result<String, SecParError> {
    if let Some(n) = name {
        return Ok(n.to_owned());
    }
    let spinner = new_spinner("Fetching secrets for selection…");
    let rows = list_secrets(client).await?;
    spinner.finish_and_clear();
    let names: Vec<String> = rows.into_iter().map(|(n, _, _)| n).collect();
    select_from_list("Select a secret", &names)
}

/// Dispatches a `sec` subcommand.
///
/// # Arguments
///
/// * `command` - The parsed [`SecCommand`] variant.
/// * `opts` - Global CLI options (region, profile).
///
/// # Errors
///
/// Propagates any [`SecParError`] produced by the underlying operations,
/// wrapped in [`color_eyre::Report`].
pub async fn process_sec_command(command: &SecCommand, opts: &GlobalOpts) -> Result<()> {
    let shared_config = crate::cli::load_shared_config(opts).await;
    let client = Client::new(&shared_config);
    match command {
        SecCommand::List {} => {
            let spinner = new_spinner("Listing secrets…");
            let rows = list_secrets(&client).await?;
            spinner.finish_and_clear();
            if rows.is_empty() {
                println!("No secrets found.");
            } else {
                let refs: Vec<(&str, &str, &str)> = rows
                    .iter()
                    .map(|(n, a, l)| (n.as_str(), a.as_str(), l.as_str()))
                    .collect();
                println!("{}", build_secrets_table(&refs));
            }
        }
        SecCommand::Get { name } => {
            let resolved = resolve_secret_name(&client, name.as_deref()).await?;
            let spinner = new_spinner(format!("Retrieving '{resolved}'…").as_str());
            let secret = retrieve_secret(&client, &resolved).await?;
            spinner.finish_and_clear();
            println!("{secret}");
        }
        SecCommand::Describe { name } => {
            let resolved = resolve_secret_name(&client, name.as_deref()).await?;
            let spinner = new_spinner(format!("Describing '{resolved}'…").as_str());
            let result = describe_secret(&client, &resolved).await;
            spinner.finish_and_clear();
            result?;
        }
        SecCommand::Create { name, secret } => {
            let spinner = new_spinner(format!("Creating secret '{name}'…").as_str());
            let arn = save_secret(&client, name, secret).await?;
            spinner.finish_and_clear();
            println!("{arn}");
        }
        SecCommand::Delete { name } => {
            let resolved = resolve_secret_name(&client, name.as_deref()).await?;
            if !confirm_delete(&resolved)? {
                println!("Aborted.");
                return Ok(());
            }
            let spinner = new_spinner(format!("Deleting '{resolved}'…").as_str());
            delete_secret(&client, &resolved).await?;
            spinner.finish_and_clear();
            println!("deleted {resolved}");
        }
    }
    Ok(())
}
