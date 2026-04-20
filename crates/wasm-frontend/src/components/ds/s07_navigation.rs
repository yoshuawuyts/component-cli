//! 07 — Navigation.

use html::text_content::Division;

/// Nav items for each group: (label, is_active).
const GROUP_1: &[(&str, bool)] = &[
    ("Tellus", true),
    ("Pellentesque Habitant", false),
    ("Vestibulum Ante", false),
    ("Convallis Dolor", false),
];

const GROUP_2: &[(&str, bool)] = &[
    ("Faucibus", false),
    ("Suspendisse", false),
    ("Aliquam Erat", false),
];

/// Render a nav link list item.
fn nav_item(label: &str, active: bool) -> String {
    let class = if active {
        "flex items-center px-3 h-9 rounded-md bg-surfaceMuted text-ink-900 font-medium"
    } else {
        "flex items-center px-3 h-9 rounded-md hover:bg-surfaceMuted text-ink-700"
    };
    format!(r##"<li><a href="#" class="{class}">{label}</a></li>"##)
}

/// Render the navigation section.
pub(crate) fn render() -> String {
    let mut ul1 = String::from(r#"<ul class="space-y-px text-[14px]">"#);
    for (label, active) in GROUP_1 {
        ul1.push_str(&nav_item(label, *active));
    }
    ul1.push_str("</ul>");

    let mut ul2 = String::from(r#"<ul class="space-y-px text-[14px]">"#);
    for (label, active) in GROUP_2 {
        ul2.push_str(&nav_item(label, *active));
    }
    ul2.push_str("</ul>");

    let content = Division::builder()
        .class("max-w-[260px]")
        .text(ul1)
        .division(|rule| rule.class("my-4 border-t-[1.5px] border-rule"))
        .text(ul2)
        .build()
        .to_string();

    super::section(
        "nav",
        "07",
        "Navigation",
        "Sidebar list. Active item uses a muted surface fill with full ink weight. Groups separated by a soft rule.",
        &content,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot() {
        insta::assert_snapshot!(render());
    }
}
