pub mod properties;
pub mod invariants;
pub mod testing;
pub mod model;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum VerificationError {
    #[error("Property violation: {0}")]
    PropertyViolation(String),
    
    #[error("Invariant violation: {0}")]
    InvariantViolation(String),
    
    #[error("Model error: {0}")]
    ModelError(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, VerificationError>; 