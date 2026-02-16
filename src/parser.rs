use crate::ast::Span;
use crate::lexer::*;
use std::iter::Peekable;
use std::slice::Iter;

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

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken {
        expected: TokenKind,
        actual: TokenKind,
        span: Span,
    },
    UnexpectedEof {
        offset: usize,
    },
}

pub struct Parser<'s> {
    stream: Peekable<Iter<'s, Token>>,
    offset: usize,
}

// TODO: error recovery by re-syncing
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

    fn expect(&mut self, expected: TokenKind) -> Result<(), ParserError> {
        match self.peek() {
            Some(actual) if actual.kind() == expected => {
                self.next();
                Ok(())
            }
            Some(actual) => {
                let start = self.offset();
                self.next();
                let stop = self.offset();
                Err(ParserError::UnexpectedToken {
                    expected,
                    actual: actual.kind(),
                    span: Span::new(start, stop),
                })
            }
            None => Err(ParserError::UnexpectedEof {
                offset: self.offset(),
            }),
        }
    }

    fn identifier(&mut self) -> Result<Identifier, ParserError> {
        let start = self.offset();
        self.expect(TokenKind::TokIdentifier)?;
        let stop = self.offset();
        Ok(Identifier {
            name: Span::new(start, stop),
        })
    }

    fn parameter(&mut self) -> Result<Parameter, ParserError> {
        let name = self.identifier()?;
        self.expect(TokenKind::TokColon)?;
        let kind = self.identifier()?;
        Ok(Parameter { name, kind })
    }

    // TODO: hard to read... is it worth it?
    fn list<E, R: FnMut(&mut Self) -> Result<E, ParserError>>(
        &mut self,
        mut rule: R,
        left_delim: TokenKind,
        right_delim: TokenKind,
        sep: TokenKind,
    ) -> Result<Vec<E>, ParserError> {
        let mut result = Vec::new();
        self.expect(left_delim)?;
        while !self.eat(right_delim) {
            result.push(rule(self)?);

            if !self.eat(sep) {
                self.expect(right_delim)?;
                break;
            }
        }
        Ok(result)
    }

    fn function(&mut self) -> Result<Function, ParserError> {
        self.expect(TokenKind::KwFn)?;

        let name = self.identifier()?;

        let parameter = self.list(
            Self::parameter,
            TokenKind::TokLeftParen,
            TokenKind::TokRightParen,
            TokenKind::TokComma,
        )?;

        let mut kind = None;
        if self.eat(TokenKind::TokArrow) {
            kind = Some(self.identifier()?);
        }

        self.expect(TokenKind::TokSemiColon)?;
        Ok(Function {
            name,
            parameter,
            kind,
        })
    }

    pub fn go(&mut self) -> Result<Function, ParserError> {
        self.function()
    }
}
