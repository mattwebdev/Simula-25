pub mod lexer;
pub mod parser;
pub mod ast;
pub mod type_checker;
pub mod diagnostics;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum FrontendError {
    #[error("Lexical error: {0}")]
    Lexical(String),
    
    #[error("Syntax error: {0}")]
    Syntax(String),
    
    #[error("Type error: {0}")]
    Type(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, FrontendError>; 