use super::PrefixParselet;
use crate::{
    lexer::{Token, TokenKind},
    parser::{Expr, Parser},
    Error, Result,
};

pub struct Parselet;

impl<'source> PrefixParselet<'source> for Parselet {
    fn parse(&self, parser: &mut Parser<'source>, token: Token<'source>) -> Result<Expr<'source>> {
        let open = token;
        let expr = Box::new(parser.parse()?);
        let close = parser.tokens.next().ok_or(Error::UnexpectedEof)??;

        if !matches!(close.kind, TokenKind::RightParen) {
            return Err(Error::UnclosedGroup {
                location: expr.span().end,
            });
        }

        Ok(Expr::Group { open, expr, close })
    }
}
