use crate::data;
use crate::models::cd::Cd;

/// `Copy` matters here: `app.rs` matches on `self.state.tab` in the same
/// expression where it also needs to borrow `self.state` mutably for the
/// matched screen. If `Tab` weren't `Copy`, that match would hold a borrow
/// of `self.state` open across the arms and the mutable borrow inside them
/// wouldn't compile.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    Cd,
    Mp3,
    Search,
    Settings,
}

/// Everything that must survive between frames lives here. Screens and
/// widgets don't keep their own state structs — they borrow `AppState`
/// mutably and read/write it directly, which is how an immediate-mode UI
/// like egui stays in sync without a separate widget tree.
pub struct AppState {
    pub tab: Tab,
    pub cds: Vec<Cd>,
    /// Index into `theme::catalog()`. The picker in the header updates this
    /// and calls `theme::apply()` directly, so this field just remembers
    /// which swatch is selected for re-drawing the picker itself.
    pub theme_index: usize,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            tab: Tab::Cd,
            cds: data::store::load_cds(),
            theme_index: 0,
        }
    }
}
