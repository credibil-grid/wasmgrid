use anyhow::{Result, anyhow};
use http::Uri;

#[derive(Debug, Clone)]
pub enum UriLike {
    Uri(Uri),
    Str(String),
}

impl From<Uri> for UriLike {
    fn from(uri: Uri) -> Self {
        UriLike::Uri(uri)
    }
}

impl From<&Uri> for UriLike {
    fn from(uri: &Uri) -> Self {
        UriLike::Uri(uri.clone())
    }
}

impl From<String> for UriLike {
    fn from(uri: String) -> Self {
        UriLike::Str(uri)
    }
}

impl From<&str> for UriLike {
    fn from(uri: &str) -> Self {
        UriLike::Str(uri.to_string())
    }
}

impl UriLike {
    pub fn into_uri(&self) -> Result<Uri> {
        match self {
            UriLike::Uri(uri) => Ok(uri.clone()),
            UriLike::Str(s) => s.parse::<Uri>().map_err(|_| anyhow!("invalid URI string")),
        }
    }
}
