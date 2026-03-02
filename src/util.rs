use crate::errors::ParseFileError;
use serde::Deserialize;
use tracing::debug;

/// Deserializes a YAML file into the given type.
///
/// # Arguments
///
/// * `path` - Path to the YAML file to read.
///
/// # Returns
///
/// The deserialized value of type `T` on success.
///
/// # Errors
///
/// - [`ParseFileError::NoSuchFile`] — the file at `path` could not be opened.
/// - [`ParseFileError::DeserializeError`] — the file contents could not be
///   deserialized as `T`.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use secpar::util::read_yaml_from_file;
/// use secpar::specs::ParameterStore;
///
/// let store: ParameterStore = read_yaml_from_file(Path::new("params.yaml")).unwrap();
/// ```
pub fn read_yaml_from_file<T>(path: &std::path::Path) -> Result<T, ParseFileError>
where
    T: for<'de> Deserialize<'de>,
{
    let f = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(e) => {
            return Err(ParseFileError::NoSuchFile(
                path.to_string_lossy().into_owned(),
                e,
            ));
        }
    };
    debug!(?f, "File info:");
    match serde_norway::from_reader(f) {
        Ok(v) => Ok(v),
        Err(e) => Err(ParseFileError::DeserializeError(
            path.to_string_lossy().into_owned(),
            e,
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::specs::ParameterStore;
    use std::io::Write as IoWrite;

    #[test]
    fn read_yaml_from_file_parses_valid_yaml() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        writeln!(
            tmp,
            "parameters:\n  - /secpar/TEST:TEST_VALUE\n  - /secpar/qa/KEY:VAL"
        )
        .unwrap();
        let result: ParameterStore = read_yaml_from_file(tmp.path()).unwrap();
        assert_eq!(result.parameters.len(), 2);
        assert_eq!(result.parameters[0], "/secpar/TEST:TEST_VALUE");
        assert_eq!(result.parameters[1], "/secpar/qa/KEY:VAL");
    }

    #[test]
    fn read_yaml_from_file_errors_on_missing_file() {
        let result: Result<ParameterStore, _> =
            read_yaml_from_file(std::path::Path::new("/nonexistent/path/file.yaml"));
        assert!(result.is_err());
        match result.unwrap_err() {
            ParseFileError::NoSuchFile(path, _) => {
                assert!(path.contains("nonexistent"));
            }
            other => panic!("Expected NoSuchFile, got {other:?}"),
        }
    }
}
