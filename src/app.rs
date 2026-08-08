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
                    if ui
                        .selectable_label(self.state.tab == Tab::Settings, "Nastavenia")
                        .clicked()
                    {
                        self.state.tab = Tab::Settings;
                    }
                    if ui
                        .selectable_label(self.state.tab == Tab::Search, "Vyhľadávanie")
                        .clicked()
                    {
                        self.state.tab = Tab::Search;
                    }
                    if ui
                        .selectable_label(self.state.tab == Tab::Mp3, "MP3 Knižnica")
                        .clicked()
                    {
                        self.state.tab = Tab::Mp3;
                    }
                    if ui
                        .selectable_label(self.state.tab == Tab::Cd, "CD Knižnica")
                        .clicked()
                    {
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
