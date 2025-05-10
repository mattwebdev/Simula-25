pub mod ir;
pub mod passes;
pub mod verification;
pub mod analysis;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum IRError {
    #[error("Invalid IR operation: {0}")]
    InvalidOperation(String),
    
    #[error("Type mismatch: {0}")]
    TypeMismatch(String),
    
    #[error("Verification failed: {0}")]
    VerificationFailed(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, IRError>; 