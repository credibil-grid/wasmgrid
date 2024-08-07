use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Key bundle is the main data type transferred to and from the Azure Key Vault API.
#[derive(Debug, Deserialize)]
pub struct KeyBundle {
    pub attributes: KeyAttributes,
    pub key: JsonWebKey,
    pub managed: Option<bool>,
    pub tags: Option<Map<String, Value>>,
}

/// A deleted key bundle consists of a key bundle and its deletion information.
#[derive(Debug, Deserialize)]
pub struct Deleted {
    #[serde(flatten)]
    pub key_bundle: KeyBundle,

    #[serde(rename = "recoveryId")]
    pub recovery_id: Option<String>,

    #[serde(
        rename = "deletedDate",
        with = "ts_seconds_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub date: Option<DateTime<Utc>>,

    #[serde(
        rename = "scheduledPurgeDate",
        with = "ts_seconds_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub scheduled_purge_date: Option<DateTime<Utc>>,
}

/// The attributes of a key managed by the key vault service.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyAttributes {
    /// Creation time in UTC.
    #[serde(with = "ts_seconds_option", skip_serializing_if = "Option::is_none", default)]
    pub created: Option<DateTime<Utc>>,

    /// Determines whether the object is enabled.
    pub enabled: Option<bool>,

    /// Expiry date in UTC.
    #[serde(
        rename = "exp",
        with = "ts_seconds_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub expires: Option<DateTime<Utc>>,

    /// Not before date in UTC.
    #[serde(
        rename = "nbf",
        with = "ts_seconds_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub not_before: Option<DateTime<Utc>>,

    /// softDelete data retention days. Value should be >=7 and <=90 when softDelete enabled, otherwise 0.
    pub recoverable_days: Option<u8>,

    /// Reflects the deletion recovery level currently in effect for keys in the current vault. If it contains 'Purgeable' the key can be permanently deleted by a privileged user; otherwise, only the system can purge the key, at the end of the retention interval.
    pub recovery_level: Option<String>,

    /// Last updated time in UTC.
    #[serde(with = "ts_seconds_option", skip_serializing_if = "Option::is_none", default)]
    pub updated: Option<DateTime<Utc>>,
}

/// See <http://tools.ietf.org/html/draft-ietf-jose-json-web-key-18>
#[derive(Debug, Deserialize, Serialize)]
pub struct JsonWebKey {
    /// Key identifier,
    /// e.g. "https://kv-credibil-vc.vault.azure.net/keys/vc-sign-unlikely-test/4399d1c799db41f6b2cf9920c7d72f14",
    pub kid: String,

    /// JsonWebKey Key Type, e.g. "EC".
    pub kty: String,

    /// Curve, e.g. "P-256K".
    pub crv: String,

    /// X component of an EC public key.
    #[serde(default)]
    pub x: String,

    /// Y component of an EC public key.
    #[serde(default)]
    pub y: String,

    /// Supported key operations e.g. "sign", "verify"
    pub key_ops: Option<Vec<String>>,
}

/// List of versions for a key.
#[derive(Debug, Deserialize)]
pub struct KeyList {
    pub value: Vec<KeyListItem>,
    // #[serde(rename = "nextLink")]
    // pub next_link: Option<String>,
}

/// List item for a list of versions for a key.
#[derive(Debug, Deserialize)]
pub struct KeyListItem {
    pub kid: String,
    pub attributes: KeyAttributes,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn deserialize_key_bundle() {
        let serialized = json!({
            "key": {
                "kid": "https://kv-credibil-vc.vault.azure.net/keys/vc-sign-unlikely-test/4399d1c799db41f6b2cf9920c7d72f14",
                "kty": "EC",
                "key_ops": [
                    "sign",
                    "verify"
                ],
                "crv": "P-256K",
                "x": "jGxSWtojklh8gDrjKaYokMW8b0ZG4gFN4hl_oiKjvfQ",
                "y": "zHbSkGNdH0RLQj6IqLYddqKryRKkPXEGaqlX6Tq2IqI"
            },
            "attributes": {
                "enabled": false,
                "created": 1697601096,
                "updated": 1697601096,
                "recoveryLevel": "Recoverable+Purgeable",
                "recoverableDays": 90
            }
        });
        let deserialized: KeyBundle =
            serde_json::from_value(serialized).expect("failed to deserialize key bundle");
        assert_eq!(deserialized.attributes.enabled, Some(false));
    }

    #[test]
    fn deserialize_deleted_key_bundle() {
        let serialized = json!({
            "recoveryId": "https://kv-credibil-vc.vault.azure.net/deletedkeys/vc-sign-unlikely-test",
            "deletedDate": 1697669021,
            "scheduledPurgeDate": 1705445021,
            "key": {
                "kid": "https://kv-credibil-vc.vault.azure.net/keys/vc-sign-unlikely-test/4399d1c799db41f6b2cf9920c7d72f14",
                "kty": "EC",
                "key_ops": [
                    "sign",
                    "verify"
                ],
                "crv": "P-256K",
                "x": "jGxSWtojklh8gDrjKaYokMW8b0ZG4gFN4hl_oiKjvfQ",
                "y": "zHbSkGNdH0RLQj6IqLYddqKryRKkPXEGaqlX6Tq2IqI"
            },
            "attributes": {
                "enabled": false,
                "created": 1697601096,
                "updated": 1697601096,
                "recoveryLevel": "Recoverable+Purgeable",
                "recoverableDays": 90
            }
        });
        let deserialized: Deleted =
            serde_json::from_value(serialized).expect("failed to deserialize deleted key bundle");
        assert!(deserialized.date.is_some());
        assert_eq!(deserialized.key_bundle.key.kty, "EC");
    }
}
