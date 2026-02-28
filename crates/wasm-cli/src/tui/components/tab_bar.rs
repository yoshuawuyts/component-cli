use ratatui::{
    prelude::*,
    widgets::{Block, Tabs},
};

/// A trait for items that can be displayed in a tab bar.
pub trait TabItem: Copy + PartialEq + 'static {
    /// Returns all tab items in order.
    fn all() -> &'static [Self];

    /// Returns the display title for this tab.
    fn title(&self) -> &'static str;

    /// Returns the next tab (wrapping around).
    #[allow(clippy::indexing_slicing)]
    fn next(&self) -> Self {
        let all = Self::all();
        let current_idx = all.iter().position(|t| t == self).unwrap_or(0);
        let next_idx = (current_idx + 1) % all.len();
        all[next_idx]
    }

    /// Returns the previous tab (wrapping around).
    #[allow(clippy::indexing_slicing)]
    fn prev(&self) -> Self {
        let all = Self::all();
        let current_idx = all.iter().position(|t| t == self).unwrap_or(0);
        let prev_idx = if current_idx == 0 {
            all.len() - 1
        } else {
            current_idx - 1
        };
        all[prev_idx]
    }
}

/// A reusable tab bar component.
#[derive(Debug)]
pub struct TabBar<'a, T: TabItem> {
    title: String,
    selected: T,
    highlight_style: Style,
    _marker: std::marker::PhantomData<&'a T>,
}

impl<'a, T: TabItem> TabBar<'a, T> {
    /// Creates a new tab bar with the given title and selected tab.
    pub fn new(title: impl Into<String>, selected: T) -> Self {
        Self {
            title: title.into(),
            selected,
            highlight_style: Style::default().bold().fg(Color::Yellow),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: TabItem> Widget for TabBar<'_, T> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let all_tabs = T::all();
        let tab_titles: Vec<&str> = all_tabs.iter().map(|t| t.title()).collect();
        let selected_index = all_tabs
            .iter()
            .position(|t| *t == self.selected)
            .unwrap_or(0);

        let tabs = Tabs::new(tab_titles)
            .block(Block::bordered().title(format!(" {} ", self.title)))
            .highlight_style(self.highlight_style)
            .select(selected_index);

        tabs.render(area, buf);
    }
}
