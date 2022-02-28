use itertools::{peek_nth, PeekNth};
use std::{fmt, str::Chars};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("unexpected char '{character}' at {location}")]
    UnexpectedChar { character: char, location: Loc },
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

#[derive(Debug)]
enum TokenKind {
    LeftParen,
    RightParen,
}

#[derive(Debug)]
struct Token<'source> {
    kind: TokenKind,
    lexeme: &'source str,
    span: Span,
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

    fn advance(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn move_span(&mut self, c: char) {
        if c == '\n' {
            self.span.end.line += 1;
            self.span.end.col = 1;
        } else {
            self.span.end.col += 1;
        }
        self.span.end.byte_offset += c.len_utf8();
    }

    fn reset_span(&mut self) {
        self.span.start = self.span.end;
    }

    fn single_char_token(&mut self, c: char, kind: TokenKind) -> Token<'source> {
        let token = Token {
            kind,
            lexeme: &self.source[self.span.start.byte_offset..=self.span.end.byte_offset],
            span: self.span,
        };

        self.move_span(c);
        self.reset_span();

        token
    }

    fn unexpected_char(&mut self, c: char) -> Result<Token<'source>> {
        let error = Error::UnexpectedChar {
            character: c,
            location: self.span.end,
        };

        self.move_span(c);

        return Err(error);
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = Result<Token<'source>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(c) = self.advance() {
                match c {
                    '(' => return Some(Ok(self.single_char_token(c, TokenKind::LeftParen))),
                    ')' => return Some(Ok(self.single_char_token(c, TokenKind::RightParen))),
                    _ => return Some(self.unexpected_char(c)),
                }
            } else {
                return None;
            }
        }
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

fn main() {
    for token in "Hello, Wit!".tokens() {
        println!("{token:?}");
    }
}
