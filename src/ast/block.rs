use super::{Binding, Show};
use std::fmt::Write;

#[derive(Debug)]
pub struct Block {
    statements: Vec<Binding>,
}

impl Block {
    pub fn new(statements: Vec<Binding>) -> Self {
        Self { statements }
    }
}

impl Show for Block {
    fn show(&self, source: &str, level: usize, buffer: &mut String) {
        let indent = Self::get_indent(level);
        writeln!(buffer, "{:indent$}Block", "").unwrap();
        for statement in &self.statements {
            statement.show(source, level + 1, buffer);
        }
    }
}
