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
    let html = render(input);
    // Extract content from the first <p>...</p> only, stripping any
    // subsequent paragraphs that would break inline contexts.
    if let Some(start) = html.find("<p>") {
        let content_start = start + 3;
        if let Some(end) = html[content_start..].find("</p>") {
            return html[content_start..content_start + end].to_owned();
        }
    }
    // Fallback: return as-is if no <p> found
    html
}
