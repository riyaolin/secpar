use super::util;
use crate::errors::SecParError;
use serde::{Deserialize, Serialize};

/// Represents the contents of a Parameter Store YAML spec file.
///
/// Each entry in `parameters` is a colon-separated `name:value` string.
///
/// ## Example YAML format
///
/// ```yaml
/// parameters:
///   - /secpar/TEST:TEST_VALUE
///   - /secpar/qa/SASL_USERNAME:USERNAME
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ParameterStore {
    pub parameters: Vec<String>,
}

impl ParameterStore {
    /// Constructs a [`ParameterStore`] by reading and deserializing a YAML file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the YAML spec file.
    ///
    /// # Returns
    ///
    /// A `ParameterStore` whose `parameters` field holds every entry from the
    /// file's `parameters` list.
    ///
    /// # Errors
    ///
    /// Returns [`SecParError::InvalidSpec`] (wrapping a [`crate::errors::ParseFileError`])
    /// when:
    /// - the file at `path` cannot be opened, or
    /// - the file contents cannot be deserialized as [`ParameterStore`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use secpar::specs::ParameterStore;
    ///
    /// let store = ParameterStore::new(Path::new("./templates/parameter_store_template.yaml"))
    ///     .expect("failed to load parameter spec");
    /// for entry in &store.parameters {
    ///     println!("{entry}");
    /// }
    /// ```
    pub fn new(path: &std::path::Path) -> Result<Self, SecParError> {
        match util::read_yaml_from_file(path) {
            Ok(content) => Ok(content),
            Err(e) => Err(SecParError::InvalidSpec(e)),
        }
    }
}
