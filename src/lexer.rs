use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Error,
    KwExport,
    KwFn,
    KwImport,
    KwLet,
    KwMut,
    TokColon,
    TokComma,
    TokDot,
    TokIdentifier,
    TokLeftBrace,
    TokLeftParen,
    TokNewline,
    TokRightBrace,
    TokRightParen,
    TokSemiColon,
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
    source: &'s str,
    chars: Peekable<CharIndices<'s>>,
    tokens: Vec<Token>,
}

impl<'s> Lexer<'s> {
    pub fn new(source: &'s str) -> Self {
        Self {
            source,
            chars: source.char_indices().peekable(),
            tokens: Vec::new(),
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().map(|(_, ch)| *ch)
    }

    fn next(&mut self) -> Option<(usize, char)> {
        self.chars.next()
    }

    fn slice(&self, start: usize, end: usize) -> &str {
        &self.source[start..end]
    }

    fn push(&mut self, kind: TokenKind, size: usize) {
        self.tokens.push(Token::new(kind, size as u32));
    }

    fn push_single(&mut self, kind: TokenKind) {
        self.push(kind, 1);
    }

    fn identifier(&mut self, start: usize) {
        let mut end = start + 1;

        while let Some(ch) = self.peek()
            && ch.is_ascii_alphanumeric()
        {
            end += 1;
            self.next();
        }
        let length = end - start;

        match self.slice(start, end) {
            "export" => self.push(KwExport, length),
            "fn" => self.push(KwFn, length),
            "import" => self.push(KwImport, length),
            "let" => self.push(KwLet, length),
            "mut" => self.push(KwMut, length),
            _ => self.push(TokIdentifier, length),
        }
    }

    fn eol(&mut self) {
        if let Some('\n') = self.peek() {
            self.next();
            self.push(TokNewline, 2);
        } else {
            self.push_single(TokNewline);
        }
    }

    pub fn go(mut self) -> Vec<Token> {
        while let Some((offset, ch)) = self.next() {
            match ch {
                ' ' | '\t' => self.push_single(TokSpace),
                '.' => self.push_single(TokDot),
                ',' => self.push_single(TokComma),
                ';' => self.push_single(TokSemiColon),
                ':' => self.push_single(TokColon),
                '(' => self.push_single(TokLeftParen),
                '{' => self.push_single(TokLeftBrace),
                '}' => self.push_single(TokRightBrace),
                ')' => self.push_single(TokRightParen),
                '\n' => self.push_single(TokNewline),
                '\r' => self.eol(),
                ch if ch.is_ascii_alphabetic() => self.identifier(offset),
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
        let source = ".,;:(){}abc123 \t\\let mut fn\n\r\n\r";
        let actual = Lexer::new(&source).go();
        let expected = [
            Token::new(TokDot, 1),
            Token::new(TokComma, 1),
            Token::new(TokSemiColon, 1),
            Token::new(TokColon, 1),
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
