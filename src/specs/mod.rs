use super::util;
use crate::errors::SecParError;
use serde::{Deserialize, Deserializer, Serialize};

// ── shared deserializer logic ────────────────────────────────────────────────

/// Deserializes a compact `"key:value"` string or an explicit map into
/// `(key, value)`, used by both [`ParameterEntry`] and [`SecretEntry`].
fn deserialize_inline_or_map<'de, D>(
    deserializer: D,
    map_key: &'static str,
    map_value: &'static str,
) -> Result<(String, String), D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{self, MapAccess, Visitor};
    use std::fmt;

    struct EntryVisitor {
        map_key: &'static str,
        map_value: &'static str,
    }

    impl<'de> Visitor<'de> for EntryVisitor {
        type Value = (String, String);

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "a 'key:value' string or a map with '{}' and '{}' fields",
                self.map_key, self.map_value
            )
        }

        // Compact form: `- name:value`
        fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
            let (k, v) = s.split_once(':').ok_or_else(|| {
                de::Error::custom(format!(
                    "expected '{}:{}' but no ':' found in '{s}'",
                    self.map_key, self.map_value
                ))
            })?;
            Ok((k.to_owned(), v.to_owned()))
        }

        // Expanded form: `- name: …\n  value: …`
        fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {
            let mut key: Option<String> = None;
            let mut val: Option<String> = None;
            while let Some(k) = map.next_key::<String>()? {
                if k == self.map_key {
                    key = Some(map.next_value()?);
                } else if k == self.map_value {
                    val = Some(map.next_value()?);
                } else {
                    map.next_value::<serde::de::IgnoredAny>()?;
                }
            }
            let key = key.ok_or_else(|| de::Error::missing_field(self.map_key))?;
            let val = val.ok_or_else(|| de::Error::missing_field(self.map_value))?;
            Ok((key, val))
        }
    }

    deserializer.deserialize_any(EntryVisitor { map_key, map_value })
}

// ── ParameterEntry ────────────────────────────────────────────────────────────

/// A single strongly-typed Parameter Store entry.
///
/// Supports two YAML representations:
///
/// **Compact** (colon-separated inline string):
/// ```yaml
/// parameters:
///   - /prod/db/host:db.internal.example.com
/// ```
///
/// **Expanded** (explicit key-value map — required when value contains colons):
/// ```yaml
/// parameters:
///   - name: /prod/db/url
///     value: postgres://user:pass@db:5432/mydb
/// ```
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ParameterEntry {
    /// The SSM parameter name (e.g. `/prod/db/host`).
    pub name: String,
    /// The parameter value.
    pub value: String,
}

impl<'de> Deserialize<'de> for ParameterEntry {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let (name, value) = deserialize_inline_or_map(deserializer, "name", "value")?;
        Ok(ParameterEntry { name, value })
    }
}

// ── SecretEntry ───────────────────────────────────────────────────────────────

/// A single strongly-typed Secrets Manager entry.
///
/// Supports two YAML representations:
///
/// **Compact** (colon-separated inline string):
/// ```yaml
/// secrets:
///   - prod/api/key:sk-abc123
/// ```
///
/// **Expanded** (explicit name/secret map — required when the secret value
/// contains colons, e.g. a JSON string or connection URL):
/// ```yaml
/// secrets:
///   - name: prod/db/password
///     secret: '{"user":"admin","pass":"s3cr3t"}'
/// ```
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct SecretEntry {
    /// The Secrets Manager secret name.
    pub name: String,
    /// The secret value (string or JSON).
    pub secret: String,
}

impl<'de> Deserialize<'de> for SecretEntry {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let (name, secret) = deserialize_inline_or_map(deserializer, "name", "secret")?;
        Ok(SecretEntry { name, secret })
    }
}

// ── ParameterStore ────────────────────────────────────────────────────────────

/// Represents the contents of a Parameter Store YAML spec file.
///
/// ```yaml
/// parameters:
///   - /prod/db/host:db.internal.example.com
///   - name: /prod/db/url
///     value: postgres://user:pass@db:5432/mydb
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ParameterStore {
    pub parameters: Vec<ParameterEntry>,
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
    /// A `ParameterStore` whose `parameters` field holds every entry as a
    /// strongly-typed [`ParameterEntry`].
    ///
    /// # Errors
    ///
    /// Returns [`SecParError::InvalidSpec`] if the file cannot be opened or
    /// any entry is malformed.
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
    ///     println!("{} = {}", entry.name, entry.value);
    /// }
    /// ```
    pub fn new(path: &std::path::Path) -> Result<Self, SecParError> {
        util::read_yaml_from_file(path).map_err(SecParError::InvalidSpec)
    }
}

// ── SecretsSpec ───────────────────────────────────────────────────────────────

/// Represents the contents of a Secrets Manager YAML spec file.
///
/// ```yaml
/// secrets:
///   - prod/api/key:sk-abc123
///   - name: prod/db/password
///     secret: '{"user":"admin","pass":"s3cr3t"}'
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SecretsSpec {
    pub secrets: Vec<SecretEntry>,
}

impl SecretsSpec {
    /// Constructs a [`SecretsSpec`] by reading and deserializing a YAML file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the YAML spec file.
    ///
    /// # Returns
    ///
    /// A `SecretsSpec` whose `secrets` field holds every entry as a
    /// strongly-typed [`SecretEntry`].
    ///
    /// # Errors
    ///
    /// Returns [`SecParError::InvalidSpec`] if the file cannot be opened or
    /// any entry is malformed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use secpar::specs::SecretsSpec;
    ///
    /// let spec = SecretsSpec::new(Path::new("./templates/secrets_template.yaml"))
    ///     .expect("failed to load secrets spec");
    /// for entry in &spec.secrets {
    ///     println!("{}", entry.name);
    /// }
    /// ```
    pub fn new(path: &std::path::Path) -> Result<Self, SecParError> {
        util::read_yaml_from_file(path).map_err(SecParError::InvalidSpec)
    }
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write as IoWrite;

    // ── ParameterEntry ────────────────────────────────────────────────────────

    #[test]
    fn parameter_deserializes_compact_form() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        writeln!(
            tmp,
            "parameters:\n  - /prod/db/host:db.example.com\n  - /prod/db/port:5432"
        )
        .unwrap();
        let store = ParameterStore::new(tmp.path()).unwrap();
        assert_eq!(store.parameters.len(), 2);
        assert_eq!(store.parameters[0].name, "/prod/db/host");
        assert_eq!(store.parameters[0].value, "db.example.com");
        assert_eq!(store.parameters[1].name, "/prod/db/port");
        assert_eq!(store.parameters[1].value, "5432");
    }

    #[test]
    fn parameter_deserializes_expanded_form() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        writeln!(
            tmp,
            "parameters:\n  - name: /prod/api/key\n    value: sk-abc123"
        )
        .unwrap();
        let store = ParameterStore::new(tmp.path()).unwrap();
        assert_eq!(store.parameters.len(), 1);
        assert_eq!(store.parameters[0].name, "/prod/api/key");
        assert_eq!(store.parameters[0].value, "sk-abc123");
    }

    #[test]
    fn parameter_deserializes_mixed_forms() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        writeln!(
            tmp,
            "parameters:\n  - /prod/db/host:db.example.com\n  - name: /prod/api/key\n    value: sk-abc123"
        )
        .unwrap();
        let store = ParameterStore::new(tmp.path()).unwrap();
        assert_eq!(store.parameters.len(), 2);
        assert_eq!(store.parameters[0].name, "/prod/db/host");
        assert_eq!(store.parameters[1].name, "/prod/api/key");
    }

    #[test]
    fn parameter_errors_on_compact_entry_missing_colon() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        writeln!(tmp, "parameters:\n  - no-colon-here").unwrap();
        let result = ParameterStore::new(tmp.path());
        assert!(result.is_err());
    }

    // ── SecretEntry ───────────────────────────────────────────────────────────

    #[test]
    fn secret_deserializes_compact_form() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        writeln!(tmp, "secrets:\n  - prod/api/key:sk-abc123").unwrap();
        let spec = SecretsSpec::new(tmp.path()).unwrap();
        assert_eq!(spec.secrets.len(), 1);
        assert_eq!(spec.secrets[0].name, "prod/api/key");
        assert_eq!(spec.secrets[0].secret, "sk-abc123");
    }

    #[test]
    fn secret_deserializes_expanded_form() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(
            tmp.path(),
            "secrets:\n  - name: prod/db/password\n    secret: '{\"user\":\"admin\"}'\n",
        )
        .unwrap();
        let spec = SecretsSpec::new(tmp.path()).unwrap();
        assert_eq!(spec.secrets.len(), 1);
        assert_eq!(spec.secrets[0].name, "prod/db/password");
        assert_eq!(spec.secrets[0].secret, r#"{"user":"admin"}"#);
    }

    #[test]
    fn secret_deserializes_mixed_forms() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        writeln!(
            tmp,
            "secrets:\n  - prod/api/key:sk-abc123\n  - name: prod/db/password\n    secret: s3cr3t"
        )
        .unwrap();
        let spec = SecretsSpec::new(tmp.path()).unwrap();
        assert_eq!(spec.secrets.len(), 2);
        assert_eq!(spec.secrets[0].name, "prod/api/key");
        assert_eq!(spec.secrets[1].name, "prod/db/password");
    }

    #[test]
    fn secret_errors_on_compact_entry_missing_colon() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        writeln!(tmp, "secrets:\n  - no-colon-here").unwrap();
        let result = SecretsSpec::new(tmp.path());
        assert!(result.is_err());
    }
}
