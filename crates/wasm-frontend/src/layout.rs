//! Base HTML document layout.
//!
//! Provides the shared page shell — `<html>`, `<head>`, and `<body>` wrapper —
//! used by all pages.

// r[impl frontend.rendering.html-crate]
// r[impl frontend.styling.tailwind]
// r[impl frontend.styling.light-theme]
// r[impl frontend.styling.accent-color]
// r[impl frontend.styling.responsive]

use crate::footer;
use crate::nav;

/// Accent color used throughout the UI.
///
/// RGB: R81 G47 B235 → `#512FEB`.
pub(crate) const ACCENT_COLOR: &str = "#512FEB";

/// Render a complete HTML document with the given title and body content.
///
/// Includes the shared navigation bar, Tailwind CSS via CDN, custom accent
/// color CSS variables, and footer.
#[must_use]
pub(crate) fn document(title: &str, body_content: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{title} — wasm registry</title>
  <script src="https://cdn.tailwindcss.com"></script>
  <script>
    tailwind.config = {{
      theme: {{
        extend: {{
          colors: {{
            accent: '{ACCENT_COLOR}',
          }},
          fontFamily: {{
            mono: ['ui-monospace', 'Cascadia Code', 'Source Code Pro', 'Menlo', 'Consolas', 'DejaVu Sans Mono', 'monospace'],
          }},
        }}
      }}
    }}
  </script>
  <style>
    :root {{
      --accent: {ACCENT_COLOR};
    }}
  </style>
</head>
<body class="bg-white text-gray-900 min-h-screen flex flex-col font-mono leading-relaxed">
  {nav}
  <main class="flex-1 w-full max-w-5xl mx-auto px-4 sm:px-6 py-10">
    {body_content}
  </main>
  {footer}
</body>
</html>"#,
        title = title,
        nav = nav::render(),
        footer = footer::render(),
        body_content = body_content,
    )
}
