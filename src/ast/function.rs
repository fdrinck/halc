use super::{Block, Identifier, Show};
use std::fmt::Write;

#[derive(Debug)]
pub struct Parameter {
    name: Identifier,
    kind: Identifier,
}

impl Parameter {
    pub fn new(name: Identifier, kind: Identifier) -> Self {
        Self { name, kind }
    }
}

impl Show for Parameter {
    fn show(&self, source: &str, level: usize, buffer: &mut String) {
        let indent = Self::get_indent(level);
        writeln!(buffer, "{:indent$}Parameter", "",).unwrap();
        self.name.show(source, level + 1, buffer);
        self.kind.show(source, level + 1, buffer);
    }
}

#[derive(Debug)]
pub struct Function {
    name: Identifier,
    parameters: Vec<Parameter>,
    kind: Option<Identifier>,
    body: Block,
}

impl Function {
    pub fn new(
        name: Identifier,
        parameters: Vec<Parameter>,
        kind: Option<Identifier>,
        body: Block,
    ) -> Self {
        Self {
            name,
            parameters,
            kind,
            body,
        }
    }
}

impl Show for Function {
    fn show(&self, source: &str, level: usize, buffer: &mut String) {
        let indent = Self::get_indent(level);
        writeln!(buffer, "{:indent$}Function", "").unwrap();
        self.name.show(source, level + 1, buffer);
        for parameter in &self.parameters {
            parameter.show(source, level + 1, buffer);
        }
        if let Some(kind) = &self.kind {
            kind.show(source, level + 1, buffer);
        }
        self.body.show(source, level + 1, buffer);
    }
}
