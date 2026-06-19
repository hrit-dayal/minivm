use std::fmt;

#[derive(Debug)]
pub enum MiniVmError {
    Io(String),
    Parse(String),
    Validation(String),
    Trap(String),
}

impl fmt::Display for MiniVmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(s) => write!(f, "{s}"),
            Self::Parse(s) => write!(f, "{s}"),
            Self::Validation(s) => write!(f, "{s}"),
            Self::Trap(s) => write!(f, "{s}"),
        }
    }
}

impl std::error::Error for MiniVmError {}

impl From<std::io::Error> for MiniVmError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, MiniVmError>;
