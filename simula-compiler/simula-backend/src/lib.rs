pub mod codegen;
pub mod targets;
pub mod optimization;
pub mod llvm;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum BackendError {
    #[error("Code generation error: {0}")]
    CodeGen(String),
    
    #[error("LLVM error: {0}")]
    LLVM(String),
    
    #[error("Target error: {0}")]
    Target(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, BackendError>; 