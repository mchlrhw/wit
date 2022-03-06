mod lexer;
mod parser;

use lexer::{
    span::{Loc, Span},
    token::{Token, TokenKind},
};
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

#[derive(Debug)]
pub enum Expr<'source> {
    Number(Token<'source>),
    BinOp {
        left: Box<Expr<'source>>,
        op: Token<'source>,
        right: Box<Expr<'source>>,
    },
    Group(Box<Expr<'source>>),
}

impl<'source> Expr<'source> {
    fn span(&self) -> Span {
        use Expr::*;

        match self {
            Number(token) => token.span,
            BinOp { left, right, .. } => {
                let start = left.span().start;
                let end = right.span().end;

                Span { start, end }
            }
            Group(expr) => expr.span(),
        }
    }
}
