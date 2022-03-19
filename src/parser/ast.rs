use crate::lexer::{Span, Token};

#[derive(Debug)]
pub enum Expr<'source> {
    BinOp {
        left: Box<Expr<'source>>,
        op: Token<'source>,
        right: Box<Expr<'source>>,
    },
    Block {
        open: Token<'source>,
        inner: Block<'source>,
        close: Token<'source>,
    },
    Group {
        open: Token<'source>,
        expr: Option<Box<Expr<'source>>>,
        close: Token<'source>,
    },
    Number(Token<'source>),
}

impl<'source> Expr<'source> {
    pub fn span(&self) -> Span {
        use Expr::*;

        match self {
            BinOp { left, right, .. } => {
                let start = left.span().start;
                let end = right.span().end;

                Span { start, end }
            }
            Block { open, close, .. } | Group { open, close, .. } => {
                let start = open.span.start;
                let end = close.span.end;

                Span { start, end }
            }
            Number(token) => token.span,
        }
    }
}

#[derive(Debug)]
pub enum Stmt<'source> {
    Empty {
        semicolon: Token<'source>,
    },
    Expr {
        expr: Expr<'source>,
        semicolon: Option<Token<'source>>,
    },
}

impl<'source> Stmt<'source> {
    pub fn span(&self) -> Span {
        use Stmt::*;

        match self {
            Empty { semicolon } => semicolon.span,
            Expr { expr, semicolon } => {
                let start = expr.span().start;
                let end = if let Some(semicolon) = semicolon {
                    semicolon.span.end
                } else {
                    expr.span().end
                };

                Span { start, end }
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct Block<'source> {
    pub stmts: Vec<Stmt<'source>>,
    pub expr: Option<Box<Expr<'source>>>,
}
