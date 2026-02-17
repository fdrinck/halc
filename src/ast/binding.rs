use super::{Identifier, Show};
use std::fmt::Write;

#[derive(Debug)]
pub struct Binding {
    name: Identifier,
    mutable: bool,
    expression: Identifier,
}

impl Binding {
    pub fn new(name: Identifier, mutable: bool, expression: Identifier) -> Self {
        Self {
            name,
            mutable,
            expression,
        }
    }
}

impl Show for Binding {
    fn show(&self, source: &str, level: usize, buffer: &mut String) {
        let indent = Self::get_indent(level);
        writeln!(buffer, "{:indent$}Binding (mut={})", "", self.mutable).unwrap();
        self.name.show(source, level + 1, buffer);
        self.expression.show(source, level + 1, buffer);
    }
}
