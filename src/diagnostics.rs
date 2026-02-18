use crate::ast::Span;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFile;
use codespan_reporting::term::emit_to_string;

pub fn show_source_context(source: &str, span: Span) -> String {
    let file = SimpleFile::new("example", source);
    let diagnostic = Diagnostic::error()
        .with_message("example")
        .with_label(Label::primary((), span.range()).with_message("here"));
    let mut writer = String::new();
    let config = codespan_reporting::term::Config::default();
    emit_to_string(&mut writer, &config, &file, &diagnostic).unwrap();
    writer
}
