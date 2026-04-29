use super::components::TabItem;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Tab {
    Local,
    Components,
    Interfaces,
    Search,
    Settings,
    Log,
}

impl Tab {
    const ALL: [Tab; 6] = [
        Tab::Local,
        Tab::Components,
        Tab::Interfaces,
        Tab::Search,
        Tab::Settings,
        Tab::Log,
    ];
}

impl TabItem for Tab {
    fn all() -> &'static [Self] {
        &Self::ALL
    }

    fn title(&self) -> &'static str {
        match self {
            Tab::Local => "Local [1]",
            Tab::Components => "Components [2]",
            Tab::Interfaces => "Interfaces [3]",
            Tab::Search => "Search [4]",
            Tab::Settings => "Settings [5]",
            Tab::Log => "Log [6]",
        }
    }
}

/// The current input mode of the application
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) enum InputMode {
    /// Normal navigation mode
    #[default]
    Normal,
    /// Viewing a package detail (with the package index)
    PackageDetail(usize),
    /// Viewing type detail
    TypeDetail,
    /// Pull prompt is active
    PullPrompt(PullPromptState),
    /// Search input is active
    SearchInput,
    /// Filter input is active (for packages tab)
    FilterInput,
}

/// State of the pull prompt
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct PullPromptState {
    pub input: String,
    pub error: Option<String>,
    pub in_progress: bool,
}

/// Manager readiness state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum ManagerState {
    #[default]
    Loading,
    Ready,
}
