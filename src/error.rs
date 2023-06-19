use thiserror::Error;

/// Error which can occur while rendering a PDF document.
#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    JvmConfigure(#[from] jni::JvmError),
    #[error("{0}")]
    JvmStart(#[from] jni::errors::StartJvmError),
    #[error("{0}")]
    Jni(#[from] jni::errors::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0:?}")]
    ColorTransform(colors_transform::ParseError),
}
