use std::str::Chars;

#[derive(Debug)]
enum TokenKind {
    Space,
    LeftBrace,
    RightBrace,
    Identifier,
    Error,
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    size: u32,
}

impl Token {
    fn new(kind: TokenKind, size: u32) -> Self {
        Self { kind, size }
    }
}

struct Lexer<'s> {
    chars: Chars<'s>,
    tokens: Vec<Token>,
}

impl<'s> Lexer<'s> {
    fn new(source: &'s str) -> Self {
        Self {
            chars: source.chars(),
            tokens: Vec::new(),
        }
    }

    fn next(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn push(&mut self, kind: TokenKind, size: u32) {
        self.tokens.push(Token::new(kind, size));
    }

    fn push_single(&mut self, kind: TokenKind) {
        self.push(kind, 1);
    }

    fn identifier(&mut self) {
        let mut size = 1;
        while let Some(ch) = self.next()
            && ch.is_ascii_alphanumeric()
        {
            size += 1;
        }
        self.push(TokenKind::Identifier, size);
    }

    fn go(mut self) -> Vec<Token> {
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

fn main() {
    let source = " {  } blabla123";
    let tokens = Lexer::new(&source).go();
    println!("{:?}", tokens);
}
