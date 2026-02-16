#[derive(Debug)]
pub struct Block {}

impl Block {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&self, source: &str) -> String {
        format!("Block")
    }
}
