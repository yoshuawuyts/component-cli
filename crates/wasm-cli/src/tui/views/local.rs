use ratatui::{
    prelude::*,
    widgets::{Block, List, ListItem, Paragraph, Widget},
};
use wasm_detector::WasmEntry;

/// View for the Local tab
#[derive(Debug)]
pub struct LocalView<'a> {
    wasm_files: &'a [WasmEntry],
}

impl<'a> LocalView<'a> {
    /// Create a new LocalView with the given WASM files
    #[must_use]
    pub fn new(wasm_files: &'a [WasmEntry]) -> Self {
        Self { wasm_files }
    }
}

impl Widget for LocalView<'_> {
    #[allow(clippy::indexing_slicing)]
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.wasm_files.is_empty() {
            let message = Paragraph::new("No local WASM files detected.\n\nPress 'r' to refresh.")
                .centered()
                .style(Style::default().fg(Color::DarkGray));
            message.render(area, buf);
        } else {
            let layout = Layout::vertical([
                Constraint::Length(2),
                Constraint::Min(0),
                Constraint::Length(2),
            ])
            .split(area);

            // Title
            let title = Paragraph::new(format!(
                "Detected {} local WASM file(s)",
                self.wasm_files.len()
            ))
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Center);
            title.render(layout[0], buf);

            // List of files
            let items: Vec<ListItem> = self
                .wasm_files
                .iter()
                .enumerate()
                .map(|(idx, entry)| {
                    let path = entry.path().display().to_string();
                    let content = format!("{}. {}", idx + 1, path);
                    ListItem::new(content).style(Style::default().fg(Color::White))
                })
                .collect();

            let list = List::new(items)
                .block(Block::bordered().title("Files"))
                .style(Style::default());

            Widget::render(list, layout[1], buf);

            // Help text
            let help = Paragraph::new("Press 'r' to refresh • 'q' to quit")
                .style(Style::default().fg(Color::DarkGray))
                .alignment(Alignment::Center);
            help.render(layout[2], buf);
        }
    }
}
