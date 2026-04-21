//! Markdown-to-HTML rendering for documentation text.

/// Render a markdown string to HTML.
///
/// Uses `pulldown-cmark` to convert doc comments into HTML.
/// The output is safe to embed directly in the page since
/// `pulldown-cmark` does not pass through raw HTML by default.
#[must_use]
pub(crate) fn render(input: &str) -> String {
    use pulldown_cmark::{Options, Parser, html};

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(input, options);
    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
}

/// Standard CSS class for rendered doc comment blocks.
#[allow(dead_code)]
pub(crate) const DOC_CLASS: &str = "text-base text-ink-500 leading-relaxed prose-doc";

/// Render markdown and wrap in a styled `<div>`.
///
/// Applies prose-like styling classes for rendered documentation.
#[must_use]
pub(crate) fn render_block(input: &str, class: &str) -> String {
    let html = render(input);
    format!(r#"<div class="{class}">{html}</div>"#)
}

/// Render a short markdown string as inline HTML.
///
/// Strips the outer `<p>` wrapper that pulldown-cmark adds for single
/// paragraphs, making the result safe for use inside table cells,
/// list items, and other inline contexts.
#[must_use]
pub(crate) fn render_inline(input: &str) -> String {
    let mut html = render(input);
    // Strip a single wrapping <p>...</p>\n
    if html.starts_with("<p>") && html.trim_end().ends_with("</p>") {
        let trimmed = html.trim_end();
        if trimmed.matches("<p>").count() == 1 {
            html = trimmed
                .strip_prefix("<p>")
                .and_then(|s| s.strip_suffix("</p>"))
                .unwrap_or(trimmed)
                .to_owned();
        }
    }
    html
}
