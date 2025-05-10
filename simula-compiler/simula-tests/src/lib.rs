pub mod frontend_tests;
pub mod ir_tests;
pub mod backend_tests;
pub mod runtime_tests;
pub mod verification_tests;
pub mod integration_tests;
pub mod benchmarks;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TestError {
    #[error("Test failure: {0}")]
    TestFailure(String),
    
    #[error("Setup error: {0}")]
    SetupError(String),
    
    #[error("Cleanup error: {0}")]
    CleanupError(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, TestError>; 