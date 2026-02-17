use super::{Show, Span};
use std::fmt::Write;

#[derive(Debug)]
pub struct Identifier {
    name: Span,
}

impl Identifier {
    pub fn new(name: Span) -> Self {
        Self { name }
    }
}

impl Show for Identifier {
    fn show(&self, source: &str, level: usize, buffer: &mut String) {
        let indent = Self::get_indent(level);
        writeln!(
            buffer,
            "{:indent$}Identifier {}",
            "",
            self.name.slice(source)
        )
        .unwrap();
    }
}
