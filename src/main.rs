// Each `mod` here is one directory (or file) under src/. This is the only
// place the module tree is declared — every other file just does
// `use crate::...` for the parts it needs.
mod app;
mod data;
mod models;
mod screens;
mod state;
mod theme;
mod widgets;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Spindle",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            theme::apply(&cc.egui_ctx, &theme::catalog()[2]);
            Ok(Box::new(app::SpindleApp::default()))
        }),
    )
}
