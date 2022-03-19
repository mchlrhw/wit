mod ast;
mod parselet;

use crate::{
    lexer::{Lexer, Token, TokenKind, Tokens},
    Error, Result,
};
use itertools::{peek_nth, PeekNth};
use parselet::{infix_parselet, prefix_parselet};

pub use ast::{Block, Expr, Stmt};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Lowest,
    Sum,
    Product,
}

pub struct Parser<'source> {
    tokens: PeekNth<Lexer<'source>>,
}

impl<'source> Parser<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            tokens: peek_nth(source.tokens()),
        }
    }

    fn next_precedence(&mut self) -> Precedence {
        if let Some(Ok(token)) = self.tokens.peek() {
            if let Some(parselet) = infix_parselet(token) {
                parselet.precedence()
            } else {
                Precedence::Lowest
            }
        } else {
            Precedence::Lowest
        }
    }

    fn parse_with_precedence(
        &mut self,
        precedence: Precedence,
        token: Token<'source>,
    ) -> Result<Expr<'source>> {
        let prefix = prefix_parselet(&token).ok_or(Error::UnexpectedToken {
            kind: token.kind,
            location: token.span.start,
        })?;

        let mut left = prefix.parse(self, token)?;

        while precedence < self.next_precedence() {
            let token = self.tokens.next().ok_or(Error::UnexpectedEof)??;

            let infix = match infix_parselet(&token) {
                Some(parselet) => parselet,
                None => return Ok(left),
            };

            left = infix.parse(self, left, token)?;
        }

        Ok(left)
    }

    fn parse_expr(&mut self, token: Token<'source>) -> Result<Expr<'source>> {
        self.parse_with_precedence(Precedence::Lowest, token)
    }

    fn parse_stmt(&mut self, token: Token<'source>) -> Result<Stmt<'source>> {
        if matches!(
            token,
            Token {
                kind: TokenKind::Semicolon,
                ..
            }
        ) {
            return Ok(Stmt::Empty { semicolon: token });
        }

        let expr = self.parse_expr(token)?;

        let semicolon = if let Some(Ok(Token {
            kind: TokenKind::Semicolon,
            ..
        })) = self.tokens.peek()
        {
            Some(
                self.tokens
                    .next()
                    .expect("we already checked next is some")?,
            )
        } else {
            None
        };

        Ok(Stmt::Expr { expr, semicolon })
    }

    pub fn parse_block_inner(
        &mut self,
        close_kind: Option<TokenKind>,
    ) -> Result<(Block<'source>, Option<Token<'source>>)> {
        let mut inner = Block::default();

        let close = loop {
            let token = if let Some(close_kind) = close_kind {
                let token = self.tokens.next().ok_or(Error::UnexpectedEof)??;
                if token.kind == close_kind {
                    break token;
                }

                token
            } else {
                let token = self.tokens.next();
                if token.is_none() {
                    return Ok((inner, None));
                }

                token.expect("we already checked next is some")?
            };

            let stmt = self.parse_stmt(token)?;
            match stmt {
                Stmt::Empty { .. } => inner.stmts.push(stmt),
                Stmt::Expr { expr, semicolon } => {
                    if let Some(semicolon) = semicolon {
                        inner.stmts.push(Stmt::Expr {
                            expr,
                            semicolon: Some(semicolon),
                        })
                    } else {
                        if let Some(Ok(token)) = self.tokens.peek() {
                            if Some(token.kind) != close_kind {
                                let unexpected =
                                    self.tokens.next().ok_or(Error::UnexpectedEof)??;
                                return Err(Error::UnexpectedToken {
                                    kind: unexpected.kind,
                                    location: unexpected.span.start,
                                });
                            }
                        }

                        inner.expr = Some(Box::new(expr));
                    }
                }
            }
        };

        Ok((inner, Some(close)))
    }
}
