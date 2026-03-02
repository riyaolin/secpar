use crate::errors::SecParError;
use crate::opt::{GlobalOpts, ParCommand};
use crate::specs::ParameterStore;
use crate::ui::{build_parameters_table, confirm_delete, new_spinner, select_from_list};
use aws_sdk_ssm::{Client, types::ParameterType};
use color_eyre::Result;
use tracing::{debug, info, warn};

/// Returns all parameters as `(name, type, last_modified)` tuples.
///
/// Iterates through all paginator pages so large stores are fully returned.
///
/// # Arguments
///
/// * `client` - An initialised SSM [`Client`].
///
/// # Returns
///
/// A `Vec` of `(name, type, last_modified)` tuples.  Fields fall back to
/// `"unknown"` / `""` when absent in the AWS response.
///
/// # Errors
///
/// Returns [`SecParError::AwsSdk`] if any paginator page fails.
///
/// # Examples
///
/// ```no_run
/// use aws_sdk_ssm::Client;
/// use secpar::cli::par::list_parameters;
/// # async fn run(client: &Client) -> Result<(), secpar::errors::SecParError> {
/// let rows = list_parameters(client).await?;
/// for (name, typ, last_modified) in &rows {
///     println!("{name}  {typ}  {last_modified}");
/// }
/// # Ok(())
/// # }
/// ```
pub async fn list_parameters(
    client: &Client,
) -> Result<Vec<(String, String, String)>, SecParError> {
    let mut pars_stream = client.describe_parameters().into_paginator().send();
    let mut rows = Vec::new();
    while let Some(page) = pars_stream.next().await {
        debug!("New Parameter Page!");
        let page = page.map_err(|e| SecParError::AwsSdk(e.to_string()))?;
        for p in page.parameters() {
            let name = p.name().unwrap_or("unknown").to_owned();
            let typ = p
                .r#type()
                .map(|t| t.as_str().to_owned())
                .unwrap_or_else(|| "unknown".to_owned());
            let last_modified = p
                .last_modified_date()
                .map(|d| d.to_string())
                .unwrap_or_default();
            rows.push((name, typ, last_modified));
        }
    }
    Ok(rows)
}

/// Retrieves the decrypted value of a parameter.
///
/// # Arguments
///
/// * `client` - An initialised SSM [`Client`].
/// * `name` - The parameter name.
///
/// # Returns
///
/// The parameter value as an owned `String`.
///
/// # Errors
///
/// Returns [`SecParError::AwsSdk`] on API failure or
/// [`SecParError::MissingValue`] when the value field is absent.
///
/// # Examples
///
/// ```no_run
/// use aws_sdk_ssm::Client;
/// use secpar::cli::par::get_parameter;
/// # async fn run(client: &Client) -> Result<(), secpar::errors::SecParError> {
/// let value = get_parameter(client, "/my/param").await?;
/// println!("{value}");
/// # Ok(())
/// # }
/// ```
pub async fn get_parameter(client: &Client, name: &str) -> Result<String, SecParError> {
    match client
        .get_parameter()
        .name(name)
        .with_decryption(true)
        .send()
        .await
    {
        Ok(output) => match output.parameter().and_then(|p| p.value()) {
            Some(value) => Ok(value.to_owned()),
            None => Err(SecParError::MissingValue(format!(
                "parameter value is empty for {name}"
            ))),
        },
        Err(e) => {
            debug!("Error: {:?}", e.to_string());
            Err(SecParError::AwsSdk(e.to_string()))
        }
    }
}

/// Creates or overwrites a parameter as `SecureString`.
///
/// # Arguments
///
/// * `client` - An initialised SSM [`Client`].
/// * `name` - The parameter name.
/// * `value` - The parameter value.
///
/// # Errors
///
/// Returns [`SecParError::AwsSdk`] if the put-parameter call fails.
///
/// # Examples
///
/// ```no_run
/// use aws_sdk_ssm::Client;
/// use secpar::cli::par::create_parameter;
/// # async fn run(client: &Client) -> Result<(), secpar::errors::SecParError> {
/// create_parameter(client, "/my/param", "s3cr3t").await?;
/// # Ok(())
/// # }
/// ```
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
        Ok(_) => {
            debug!("created");
            Ok(())
        }
        Err(e) => {
            debug!("Error: {:?}", e.to_string());
            Err(SecParError::AwsSdk(e.to_string()))
        }
    }
}

/// Deletes the specified parameter.
///
/// # Arguments
///
/// * `client` - An initialised SSM [`Client`].
/// * `name` - The parameter name.
///
/// # Errors
///
/// Returns [`SecParError::AwsSdk`] if the delete call fails.
///
/// # Examples
///
/// ```no_run
/// use aws_sdk_ssm::Client;
/// use secpar::cli::par::delete_parameter;
/// # async fn run(client: &Client) -> Result<(), secpar::errors::SecParError> {
/// delete_parameter(client, "/my/param").await?;
/// # Ok(())
/// # }
/// ```
pub async fn delete_parameter(client: &Client, name: &str) -> Result<(), SecParError> {
    match client.delete_parameter().name(name).send().await {
        Ok(_) => {
            info!("Deleted parameter: {name}");
            Ok(())
        }
        Err(e) => {
            debug!("Error: {:?}", e.to_string());
            Err(SecParError::AwsSdk(e.to_string()))
        }
    }
}

/// Reads and applies all parameters defined in a YAML spec file.
///
/// Each entry must be in the form `name:value` (colon-separated).  Malformed
/// entries are logged as warnings and skipped.
///
/// # Arguments
///
/// * `client` - An initialised SSM [`Client`].
/// * `path` - Path to the YAML spec file.
///
/// # Errors
///
/// Returns [`SecParError::InvalidSpec`] if the file cannot be parsed, or
/// [`SecParError::AwsSdk`] if any individual put-parameter call fails.
///
/// # Examples
///
/// ```no_run
/// use aws_sdk_ssm::Client;
/// use secpar::cli::par::apply_parameters;
/// # async fn run(client: &Client) -> Result<(), secpar::errors::SecParError> {
/// apply_parameters(client, std::path::Path::new("./params.yaml")).await?;
/// # Ok(())
/// # }
/// ```
pub async fn apply_parameters(client: &Client, path: &std::path::Path) -> Result<(), SecParError> {
    let spec_content = ParameterStore::new(path)?;
    for parameter_pair in spec_content.parameters {
        if let Some(tuple) = parameter_pair.split_once(':') {
            info!("name: {}, value: {}", tuple.0, tuple.1);
            create_parameter(client, tuple.0, tuple.1).await?;
        } else {
            warn!("Skipping invalid parameter entry: {parameter_pair}");
        }
    }
    Ok(())
}

/// Resolves a parameter name: returns `name` directly when provided, or
/// presents an interactive selection menu populated from `list_parameters`.
async fn resolve_parameter_name(
    client: &Client,
    name: Option<&str>,
) -> Result<String, SecParError> {
    if let Some(n) = name {
        return Ok(n.to_owned());
    }
    let spinner = new_spinner("Fetching parameters for selection…");
    let rows = list_parameters(client).await?;
    spinner.finish_and_clear();
    let names: Vec<String> = rows.into_iter().map(|(n, _, _)| n).collect();
    select_from_list("Select a parameter", &names)
}

/// Dispatches a `par` subcommand.
///
/// # Arguments
///
/// * `command` - The parsed [`ParCommand`] variant.
/// * `opts` - Global CLI options (region, profile).
///
/// # Errors
///
/// Propagates any [`SecParError`] produced by the underlying operations,
/// wrapped in [`color_eyre::Report`].
pub async fn process_par_command(command: &ParCommand, opts: &GlobalOpts) -> Result<()> {
    let shared_config = crate::cli::load_shared_config(opts).await;
    let client = Client::new(&shared_config);
    match command {
        ParCommand::List {} => {
            let spinner = new_spinner("Listing parameters…");
            let rows = list_parameters(&client).await?;
            spinner.finish_and_clear();
            if rows.is_empty() {
                println!("No parameters found.");
            } else {
                let refs: Vec<(&str, &str, &str)> = rows
                    .iter()
                    .map(|(n, t, l)| (n.as_str(), t.as_str(), l.as_str()))
                    .collect();
                println!("{}", build_parameters_table(&refs));
            }
        }
        ParCommand::Get { name } => {
            let resolved = resolve_parameter_name(&client, name.as_deref()).await?;
            let spinner = new_spinner(format!("Retrieving '{resolved}'…").as_str());
            let value = get_parameter(&client, &resolved).await?;
            spinner.finish_and_clear();
            println!("{value}");
        }
        ParCommand::Create { name, value } => {
            let spinner = new_spinner(format!("Creating parameter '{name}'…").as_str());
            create_parameter(&client, name, value).await?;
            spinner.finish_and_clear();
            println!("created {name}");
        }
        ParCommand::Delete { name } => {
            let resolved = resolve_parameter_name(&client, name.as_deref()).await?;
            if !confirm_delete(&resolved)? {
                println!("Aborted.");
                return Ok(());
            }
            let spinner = new_spinner(format!("Deleting '{resolved}'…").as_str());
            delete_parameter(&client, &resolved).await?;
            spinner.finish_and_clear();
            println!("deleted {resolved}");
        }
        ParCommand::Apply { path } => {
            let spinner = new_spinner("Applying parameters from spec…");
            apply_parameters(&client, path).await?;
            spinner.finish_and_clear();
        }
    }
    Ok(())
}
