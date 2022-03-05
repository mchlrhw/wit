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

        while precedence < self.next_precedence() {
            let token = self.tokens.next().ok_or(Error::UnexpectedEof)??;

            let infix = match token.infix_parselet() {
                Some(parselet) => parselet,
                None => return Ok(left),
            };

            left = infix.parse(self, left, token)?;
        }

        Ok(left)
    }

    pub fn parse(&mut self) -> Result<Expr<'source>> {
        self.parse_with_precedence(Precedence::Lowest)
    }
}
