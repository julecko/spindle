use egui::Ui;

use crate::state::AppState;

/// Not built out yet — same `show(ui, state)` shape as `cd_library`, so
/// filling this in later (its own `models::mp3`, its own view) is a
/// drop-in, not a redesign.
pub fn show(ui: &mut Ui, _state: &mut AppState) {
    ui.heading("MP3 Knižnica content");
}
