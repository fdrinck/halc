use super::Identifier;

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

    pub fn show(&self, source: &str) -> String {
        let name = self.name.show(source);
        let expression = self.expression.show(source);
        format!("Binding (mut={}) {} = {}", self.mutable, name, expression)
    }
}
