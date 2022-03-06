pub mod codegen;
mod lexer;
mod parser;

use inkwell::execution_engine::FunctionLookupError;
use lexer::{Loc, Span, TokenKind};

pub use parser::Parser;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("execution engine error: {message}")]
    ExecutionEngine { message: String },

    #[error(transparent)]
    FunctionLookup(#[from] FunctionLookupError),

    #[error("llvm error: {message}")]
    Llvm { message: String },

    #[error("unclosed group at {location}")]
    UnclosedGroup { location: Loc },

    #[error("unexpected char '{character}' at {location}")]
    UnexpectedChar { character: char, location: Loc },

    #[error("unexpected EOF")]
    UnexpectedEof,

    #[error("unexpected token of kind '{kind}' at {location}")]
    UnexpectedToken { kind: TokenKind, location: Loc },
}

type Result<T> = std::result::Result<T, Error>;
