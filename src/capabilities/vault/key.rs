
use serde::{Deserialize, Serialize};

/// Key bundle is the main data type transferred to and from the Azure Key Vault API.
#[derive(Debug, Default, Deserialize)]
pub struct KeyBundle {
    #[serde(skip)]
    key_name: String,

    #[serde(rename = "key")]
    public_key: JsonWebKey,

    attributes: KeyAttributes,
}

impl KeyBundle {
    pub fn public_key(&self) -> JsonWebKey {
        self.public_key.clone()
    }

    pub fn sign(&self, msg: &[u8]) -> anyhow::Result<Vec<u8>> {
        todo!()
    }
}

/// The attributes of a key managed by the key vault service.
#[derive(Debug, Default, Deserialize)]
pub struct KeyAttributes {
    pub enabled: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct JsonWebKey {
    pub kid: String,
    pub kty: String,
    pub crv: String,
    pub x: String,
    pub y: String,
}

/// Sign request body.
#[derive(Serialize)]
pub struct SigningRequest {
    /// Algorithm
    pub alg: String,

    /// Message to sign
    pub value: String,
}

/// Signature response.
#[derive(Deserialize)]
pub struct SigningResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    aad: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    iv: Option<String>,

    /// Key identifier
    kid: String,

    // #[serde(skip_serializing_if = "Option::is_none")]
    // tag: Option<String>,
    /// Signature
    pub value: String,
}

/// Error returned from Azure Key Vault.
#[derive(Debug, Deserialize)]
struct AzError {
    error: AzErrorDetail,
}

/// Error returned from Azure Key Vault.
#[derive(Debug, Deserialize)]
struct AzErrorDetail {
    code: String,
    message: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vault::client::KeyClient;

    // Get an access token without error.
    #[tokio::test]
    async fn get_keypair() {
        dotenv::dotenv().ok();

        let client = KeyClient::new("https://kv-credibil-demo.vault.azure.net")
            .expect("should create client");
        let keypair =
            client.get_key("demo-credibil-io-signing-key").await.expect("should get keypair");
        println!("{:?}", keypair);
    }
}
