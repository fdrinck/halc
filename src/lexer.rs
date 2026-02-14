use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Space,
    LeftBrace,
    RightBrace,
    Identifier,
    Error,
}

// TODO: struct-of-array?
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
    chars: Chars<'s>,
    tokens: Vec<Token>,
}

impl<'s> Lexer<'s> {
    pub fn new(source: &'s str) -> Self {
        Self {
            chars: source.chars(),
            tokens: Vec::new(),
        }
    }

    fn next(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.clone().next() // TODO: overhead?
    }

    fn push(&mut self, kind: TokenKind, size: u32) {
        self.tokens.push(Token::new(kind, size));
    }

    fn push_single(&mut self, kind: TokenKind) {
        self.push(kind, 1);
    }

    fn identifier(&mut self) {
        let mut size = 1;
        while let Some(ch) = self.peek()
            && ch.is_ascii_alphanumeric()
        {
            size += 1;
            self.next();
        }
        self.push(TokenKind::Identifier, size);
    }

    pub fn go(mut self) -> Vec<Token> {
        while let Some(ch) = self.next() {
            match ch {
                ' ' | '\t' => self.push_single(TokenKind::Space),
                '{' => self.push_single(TokenKind::LeftBrace),
                '}' => self.push_single(TokenKind::RightBrace),
                ch if ch.is_ascii_alphabetic() => self.identifier(),
                _ => self.push_single(TokenKind::Error),
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
        let source = "{}abc \tä";
        let actual = Lexer::new(&source).go();
        use TokenKind::*;
        let expected = [
            Token::new(LeftBrace, 1),
            Token::new(RightBrace, 1),
            Token::new(Identifier, 3),
            Token::new(Space, 1),
            Token::new(Space, 1),
            Token::new(Error, 1),
        ];
        assert_eq!(actual, expected);
    }
}
