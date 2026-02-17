use crate::ast::Span;

pub fn show_source_context(source: &str, span: Span) -> String {
    let front = &source[..span.start()];
    let context_start = front.rfind(|ch| ch == '\n').map(|i| i + 1).unwrap_or(0);
    let back = &source[span.end()..];
    let context_end = span.end() + back.find(|ch| ch == '\n').unwrap_or(back.len());
    let indent = span.start() - context_start;
    let underline = "~".repeat(span.length());
    format!(
        "{}\n{:indent$}{underline}",
        &source[context_start..context_end],
        ""
    )
}
