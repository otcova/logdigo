use derive_more::{Display, From};

#[derive(Debug, Display, Clone, From)]
pub enum ModError {
    Str(String),
}

impl From<&str> for ModError {
    fn from(value: &str) -> Self {
        Self::Str(value.into())
    }
}

impl std::error::Error for ModError {}
