use egui::Ui;

use crate::state::AppState;

pub fn show(ui: &mut Ui, _state: &mut AppState) {
    ui.heading("Vyhľadávanie content");
}
