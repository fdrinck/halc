use super::Block;
use super::Identifier;

#[derive(Debug)]
pub struct Parameter {
    name: Identifier,
    kind: Identifier,
}

impl Parameter {
    pub fn new(name: Identifier, kind: Identifier) -> Self {
        Self { name, kind }
    }

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

    pub fn show(&self, source: &str) -> String {
        let mut result = "Function\n".to_owned();
        let name = self.name.show(source);
        result.push_str(&format!("  name = {}\n", name));
        for (idx, parameter) in self.parameters.iter().enumerate() {
            let parameter = parameter.show(source);
            result.push_str(&format!("  p[{}] = {}\n", idx, parameter));
        }
        if let Some(kind) = &self.kind {
            let kind = kind.show(source);
            result.push_str(&format!("  kind = {}\n", kind));
        }
        let body = self.body.show(source);
        result.push_str(&format!("  block = {}\n", body));
        result
    }
}
