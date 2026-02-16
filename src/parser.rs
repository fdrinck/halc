use crate::ast::{Binding, Block, Function, Identifier, Parameter, Span};
use crate::lexer::*;
use std::iter::Peekable;
use std::slice::Iter;

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
        Ok(Identifier::new(Span::new(start, stop)))
    }

    fn parameter(&mut self) -> Result<Parameter, ParserError> {
        let name = self.identifier()?;
        self.expect(TokenKind::TokColon)?;
        let kind = self.identifier()?;
        Ok(Parameter::new(name, kind))
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

    fn binding(&mut self) -> Result<Binding, ParserError> {
        self.expect(TokenKind::KwLet)?;
        let mutable = self.eat(TokenKind::KwMut);
        let name = self.identifier()?;
        self.expect(TokenKind::TokEquals)?;
        let expression = self.identifier()?;
        self.expect(TokenKind::TokSemiColon)?;
        Ok(Binding::new(name, mutable, expression))
    }

    fn block(&mut self) -> Result<Block, ParserError> {
        self.expect(TokenKind::TokLeftBrace)?;
        let mut statements = Vec::new();
        match self.peek().map(|t| t.kind()) {
            Some(TokenKind::KwLet) => statements.push(self.binding()?),
            Some(TokenKind::TokSemiColon) => {
                self.next();
            }
            _ => panic!(),
        }
        self.expect(TokenKind::TokRightBrace)?;
        Ok(Block::new(statements))
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

        let body = self.block()?;
        Ok(Function::new(name, parameter, kind, body))
    }

    pub fn go(&mut self) -> Result<Function, ParserError> {
        self.function()
    }
}
