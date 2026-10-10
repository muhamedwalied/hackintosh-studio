use thiserror::Error;

/// Core error types for Hackintosh Studio.
#[derive(Debug, Error)]
pub enum CoreError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Platform error: {0}")]
    Platform(String),

    #[error("Hardware detection error: {0}")]
    HardwareDetection(String),

    #[error("Not implemented: {0}")]
    NotImplemented(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Resource not found: {0}")]
    NotFound(String),
}

impl CoreError {
    /// Check if this error is retryable.
    pub fn is_retryable(&self) -> bool {
        matches!(self, CoreError::Io(_) | CoreError::Platform(_))
    }

    /// Get a user-friendly description of what action is needed.
    pub fn user_action(&self) -> Option<&'static str> {
        match self {
            CoreError::PermissionDenied(_) => {
                Some("Run with elevated privileges or check permissions")
            }
            CoreError::NotFound(_) => Some("Verify the resource exists and is accessible"),
            CoreError::InvalidInput(_) => Some("Check input parameters and try again"),
            _ => None,
        }
    }
}

/// Result type alias for core operations.
pub type CoreResult<T> = Result<T, CoreError>;
