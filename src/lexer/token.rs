use crate::Span;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,

    Plus,
    Minus,
    Slash,
    Asterisk,

    Number,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug)]
pub struct Token<'source> {
    pub kind: TokenKind,
    pub lexeme: &'source str,
    pub span: Span,
}
