use super::PrefixParselet;
use crate::{
    lexer::Token,
    parser::{Expr, Parser},
    Result,
};

pub struct Parselet;

impl<'source> PrefixParselet<'source> for Parselet {
    fn parse(&self, _parser: &mut Parser<'source>, token: Token<'source>) -> Result<Expr<'source>> {
        Ok(Expr::Number(token))
    }
}
