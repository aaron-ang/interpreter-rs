use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoxError {
    #[error("[line {line}] Error at '{lexeme}': {message}")]
    SyntaxError {
        line: usize,
        lexeme: String,
        message: String,
    },

    #[error("[line {line}] Error at end: {message}")]
    SyntaxErrorAtEnd { line: usize, message: String },

    #[error("{0}")]
    TypeError(String),

    #[error("Undefined variable '{name}'.\n[line {line}]")]
    UndefinedVariable { name: String, line: usize },

    #[error("Undefined property '{0}'.")]
    UndefinedProperty(String),

    #[error("Expected {expected} arguments but got {got}.")]
    ArgumentCountError { expected: usize, got: usize },
}
