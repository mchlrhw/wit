use itertools::{peek_nth, PeekNth};
use std::{fmt, str::Chars};

#[derive(Clone, Debug, thiserror::Error)]
enum Error {
    #[error("unclosed group at {location}")]
    UnclosedGroup { location: Loc },

    #[error("unexpected char '{character}' at {location}")]
    UnexpectedChar { character: char, location: Loc },

    #[error("unexpected EOF")]
    UnexpectedEof,

    #[error("unexpected token of kind '{kind}' at {location}")]
    UnexpectedToken { kind: TokenKind, location: Loc },
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Copy, Clone, Debug)]
struct Loc {
    byte_offset: usize,
    line: usize,
    col: usize,
}

impl Default for Loc {
    fn default() -> Self {
        Self {
            byte_offset: 0,
            line: 1,
            col: 1,
        }
    }
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line: {} column: {}", self.line, self.col)
    }
}

#[derive(Copy, Clone, Default, Debug)]
struct Span {
    start: Loc,
    end: Loc,
}

#[derive(Copy, Clone, Debug)]
enum TokenKind {
    LeftParen,
    RightParen,
    Plus,
    Minus,
    Slash,
    Asterisk,

    Number,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug)]
struct Token<'source> {
    kind: TokenKind,
    lexeme: &'source str,
    span: Span,
}

impl<'source> Token<'source> {
    fn prefix_parselet(&self) -> Option<Box<dyn PrefixParselet<'source>>> {
        use TokenKind::*;

        match self.kind {
            Number => Some(Box::new(NumberParselet)),
            LeftParen => Some(Box::new(GroupParselet)),
            _ => None,
        }
    }

    fn infix_parselet(&self) -> Option<Box<dyn InfixParselet<'source>>> {
        match self.kind {
            TokenKind::Plus | TokenKind::Minus | TokenKind::Slash | TokenKind::Asterisk => {
                Some(Box::new(BinOpParselet { kind: self.kind }))
            }
            _ => None,
        }
    }
}

struct Lexer<'source> {
    source: &'source str,
    chars: PeekNth<Chars<'source>>,
    span: Span,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        let chars = peek_nth(source.chars());
        let span = Span::default();

        Self {
            source,
            chars,
            span,
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn advance(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn extend_span(&mut self, c: char) {
        if c == '\n' {
            self.span.end.line += 1;
            self.span.end.col = 1;
        } else {
            self.span.end.col += 1;
        }
        self.span.end.byte_offset += c.len_utf8();
    }

    fn close_span(&mut self) {
        self.span.start = self.span.end;
    }

    fn move_span(&mut self, c: char) {
        self.extend_span(c);
        self.close_span();
    }

    fn get_lexeme(&mut self) -> &'source str {
        &self.source[self.span.start.byte_offset..=self.span.end.byte_offset]
    }

    fn single_char(&mut self, c: char, kind: TokenKind) -> Result<Token<'source>> {
        let token = Token {
            kind,
            lexeme: self.get_lexeme(),
            span: self.span,
        };

        self.move_span(c);

        Ok(token)
    }

    fn number(&mut self, c: char) -> Result<Token<'source>> {
        let mut previous = c;
        while matches!(self.peek(), Some(c) if c.is_digit(10)) {
            self.extend_span(previous);
            if let Some(c) = self.advance() {
                previous = c;
            }
        }

        if matches!(self.peek(), Some(c) if *c == '.') {
            self.extend_span(previous);
            if let Some(c) = self.advance() {
                previous = c;
            }

            while matches!(self.peek(), Some(c) if c.is_digit(10)) {
                self.extend_span(previous);
                if let Some(c) = self.advance() {
                    previous = c;
                }
            }
        }

        let token = Token {
            kind: TokenKind::Number,
            lexeme: self.get_lexeme(),
            span: self.span,
        };

        self.move_span(previous);

        Ok(token)
    }

    fn unexpected_char(&mut self, c: char) -> Result<Token<'source>> {
        let error = Error::UnexpectedChar {
            character: c,
            location: self.span.end,
        };

        self.move_span(c);

        Err(error)
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = Result<Token<'source>>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.advance() {
            let res = match c {
                '(' => self.single_char(c, TokenKind::LeftParen),
                ')' => self.single_char(c, TokenKind::RightParen),
                '+' => self.single_char(c, TokenKind::Plus),
                '-' => self.single_char(c, TokenKind::Minus),
                '*' => self.single_char(c, TokenKind::Asterisk),
                '/' => self.single_char(c, TokenKind::Slash),
                c if c.is_digit(10) => self.number(c),
                c if c.is_whitespace() => {
                    self.move_span(c);
                    continue;
                }
                _ => self.unexpected_char(c),
            };

            return Some(res);
        }

        None
    }
}

trait Tokens<'source> {
    fn tokens(self) -> Lexer<'source>;
}

impl<'source> Tokens<'source> for &'source str {
    fn tokens(self) -> Lexer<'source> {
        Lexer::new(self)
    }
}

#[derive(Debug)]
enum Expr<'source> {
    Number(Token<'source>),
    BinOp {
        left: Box<Expr<'source>>,
        op: Token<'source>,
        right: Box<Expr<'source>>,
    },
    Group(Box<Expr<'source>>),
}

impl<'source> Expr<'source> {
    fn span(&self) -> Span {
        use Expr::*;

        match self {
            Number(token) => token.span,
            BinOp { left, right, .. } => {
                let start = left.span().start;
                let end = right.span().end;

                Span { start, end }
            }
            Group(expr) => expr.span(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    Lowest,
    Sum,
    Product,
}

trait PrefixParselet<'source> {
    fn parse(&self, parser: &mut Parser<'source>, token: Token<'source>) -> Result<Expr<'source>>;
}

trait InfixParselet<'source> {
    fn precedence(&self) -> Precedence;

    fn parse(
        &self,
        parser: &mut Parser<'source>,
        left: Expr<'source>,
        token: Token<'source>,
    ) -> Result<Expr<'source>>;
}

struct NumberParselet;

impl<'source> PrefixParselet<'source> for NumberParselet {
    fn parse(&self, _parser: &mut Parser<'source>, token: Token<'source>) -> Result<Expr<'source>> {
        Ok(Expr::Number(token))
    }
}

struct GroupParselet;

impl<'source> PrefixParselet<'source> for GroupParselet {
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

struct BinOpParselet {
    kind: TokenKind,
}

impl<'source> InfixParselet<'source> for BinOpParselet {
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
        let right = Box::new(parser.parse_with_precedence(self.precedence())?);

        Ok(Expr::BinOp {
            left: Box::new(left),
            op: token,
            right,
        })
    }
}

struct Parser<'source> {
    tokens: PeekNth<Lexer<'source>>,
}

impl<'source> Parser<'source> {
    fn new(source: &'source str) -> Self {
        Self {
            tokens: peek_nth(source.tokens()),
        }
    }

    fn get_next_precedence(&mut self) -> Precedence {
        if let Some(Ok(token)) = self.tokens.peek() {
            if let Some(parselet) = token.infix_parselet() {
                parselet.precedence()
            } else {
                Precedence::Lowest
            }
        } else {
            Precedence::Lowest
        }
    }

    fn parse_with_precedence(&mut self, precedence: Precedence) -> Result<Expr<'source>> {
        let token = self.tokens.next().ok_or(Error::UnexpectedEof)??;

        let prefix = token.prefix_parselet().ok_or(Error::UnexpectedToken {
            kind: token.kind,
            location: token.span.start,
        })?;

        let mut left = prefix.parse(self, token)?;

        while precedence < self.get_next_precedence() {
            let token = self.tokens.next().ok_or(Error::UnexpectedEof)??;

            let infix = match token.infix_parselet() {
                Some(parselet) => parselet,
                None => return Ok(left),
            };

            left = infix.parse(self, left, token)?;
        }

        Ok(left)
    }

    fn parse(&mut self) -> Result<Expr<'source>> {
        self.parse_with_precedence(Precedence::Lowest)
    }
}

fn main() -> anyhow::Result<()> {
    let source = "1.2 - 34 * 5 - 6.7 / 89";
    let expr = Parser::new(source).parse()?;
    println!("{expr:#?}");

    Ok(())
}
