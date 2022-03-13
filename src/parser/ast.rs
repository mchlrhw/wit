use crate::lexer::{Span, Token};

#[derive(Debug)]
pub enum Expr<'source> {
    Number(Token<'source>),
    BinOp {
        left: Box<Expr<'source>>,
        op: Token<'source>,
        right: Box<Expr<'source>>,
    },
    Group {
        open: Token<'source>,
        expr: Box<Expr<'source>>,
        close: Token<'source>,
    },
}

impl<'source> Expr<'source> {
    pub fn span(&self) -> Span {
        use Expr::*;

        match self {
            Number(token) => token.span,
            BinOp { left, right, .. } => {
                let start = left.span().start;
                let end = right.span().end;

                Span { start, end }
            }
            Group { open, close, .. } => {
                let start = open.span.start;
                let end = close.span.end;

                Span { start, end }
            }
        }
    }
}
