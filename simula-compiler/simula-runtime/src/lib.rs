pub mod simulation;
pub mod process;
pub mod resource;
pub mod event;
pub mod time;
pub mod memory;
pub mod error;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Process error: {0}")]
    Process(String),
    
    #[error("Resource error: {0}")]
    Resource(String),
    
    #[error("Event error: {0}")]
    Event(String),
    
    #[error("Time error: {0}")]
    Time(String),
    
    #[error("Memory error: {0}")]
    Memory(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, RuntimeError>; 