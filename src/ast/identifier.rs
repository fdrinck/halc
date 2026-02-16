use super::Span;

#[derive(Debug)]
pub struct Identifier {
    name: Span,
}

impl Identifier {
    pub fn new(name: Span) -> Self {
        Self { name }
    }

    pub fn show(&self, source: &str) -> String {
        format!("Identifier {}", self.name.slice(source))
    }
}
