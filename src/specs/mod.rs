use super::util;
use crate::errors::SecParError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
/// For the par apply sub-subcommand
/// The spec is in yaml format and each parameter entry’s name and value
/// are separated by : , a colon symbol:
/// Examples:
/// parameters:
///     - /secpar/TEST:TEST_VALUE
///     - /secpar/qa/SASL_USERNAME:USERNAME
pub struct ParameterStore {
    pub parameters: Vec<String>,
}

impl ParameterStore {
    /// new a parameter store from a yaml spec file
    pub fn new(path: &std::path::Path) -> Result<Self, SecParError> {
        match util::read_yaml_from_file(path) {
            Ok(content) => Ok(content),
            Err(e) => Err(SecParError::InvalidSpec(e)),
        }
    }
}
