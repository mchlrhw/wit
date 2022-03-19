use super::PrefixParselet;
use crate::{
    lexer::{Token, TokenKind},
    parser::{Expr, Parser},
    Result,
};

pub struct Parselet;

impl<'source> PrefixParselet<'source> for Parselet {
    fn parse(&self, parser: &mut Parser<'source>, token: Token<'source>) -> Result<Expr<'source>> {
        let open = token;
        let (inner, close) = parser.parse_block_inner(Some(TokenKind::RightBrace))?;

        Ok(Expr::Block {
            open,
            inner,
            close: close.expect("we get a close if we specify some close kind"),
        })
    }
}
