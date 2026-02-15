#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Error,
    KwExport,
    KwFn,
    KwImport,
    KwLet,
    KwMut,
    TokIdentifier,
    TokLeftBrace,
    TokLeftParen,
    TokNewline,
    TokRightBrace,
    TokRightParen,
    TokSpace,
}
use TokenKind::*;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub size: u32,
}

impl Token {
    fn new(kind: TokenKind, size: u32) -> Self {
        Self { kind, size }
    }
}

pub struct Lexer<'s> {
    source: &'s [u8],
    offset: usize,
    tokens: Vec<Token>,
}

impl<'s> Lexer<'s> {
    pub fn new(source: &'s [u8]) -> Self {
        Self {
            source,
            offset: 0,
            tokens: Vec::new(),
        }
    }

    fn offset(&self) -> usize {
        self.offset
    }

    fn current(&mut self) -> Option<u8> {
        self.source.get(self.offset).copied()
    }

    fn peek(&mut self) -> Option<u8> {
        self.source.get(self.offset + 1).copied()
    }

    fn advance(&mut self, offset: usize) {
        self.offset += offset;
    }

    fn bump(&mut self) {
        self.advance(1);
    }

    fn slice(&self, start: usize, end: usize) -> &[u8] {
        &self.source[start..end]
    }

    fn push(&mut self, kind: TokenKind, size: usize) {
        self.tokens.push(Token::new(kind, size as u32));
        self.bump(); // TODO: elegant or ugly?
    }

    fn push_single(&mut self, kind: TokenKind) {
        self.push(kind, 1);
    }

    fn identifier(&mut self) {
        let start = self.offset();
        let mut end = start + 1;

        while let Some(ch) = self.peek()
            && ch.is_ascii_alphanumeric()
        {
            end += 1;
            self.bump();
        }
        let length = end - start;

        match self.slice(start, end) {
            b"export" => self.push(KwExport, length),
            b"fn" => self.push(KwFn, length),
            b"import" => self.push(KwImport, length),
            b"let" => self.push(KwLet, length),
            b"mut" => self.push(KwMut, length),
            _ => self.push(TokIdentifier, length),
        }
    }

    fn eol(&mut self) {
        if let Some(b'\n') = self.peek() {
            self.bump();
            self.push(TokNewline, 2);
        } else {
            self.push_single(TokNewline);
        }
    }

    pub fn go(mut self) -> Vec<Token> {
        while let Some(ch) = self.current() {
            match ch {
                b' ' | b'\t' => self.push_single(TokSpace),
                b'(' => self.push_single(TokLeftParen),
                b'{' => self.push_single(TokLeftBrace),
                b'}' => self.push_single(TokRightBrace),
                b')' => self.push_single(TokRightParen),
                b'\n' => self.push_single(TokNewline),
                b'\r' => self.eol(),
                ch if ch.is_ascii_alphabetic() => self.identifier(),
                _ => self.push_single(Error),
            }
        }
        self.tokens
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        let source = "(){}abc123 \t\\let mut fn\n\r\n\r";
        let actual = Lexer::new(&source.as_bytes()).go();
        let expected = [
            Token::new(TokLeftParen, 1),
            Token::new(TokRightParen, 1),
            Token::new(TokLeftBrace, 1),
            Token::new(TokRightBrace, 1),
            Token::new(TokIdentifier, 6),
            Token::new(TokSpace, 1),
            Token::new(TokSpace, 1),
            Token::new(Error, 1),
            Token::new(KwLet, 3),
            Token::new(TokSpace, 1),
            Token::new(KwMut, 3),
            Token::new(TokSpace, 1),
            Token::new(KwFn, 2),
            Token::new(TokNewline, 1),
            Token::new(TokNewline, 2),
            Token::new(TokNewline, 1),
        ];
        assert_eq!(actual, expected);
    }
}
