use egui::{Color32, vec2};

/// The app's logo — no brand wordmark, just a painted disc glyph. Lives in
/// `widgets/` (not inline in `app.rs`) because it's a self-contained piece
/// of UI with no dependency on `AppState`.
pub fn draw(ui: &mut egui::Ui) {
    let orange = Color32::from_rgb(230, 100, 40);

    ui.add(
        egui::Image::new(egui::include_image!("../../assets/app_icon.svg"))
            .fit_to_exact_size(vec2(24.0, 24.0))
            .tint(orange),
    );
}
