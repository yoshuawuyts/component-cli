//! Search bar component.
//!
//! Renders a search form with an input, optional carousel placeholder,
//! keyboard shortcut badge, and submit button.

use html::text_content::Division;

/// Configuration for the search bar.
pub(crate) struct SearchBar {
    /// Current query value (empty for no pre-fill).
    pub query: String,
    /// HTML id for the input element (for focus-on-/ shortcut).
    pub input_id: &'static str,
    /// Whether to show the animated carousel placeholder.
    pub carousel: bool,
}

impl Default for SearchBar {
    fn default() -> Self {
        Self {
            query: String::new(),
            input_id: "search-input",
            carousel: false,
        }
    }
}

/// Render a compact search bar for nav / inline use.
///
/// 36px tall, border + surface background, `/` kbd badge.
#[allow(dead_code)]
pub(crate) fn compact(input_id: &str) -> Division {
    Division::builder()
        .form(|form| {
            form.action("/search")
                .method("get")
                .class("relative flex search-form")
                .input(|input| {
                    input
                        .type_("search")
                        .name("q")
                        .placeholder("Search\u{2026}")
                        .aria_label("Search")
                        .id(input_id.to_owned())
                        .class("w-full sm:w-48 h-9 px-3 pr-10 rounded-md border border-line bg-surface text-[14px] text-ink-900 placeholder:text-ink-400 focus:outline-none focus:border-ink-900")
                })
                .span(|kbd| {
                    kbd.class("search-kbd")
                        .aria_hidden(true)
                        .text("/".to_owned())
                })
        })
        .build()
}

/// Render the hero search bar with carousel placeholder and submit button.
pub(crate) fn hero(cfg: &SearchBar) -> Division {
    let mut wrapper = Division::builder();
    wrapper.form(|form| {
        form.action("/search")
            .method("get")
            .class("flex flex-1 max-w-lg search-form")
            .division(|inner| {
                inner
                    .class("flex-1 relative")
                    .input(|input| {
                        let mut i = input
                            .type_("search")
                            .name("q")
                            .id(cfg.input_id.to_owned())
                            .aria_label("Search components and interfaces")
                            .autofocus(true)
                            .class("w-full h-10 pl-10 pr-8 rounded-l-lg border border-line bg-canvas text-[14px] text-ink-900 placeholder:text-ink-400 focus:outline-none focus:border-ink-900");
                        if !cfg.query.is_empty() {
                            i = i.value(cfg.query.clone());
                        }
                        i
                    });
                if cfg.carousel {
                    inner
                        .span(|overlay| {
                            overlay
                                .id("search-carousel")
                                .class("search-carousel")
                                .aria_hidden(true)
                                .span(|prefix| prefix.text("Search ".to_owned()))
                                .span(|word| {
                                    word.id("carousel-word")
                                        .class("carousel-word")
                                        .text("components\u{2026}")
                                })
                        });
                }
                inner
            })
            .button(|btn| {
                btn.type_("submit")
                    .class("h-10 px-4 rounded-r-md border-[1.5px] border-l-0 border-ink-900 bg-surface text-ink-900 text-[13px] font-medium hover:bg-surfaceMuted")
                    .text("Search")
            })
    });
    wrapper.build()
}

/// Render a simple inline search form (for search results page refinement).
pub(crate) fn inline(query: &str) -> Division {
    Division::builder()
        .class("mb-8")
        .form(|form| {
            form.class("flex gap-2")
                .method("get")
                .action("/search")
                .input(|input| {
                    input
                        .type_("search")
                        .name("q")
                        .value(query.to_owned())
                        .placeholder("Search\u{2026}")
                        .class("flex-1 h-9 px-3 rounded-md border border-line bg-surface text-[14px] text-ink-900 placeholder:text-ink-400 focus:outline-none focus:border-ink-900")
                })
                .button(|btn| {
                    btn.type_("submit")
                        .class("h-9 px-4 rounded-md bg-ink-900 text-canvas text-[13px] font-medium hover:bg-ink-700 transition-colors")
                        .text("Search")
                })
        })
        .build()
}
