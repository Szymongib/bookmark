

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AppEvent {
    ShowInfoPopup(String),
    CommandMode,
    SearchMode,
    ConfirmDelete,
    ShowEditPopup,
    ShowHelpPopup,
}


