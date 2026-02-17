mod binding;
mod block;
mod function;
mod identifier;
mod span;

pub use binding::Binding;
pub use block::Block;
pub use function::{Function, Parameter};
pub use identifier::Identifier;
pub use span::Span;

pub trait Show {
    const INDENT_WIDTH: usize = 2;

    fn get_indent(level: usize) -> usize {
        Self::INDENT_WIDTH * level
    }

    fn show(&self, source: &str, level: usize, buffer: &mut String);
}
