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
    Plus,
    Minus,
    Slash,
    Asterisk,

    Number,
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

fn main() {
    for token in "1.2 + 34\n5 + 6\n78. + 90\n".tokens() {
        println!("{token:#?}");
    }
}
