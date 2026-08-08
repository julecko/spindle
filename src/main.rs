// src/main.rs
use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Spindle",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

#[derive(Default)]
struct MyApp {
    count: i32,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.label(format!("Count: {}", self.count));

        if ui.button("Increment").clicked() {
            self.count += 1;
        }

        if ui.button("Reset").clicked() {
            self.count = 0;
        }
    }
}
