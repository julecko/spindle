use egui::Ui;

use crate::state::AppState;
use crate::widgets::card_view;

/// One screen = one file, one `show(ui, state)` function. This is the one
/// tab with real data wired through it end to end: `data` seeds
/// `state.cds`, this screen reads it, `widgets::card_view` renders it.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    ui.heading("CD Knižnica");
    ui.separator();
    card_view::show(ui, &state.cds);
}
