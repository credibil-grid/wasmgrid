use anyhow::{Result, anyhow};
use http::Uri;

#[derive(Debug, Clone)]
pub enum UriLike {
    Uri(Uri),
    Str(String),
}

impl From<Uri> for UriLike {
    fn from(uri: Uri) -> Self {
        Self::Uri(uri)
    }
}

impl From<&Uri> for UriLike {
    fn from(uri: &Uri) -> Self {
        Self::Uri(uri.clone())
    }
}

impl From<String> for UriLike {
    fn from(uri: String) -> Self {
        Self::Str(uri)
    }
}

impl From<&str> for UriLike {
    fn from(uri: &str) -> Self {
        Self::Str(uri.to_string())
    }
}

impl UriLike {
    /// Attempt to convert the URI-like value into a `Uri`.
    ///
    /// # Errors
    ///
    /// Returns an error if the value cannot be converted into a valid URI.
    pub fn into_uri(&self) -> Result<Uri> {
        match self {
            Self::Uri(uri) => Ok(uri.clone()),
            Self::Str(s) => s.parse::<Uri>().map_err(|_| anyhow!("invalid URI string")),
        }
    }
}
