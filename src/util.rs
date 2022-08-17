use crate::errors::ParseFileError;
use serde::Deserialize;
use tracing::debug;

/// yaml deserialization
pub fn read_yaml_from_file<T>(path: &std::path::Path) -> Result<T, ParseFileError>
where
    T: for<'de> Deserialize<'de>,
{
    let f = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(e) => {
            return Err(ParseFileError::NoSuchFile(
                path.to_str().unwrap().to_owned(),
                e,
            ))
        }
    };
    debug!(?f, "File info:");
    match serde_yaml::from_reader(f) {
        Ok(v) => Ok(v),
        Err(e) => Err(ParseFileError::DeserializeError(
            path.to_str().unwrap().to_owned(),
            e,
        )),
    }
}
