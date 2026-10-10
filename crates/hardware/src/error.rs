use thiserror::Error;

/// Hardware detection errors.
#[derive(Debug, Error)]
pub enum HardwareError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Platform error: {0}")]
    Platform(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Permission denied accessing {path}: {source}")]
    PermissionDenied {
        path: String,
        source: std::io::Error,
    },

    #[error("Not implemented on this platform: {0}")]
    NotImplemented(String),

    #[error("Unsupported kernel version: {0}")]
    UnsupportedKernel(String),

    #[error("Missing required file: {0}")]
    MissingFile(String),

    #[error("Invalid data: {0}")]
    InvalidData(String),
}

impl HardwareError {
    pub fn permission_denied(path: impl Into<String>, source: std::io::Error) -> Self {
        Self::PermissionDenied {
            path: path.into(),
            source,
        }
    }

    pub fn platform(msg: impl Into<String>) -> Self {
        Self::Platform(msg.into())
    }

    pub fn parse(msg: impl Into<String>) -> Self {
        Self::Parse(msg.into())
    }

    pub fn not_implemented(msg: impl Into<String>) -> Self {
        Self::NotImplemented(msg.into())
    }

    pub fn missing_file(msg: impl Into<String>) -> Self {
        Self::MissingFile(msg.into())
    }

    pub fn invalid_data(msg: impl Into<String>) -> Self {
        Self::InvalidData(msg.into())
    }
}

impl From<HardwareError> for crate::error::CoreError {
    fn from(err: HardwareError) -> Self {
        match err {
            HardwareError::Io(e) => CoreError::Io(e),
            HardwareError::PermissionDenied { path, source } => {
                CoreError::PermissionDenied(format!("{}: {}", path, source))
            }
            HardwareError::NotImplemented(msg) => CoreError::NotImplemented(msg),
            _ => CoreError::HardwareDetection(err.to_string()),
        }
    }
}

type CoreError = hackintosh_studio_core::error::CoreError;
