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
        let next = parser.tokens.next().ok_or(Error::UnexpectedEof)??;
        let (expr, close) = if let Token {
            kind: TokenKind::RightParen,
            ..
        } = next
        {
            (None, next)
        } else {
            let expr = Box::new(parser.parse_expr(next)?);
            let close = parser.tokens.next().ok_or(Error::UnexpectedEof)??;

            (Some(expr), close)
        };

        if !matches!(close.kind, TokenKind::RightParen) {
            return Err(Error::UnclosedGroup {
                location: close.span.start,
            });
        }

        Ok(Expr::Group { open, expr, close })
    }
}
