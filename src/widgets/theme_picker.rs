use egui::{Sense, Stroke, Ui, vec2};

use crate::theme;

/// Small clickable color swatches, one per theme in `theme::catalog()`.
/// Lives in `widgets/` (not `app.rs`) since it only needs the index it's
/// given — nothing here depends on the rest of `AppState`.
pub fn show(ui: &mut Ui, theme_index: &mut usize) {
    ui.horizontal(|ui| {
        for (i, t) in theme::catalog().iter().enumerate() {
            let (rect, response) = ui.allocate_exact_size(vec2(18.0, 18.0), Sense::click());
            ui.painter().circle_filled(rect.center(), 8.0, t.accent);
            if *theme_index == i {
                ui.painter()
                    .circle_stroke(rect.center(), 9.0, Stroke::new(2.0, t.text));
            }

            let response = response.on_hover_text(t.name);
            if response.clicked() {
                *theme_index = i;
                theme::apply(ui.ctx(), t);
            }
        }
    });
}
