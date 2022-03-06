use crate::lexer::{Span, Token};

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
    pub fn span(&self) -> Span {
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
