mod lexer;
mod parser;

use lexer::{Loc, Span, TokenKind};
pub use parser::Parser;

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
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
