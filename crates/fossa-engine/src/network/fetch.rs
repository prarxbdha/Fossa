use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FetchError {
    UnsupportedScheme(String),
    NotImplemented,
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedScheme(url) => write!(f, "unsupported URL: {url}"),
            Self::NotImplemented => write!(f, "network fetching is not implemented yet"),
        }
    }
}

impl std::error::Error for FetchError {}

pub fn fetch(url: &str) -> Result<String, FetchError> {
    if url.starts_with("http://") || url.starts_with("https://") {
        Err(FetchError::NotImplemented)
    } else {
        Err(FetchError::UnsupportedScheme(url.to_owned()))
    }
}
