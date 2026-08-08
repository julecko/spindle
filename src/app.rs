use eframe::egui;
use egui::{Align, Layout};

use crate::state::{AppState, Tab};
use crate::{screens, widgets};

/// The root `eframe::App`. It owns the single `AppState` and, each frame,
/// draws the nav header, then dispatches to whichever screen is active.
/// This file ties `state`/`screens`/`widgets`/`data` together — everything
/// else only ever talks to its neighbors through here.
pub struct SpindleApp {
    state: AppState,
}

impl Default for SpindleApp {
    fn default() -> Self {
        Self {
            state: AppState::default(),
        }
    }
}

impl eframe::App for SpindleApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("header").show(ui, |ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                widgets::disc_icon::draw(ui);
                ui.add_space(12.0);

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if nav_link(ui, self.state.tab == Tab::Settings, "Nastavenia").clicked() {
                        self.state.tab = Tab::Settings;
                    }
                    if nav_link(ui, self.state.tab == Tab::Search, "Vyhľadávanie").clicked() {
                        self.state.tab = Tab::Search;
                    }
                    if nav_link(ui, self.state.tab == Tab::Mp3, "MP3 Knižnica").clicked() {
                        self.state.tab = Tab::Mp3;
                    }
                    if nav_link(ui, self.state.tab == Tab::Cd, "CD Knižnica").clicked() {
                        self.state.tab = Tab::Cd;
                    }
                });
            });
            ui.add_space(8.0);
        });

        // `self.state.tab` copies out (see the `Copy` note on `Tab`), so
        // each arm below is free to borrow `self.state` mutably.
        match self.state.tab {
            Tab::Cd => screens::cd_library::show(ui, &mut self.state),
            Tab::Mp3 => screens::mp3_library::show(ui, &mut self.state),
            Tab::Search => screens::search::show(ui, &mut self.state),
            Tab::Settings => screens::settings::show(ui, &mut self.state),
        }
    }
}

/// Header nav item: `.frame(false)` bypasses the theme's `weak_bg_fill`/
/// `bg_stroke` entirely, so it never paints a background or border here —
/// scoped to just these buttons rather than a global "no chrome" rule, so
/// other buttons elsewhere in the app can still look like normal buttons.
/// Text color still reacts to state (default vs. hovered vs. selected) via
/// each `Theme`'s `fg_stroke`, set once in `theme::apply()`.
fn nav_link(ui: &mut egui::Ui, selected: bool, text: &str) -> egui::Response {
    ui.add(egui::Button::selectable(selected, text).frame(false))
}
