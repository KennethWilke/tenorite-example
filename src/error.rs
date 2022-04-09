use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ExampleError {
    #[error("Invalid key!")]
    InvalidKey(String),

    #[error("Unexpected error!")]
    Unexpected,
}
