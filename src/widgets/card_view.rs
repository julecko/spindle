use egui::Ui;

use crate::models::cd::Cd;

/// A reusable piece of UI, called from `screens/`. It only knows how to
/// draw a list of `Cd`s — it doesn't know or care which screen called it.
/// Keeping it here instead of inline in `cd_library.rs` means the layout
/// has one place to change, and the MP3 library screen can reuse it later.
pub fn show(ui: &mut Ui, cds: &[Cd]) {
    for cd in cds {
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.strong(&cd.nazov_albumu);
            ui.label(&cd.skupina);
        });
    }
}
