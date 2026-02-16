use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    Error,
    KwExport,
    KwFn,
    KwImport,
    KwLet,
    KwMut,
    TokArrow,
    TokColon,
    TokComma,
    TokDot,
    TokEquals,
    TokIdentifier,
    TokLeftAngle,
    TokLeftBrace,
    TokLeftParen,
    TokMinus,
    TokMinusEquals,
    TokNewline,
    TokPlus,
    TokPlusEquals,
    TokRightAngle,
    TokRightBrace,
    TokRightParen,
    TokSemiColon,
    TokSpace,
    TokUnderscore,
}
use TokenKind::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token {
    kind: TokenKind,
    size: u16, // match to max span length
}

impl Token {
    fn new(kind: TokenKind, size: u16) -> Self {
        Self { kind, size }
    }

    pub fn is_ws(&self) -> bool {
        matches!(self.kind, TokSpace | TokNewline)
    }

    pub fn size(&self) -> usize {
        self.size as usize
    }

    pub fn kind(&self) -> TokenKind {
        self.kind
    }
}

pub struct Lexer<'s> {
    source: &'s str,
    chars: Peekable<CharIndices<'s>>,
    tokens: Vec<Token>,
}

fn is_ident_ch(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

impl<'s> Lexer<'s> {
    pub fn new(source: &'s str) -> Self {
        Self {
            source,
            chars: source.char_indices().peekable(),
            tokens: Vec::new(),
        }
    }

    fn peek(&mut self) -> Option<(usize, char)> {
        self.chars.peek().copied()
    }

    fn next(&mut self) -> Option<(usize, char)> {
        self.chars.next()
    }

    fn slice(&self, start: usize, end: usize) -> &str {
        &self.source[start..end]
    }

    fn push(&mut self, kind: TokenKind, size: usize) {
        self.tokens.push(Token::new(kind, size.try_into().unwrap()));
    }

    fn push_single(&mut self, kind: TokenKind) {
        self.push(kind, 1);
    }

    fn identifier(&mut self, start: usize) {
        let mut end = start + 1;

        while let Some((new_end, ch)) = self.peek()
            && is_ident_ch(ch)
        {
            end = new_end + 1;
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
        if let Some((_, '\n')) = self.peek() {
            self.next();
            self.push(TokNewline, 2);
        } else {
            self.push_single(TokNewline);
        }
    }

    fn minus(&mut self) {
        match self.peek() {
            Some((_, '>')) => {
                self.next();
                self.push(TokArrow, 2);
            }
            Some((_, '=')) => {
                self.next();
                self.push(TokMinusEquals, 2);
            }
            _ => {
                self.push_single(TokMinus);
            }
        }
    }

    fn plus(&mut self) {
        match self.peek() {
            Some((_, '=')) => {
                self.next();
                self.push(TokPlusEquals, 2);
            }
            _ => {
                self.push_single(TokPlus);
            }
        }
    }

    fn underscore(&mut self, offset: usize) {
        if let Some((_, ch)) = self.peek()
            && !is_ident_ch(ch)
        {
            self.push_single(TokUnderscore);
        } else {
            // "_1", "__" and other strange names are allowed
            self.identifier(offset);
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
                '=' => self.push_single(TokEquals),
                '(' => self.push_single(TokLeftParen),
                '{' => self.push_single(TokLeftBrace),
                '<' => self.push_single(TokLeftAngle),
                '}' => self.push_single(TokRightBrace),
                ')' => self.push_single(TokRightParen),
                '>' => self.push_single(TokRightAngle),
                '\n' => self.push_single(TokNewline),
                '\r' => self.eol(),
                '-' => self.minus(),
                '+' => self.plus(),
                '_' => self.underscore(offset),
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
        let source = ".,;:=(){}<>--=->++=abc123 _1_ _\t\\let mut fn\n\r\n\r";
        let actual = Lexer::new(&source).go();
        let expected = [
            Token::new(TokDot, 1),
            Token::new(TokComma, 1),
            Token::new(TokSemiColon, 1),
            Token::new(TokColon, 1),
            Token::new(TokEquals, 1),
            Token::new(TokLeftParen, 1),
            Token::new(TokRightParen, 1),
            Token::new(TokLeftBrace, 1),
            Token::new(TokRightBrace, 1),
            Token::new(TokLeftAngle, 1),
            Token::new(TokRightAngle, 1),
            Token::new(TokMinus, 1),
            Token::new(TokMinusEquals, 2),
            Token::new(TokArrow, 2),
            Token::new(TokPlus, 1),
            Token::new(TokPlusEquals, 2),
            Token::new(TokIdentifier, 6),
            Token::new(TokSpace, 1),
            Token::new(TokIdentifier, 3),
            Token::new(TokSpace, 1),
            Token::new(TokUnderscore, 1),
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
