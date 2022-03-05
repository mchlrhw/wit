use super::PrefixParselet;
use crate::{Result, parser::Parser, Expr, Token};

pub struct Parselet;

impl<'source> PrefixParselet<'source> for Parselet {
    fn parse(&self, _parser: &mut Parser<'source>, token: Token<'source>) -> Result<Expr<'source>> {
        Ok(Expr::Number(token))
    }
}
