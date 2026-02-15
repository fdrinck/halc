use crate::lexer::*;
use std::iter::Peekable;
use std::slice::Iter;

#[derive(Debug, Copy, Clone)]
pub struct Span(u64);

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        assert!(start >> 48 == 0);
        assert!(start <= end);
        let length = end - start;
        assert!(length >> 16 == 0);
        Self(((start as u64) << 16) | (length & 0xFFFF) as u64)
    }

    pub fn start(&self) -> usize {
        (self.0 >> 16) as usize
    }

    pub fn length(&self) -> usize {
        (self.0 & 0xFFFF) as usize
    }

    pub fn end(&self) -> usize {
        self.length() + self.start()
    }

    pub fn slice<'s>(&self, source: &'s str) -> &'s str {
        &source[self.start()..self.end()]
    }
}

#[derive(Debug)]
pub struct Identifier {
    name: Span,
}

impl Identifier {
    pub fn show(&self, source: &str) -> String {
        format!("Identifier {}", self.name.slice(source))
    }
}

#[derive(Debug)]
pub struct Parameter {
    name: Identifier,
    kind: Identifier,
}

impl Parameter {
    pub fn show(&self, source: &str) -> String {
        format!(
            "Parameter {} : {}",
            self.name.show(source),
            self.kind.show(source)
        )
    }
}

#[derive(Debug)]
pub struct Function {
    name: Identifier,
    parameter: Vec<Parameter>,
    kind: Option<Identifier>,
}

impl Function {
    pub fn show(&self, source: &str) -> String {
        let mut result = "Function\n".to_owned();
        let name = self.name.show(source);
        result.push_str(&format!("  name = {}\n", name));
        for (idx, parameter) in self.parameter.iter().enumerate() {
            let parameter = parameter.show(source);
            result.push_str(&format!("  p[{}] = {}\n", idx, parameter));
        }
        if let Some(kind) = &self.kind {
            let kind = kind.show(source);
            result.push_str(&format!("  kind = {}\n", kind));
        }
        result
    }
}

pub struct Parser<'s> {
    stream: Peekable<Iter<'s, Token>>,
    offset: usize,
}

impl<'s> Parser<'s> {
    pub fn new(tokens: &'s [Token]) -> Self {
        let mut result = Self {
            stream: tokens.iter().peekable(),
            offset: 0,
        };
        result.skip_trivia();
        result
    }

    fn offset(&self) -> usize {
        self.offset
    }

    fn incr_offset(&mut self, delta: usize) {
        self.offset += delta;
    }

    fn skip_trivia(&mut self) {
        while let Some(t) = self.stream.next_if(|&t| t.is_ws()) {
            self.incr_offset(t.size());
        }
    }

    fn peek(&mut self) -> Option<Token> {
        self.stream.peek().copied().copied()
    }

    fn eat(&mut self, kind: TokenKind) -> bool {
        if let Some(actual) = self.peek()
            && actual.kind() == kind
        {
            self.next();
            true
        } else {
            false
        }
    }

    fn next(&mut self) -> Option<Token> {
        let result = self.stream.next().copied();
        if let Some(token) = result {
            self.incr_offset(token.size());
        }
        self.skip_trivia();
        result
    }

    fn expect(&mut self, expected: TokenKind) {
        if let Some(actual) = self.peek()
            && actual.kind() == expected
        {
            self.next();
        } else {
            panic!()
        }
    }

    fn identifier(&mut self) -> Identifier {
        let start = self.offset();
        self.expect(TokenKind::TokIdentifier);
        let stop = self.offset();
        Identifier {
            name: Span::new(start, stop),
        }
    }

    fn parameter(&mut self) -> Parameter {
        let name = self.identifier();
        self.expect(TokenKind::TokColon);
        let kind = self.identifier();
        Parameter { name, kind }
    }

    // TODO: hard to read... is it worth it?
    fn list<R, A>(
        &mut self,
        mut rule: R,
        left_delim: TokenKind,
        right_delim: TokenKind,
        sep: TokenKind,
    ) -> Vec<A>
    where
        R: FnMut(&mut Self) -> A,
    {
        let mut result = Vec::new();
        self.expect(left_delim);
        if !self.eat(right_delim) {
            loop {
                result.push(rule(self));

                if !self.eat(sep) {
                    self.expect(right_delim);
                    break;
                }

                // trailing sep is ok
                if self.eat(right_delim) {
                    break;
                }
            }
        }
        result
    }

    fn function(&mut self) -> Function {
        self.expect(TokenKind::KwFn);

        let name = self.identifier();

        let parameter = self.list(
            Self::parameter,
            TokenKind::TokLeftParen,
            TokenKind::TokRightParen,
            TokenKind::TokComma,
        );

        let mut kind = None;
        if self.eat(TokenKind::TokArrow) {
            kind = Some(self.identifier());
        }

        self.expect(TokenKind::TokSemiColon);
        Function {
            name,
            parameter,
            kind,
        }
    }

    pub fn go(&mut self) -> Function {
        self.function()
    }
}
