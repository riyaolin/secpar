//! Shared UX primitives: spinners, confirmation prompts, interactive selection, and table builders.

use crate::errors::SecParError;
use comfy_table::{Table, presets::UTF8_FULL};
use dialoguer::{Confirm, Select};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

/// Creates a new braille spinner with the given message.
///
/// # Arguments
///
/// * `message` - The message to display next to the spinner.
///
/// # Returns
///
/// A [`ProgressBar`] configured as a braille spinner ticking every 80 ms.
///
/// # Examples
///
/// ```no_run
/// use secpar::ui::new_spinner;
/// let spinner = new_spinner("Fetching secrets…");
/// // do work…
/// spinner.finish_and_clear();
/// ```
pub fn new_spinner(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(80));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb.set_message(message.to_owned());
    pb
}

/// Prompts the user to confirm a delete operation.
///
/// # Arguments
///
/// * `resource_name` - The name of the resource to be deleted, shown in the prompt.
///
/// # Returns
///
/// `Ok(true)` if the user confirmed, `Ok(false)` if declined.
///
/// # Errors
///
/// Returns [`SecParError::Interactive`] if the terminal prompt fails.
///
/// # Examples
///
/// ```no_run
/// use secpar::ui::confirm_delete;
/// if confirm_delete("my-secret")? {
///     // proceed with deletion
/// }
/// # Ok::<(), secpar::errors::SecParError>(())
/// ```
pub fn confirm_delete(resource_name: &str) -> Result<bool, SecParError> {
    Confirm::new()
        .with_prompt(format!("Delete '{resource_name}'?"))
        .default(false)
        .interact()
        .map_err(|e| SecParError::Interactive(e.to_string()))
}

/// Presents an interactive selection menu and returns the chosen item.
///
/// # Arguments
///
/// * `prompt` - The prompt text shown above the list.
/// * `items` - The list of items to display.
///
/// # Returns
///
/// The selected item as an owned `String`.
///
/// # Errors
///
/// Returns [`SecParError::Interactive`] if `items` is empty or if the terminal prompt fails.
///
/// # Examples
///
/// ```no_run
/// use secpar::ui::select_from_list;
/// let items = vec!["alpha".to_string(), "beta".to_string()];
/// let chosen = select_from_list("Pick one", &items)?;
/// println!("You chose: {chosen}");
/// # Ok::<(), secpar::errors::SecParError>(())
/// ```
pub fn select_from_list(prompt: &str, items: &[String]) -> Result<String, SecParError> {
    if items.is_empty() {
        return Err(SecParError::Interactive(
            "No items available to select from".to_string(),
        ));
    }
    let idx = Select::new()
        .with_prompt(prompt)
        .items(items)
        .default(0)
        .interact()
        .map_err(|e| SecParError::Interactive(e.to_string()))?;
    Ok(items[idx].clone())
}

/// Builds a formatted table for displaying secrets.
///
/// Columns: `NAME`, `ARN`, `LAST CHANGED`.
///
/// # Arguments
///
/// * `rows` - Slice of `(name, arn, last_changed)` tuples.
///
/// # Returns
///
/// A [`Table`] ready to be printed with `println!("{table}")`.
///
/// # Examples
///
/// ```
/// use secpar::ui::build_secrets_table;
/// let rows = vec![("my-secret", "arn:aws:…", "2024-01-01")];
/// let table = build_secrets_table(&rows);
/// let rendered = table.to_string();
/// assert!(rendered.contains("NAME"));
/// assert!(rendered.contains("my-secret"));
/// ```
pub fn build_secrets_table(rows: &[(&str, &str, &str)]) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(["NAME", "ARN", "LAST CHANGED"]);
    for (name, arn, last_changed) in rows {
        table.add_row([*name, *arn, *last_changed]);
    }
    table
}

/// Builds a formatted table for displaying parameters.
///
/// Columns: `NAME`, `TYPE`, `LAST MODIFIED`.
///
/// # Arguments
///
/// * `rows` - Slice of `(name, type, last_modified)` tuples.
///
/// # Returns
///
/// A [`Table`] ready to be printed with `println!("{table}")`.
///
/// # Examples
///
/// ```
/// use secpar::ui::build_parameters_table;
/// let rows = vec![("/my/param", "SecureString", "2024-01-01")];
/// let table = build_parameters_table(&rows);
/// let rendered = table.to_string();
/// assert!(rendered.contains("NAME"));
/// assert!(rendered.contains("/my/param"));
/// ```
pub fn build_parameters_table(rows: &[(&str, &str, &str)]) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(["NAME", "TYPE", "LAST MODIFIED"]);
    for (name, typ, last_modified) in rows {
        table.add_row([*name, *typ, *last_modified]);
    }
    table
}

/// Prints a success message with a ✅ prefix.
pub fn print_success(msg: &str) {
    println!("✅ {msg}");
}

/// Prints an informational message with a ℹ️  prefix.
pub fn print_info(msg: &str) {
    println!("ℹ️  {msg}");
}

/// Prints a retrieved value with a 🔑 prefix.
pub fn print_value(label: &str, value: &str) {
    println!("🔑 {label}\n{value}");
}

/// Prints an aborted-operation message with a 🚫 prefix.
pub fn print_aborted() {
    println!("🚫 Aborted.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_from_list_errors_on_empty_input() {
        let result = select_from_list("Pick one", &[]);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("No items available"));
    }

    #[test]
    fn build_secrets_table_contains_header_and_row() {
        let rows = vec![(
            "my-secret",
            "arn:aws:secretsmanager:us-east-1:123:secret:my-secret",
            "2024-06-01",
        )];
        let table = build_secrets_table(&rows);
        let rendered = table.to_string();
        assert!(rendered.contains("NAME"));
        assert!(rendered.contains("ARN"));
        assert!(rendered.contains("LAST CHANGED"));
        assert!(rendered.contains("my-secret"));
        assert!(rendered.contains("2024-06-01"));
    }

    #[test]
    fn build_secrets_table_handles_empty_rows() {
        let table = build_secrets_table(&[]);
        let rendered = table.to_string();
        assert!(rendered.contains("NAME"));
        assert!(rendered.contains("ARN"));
        assert!(rendered.contains("LAST CHANGED"));
    }

    #[test]
    fn build_parameters_table_contains_header_and_row() {
        let rows = vec![("/my/param", "SecureString", "2024-06-01")];
        let table = build_parameters_table(&rows);
        let rendered = table.to_string();
        assert!(rendered.contains("NAME"));
        assert!(rendered.contains("TYPE"));
        assert!(rendered.contains("LAST MODIFIED"));
        assert!(rendered.contains("/my/param"));
        assert!(rendered.contains("SecureString"));
    }

    #[test]
    fn build_parameters_table_handles_empty_rows() {
        let table = build_parameters_table(&[]);
        let rendered = table.to_string();
        assert!(rendered.contains("NAME"));
        assert!(rendered.contains("TYPE"));
        assert!(rendered.contains("LAST MODIFIED"));
    }
}
