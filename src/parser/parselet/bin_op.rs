use super::InfixParselet;
use crate::{
    lexer::{Token, TokenKind},
    parser::{Expr, Parser, Precedence},
    Error, Result,
};

pub struct Parselet {
    kind: TokenKind,
}

impl Parselet {
    pub fn new(kind: TokenKind) -> Self {
        Self { kind }
    }
}

impl<'source> InfixParselet<'source> for Parselet {
    fn precedence(&self) -> Precedence {
        use TokenKind::*;

        match self.kind {
            Plus | Minus => Precedence::Sum,
            Slash | Asterisk => Precedence::Product,
            _ => unimplemented!(),
        }
    }

    fn parse(
        &self,
        parser: &mut Parser<'source>,
        left: Expr<'source>,
        token: Token<'source>,
    ) -> Result<Expr<'source>> {
        let next = parser.tokens.next().ok_or(Error::UnexpectedEof)??;
        let right = Box::new(parser.parse_with_precedence(self.precedence(), next)?);

        Ok(Expr::BinOp {
            left: Box::new(left),
            op: token,
            right,
        })
    }
}
