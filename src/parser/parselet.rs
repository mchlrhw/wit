mod bin_op;
mod group;
mod number;

use super::{Expr, Parser, Precedence};
use crate::{
    lexer::{Token, TokenKind},
    Result,
};

pub trait PrefixParselet<'source> {
    fn parse(&self, parser: &mut Parser<'source>, token: Token<'source>) -> Result<Expr<'source>>;
}

pub trait InfixParselet<'source> {
    fn precedence(&self) -> Precedence;

    fn parse(
        &self,
        parser: &mut Parser<'source>,
        left: Expr<'source>,
        token: Token<'source>,
    ) -> Result<Expr<'source>>;
}

pub fn prefix_parselet<'source>(
    token: &Token<'source>,
) -> Option<Box<dyn PrefixParselet<'source>>> {
    use TokenKind::*;

    match token.kind {
        Number => Some(Box::new(number::Parselet)),
        LeftParen => Some(Box::new(group::Parselet)),
        _ => None,
    }
}

pub fn infix_parselet<'source>(token: &Token<'source>) -> Option<Box<dyn InfixParselet<'source>>> {
    match token.kind {
        TokenKind::Plus | TokenKind::Minus | TokenKind::Slash | TokenKind::Asterisk => {
            Some(Box::new(bin_op::Parselet::new(token.kind)))
        }
        _ => None,
    }
}
