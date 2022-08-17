use super::util;
use crate::errors::SecParError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ParameterStore {
    pub parameters: Vec<String>,
}

impl ParameterStore {
    ///
    pub fn new(path: &std::path::Path) -> Result<Self, SecParError> {
        match util::read_yaml_from_file(path) {
            Ok(content) => Ok(content),
            Err(e) => Err(SecParError::InvalidSpec(e)),
        }
    }
}
