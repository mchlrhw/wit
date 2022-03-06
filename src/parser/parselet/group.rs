use super::PrefixParselet;
use crate::{
    lexer::{Token, TokenKind},
    parser::{Expr, Parser},
    Error, Result,
};

pub struct Parselet;

impl<'source> PrefixParselet<'source> for Parselet {
    fn parse(&self, parser: &mut Parser<'source>, _token: Token<'source>) -> Result<Expr<'source>> {
        let expr = Box::new(parser.parse()?);
        if !matches!(
            parser.tokens.next(),
            Some(Ok(Token {
                kind: TokenKind::RightParen,
                ..
            }))
        ) {
            return Err(Error::UnclosedGroup {
                location: expr.span().end,
            });
        }

        Ok(Expr::Group(expr))
    }
}
