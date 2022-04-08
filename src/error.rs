use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ExampleError {
    #[error("Unexpected error!")]
    Unexpected,
}
