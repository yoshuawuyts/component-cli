//! 07 — Navigation.

use html::text_content::{Division, UnorderedList};

/// Nav items for each group: (label, is_active).
pub(crate) const GROUP_1: &[(&str, bool)] = &[
    ("Tellus", true),
    ("Pellentesque Habitant", false),
    ("Vestibulum Ante", false),
    ("Convallis Dolor", false),
];

pub(crate) const GROUP_2: &[(&str, bool)] = &[
    ("Faucibus", false),
    ("Suspendisse", false),
    ("Aliquam Erat", false),
];

#[allow(dead_code)]
/// Build a nav list from items.
pub(crate) fn nav_list(items: &[(&str, bool)]) -> UnorderedList {
    let mut ul = UnorderedList::builder();
    ul.class("space-y-px text-[14px]");
    for (label, active) in items {
        let class = if *active {
            "flex items-center px-3 h-9 rounded-md bg-surfaceMuted text-ink-900 font-medium"
        } else {
            "flex items-center px-3 h-9 rounded-md hover:bg-surfaceMuted text-ink-700"
        };
        let label = (*label).to_owned();
        let class = class.to_owned();
        let li = html::text_content::ListItem::builder()
            .anchor(|a| a.href("#".to_owned()).class(class).text(label))
            .build();
        ul.push(li);
    }
    ul.build()
}

/// Render the navigation section.
pub(crate) fn render(
    section_id: &str,
    num: &str,
    title: &str,
    desc: &str,
    group_1: &[(&str, bool)],
    group_2: &[(&str, bool)],
) -> String {
    let content = Division::builder()
        .class("max-w-[260px]")
        .push(nav_list(group_1))
        .division(|rule| rule.class("my-4 border-t-[1.5px] border-rule"))
        .push(nav_list(group_2))
        .build()
        .to_string();

    super::section(section_id, num, title, desc, &content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot() {
        insta::assert_snapshot!(crate::components::ds::pretty_html(&render(
            "nav",
            "07",
            "Navigation",
            "Sidebar list. Active item uses a muted surface fill with full ink weight. Groups separated by a soft rule.",
            GROUP_1,
            GROUP_2,
        )));
    }
}
