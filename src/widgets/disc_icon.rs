use egui::{Color32, Sense, Stroke, vec2};

/// The app's logo — no brand wordmark, just a painted disc glyph. Lives in
/// `widgets/` (not inline in `app.rs`) because it's a self-contained piece
/// of UI with no dependency on `AppState`.
pub fn draw(ui: &mut egui::Ui) {
    let (rect, _response) = ui.allocate_exact_size(vec2(24.0, 24.0), Sense::hover());
    let painter = ui.painter();
    let center = rect.center();
    let orange = Color32::from_rgb(230, 100, 40);

    painter.circle_stroke(center, 10.0, Stroke::new(1.5, orange));
    painter.circle_filled(center, 3.0, orange);
}
