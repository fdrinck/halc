use super::Binding;

#[derive(Debug)]
pub struct Block {
    statements: Vec<Binding>,
}

impl Block {
    pub fn new(statements: Vec<Binding>) -> Self {
        Self { statements }
    }

    pub fn show(&self, source: &str) -> String {
        let mut result = "Block\n".to_owned();
        for statement in &self.statements {
            result.push_str(&format!("{}", statement.show(source)));
        }
        result
    }
}
