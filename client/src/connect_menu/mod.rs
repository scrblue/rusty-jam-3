use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::MainState;

#[derive(Resource)]
pub struct ConnectMenuState {
    pub addr: String,
    pub user: String,
    pub pass: String,
}

impl Default for ConnectMenuState {
    fn default() -> Self {
        Self {
            addr: "http://127.0.0.1:2000".to_owned(),
            user: String::new(),
            pass: String::new(),
        }
    }
}

pub fn connect_menu(
    mut app_state: ResMut<NextState<MainState>>,
    mut menu_state: ResMut<ConnectMenuState>,
    mut contexts: EguiContexts,
) {
    egui::Window::new("Connect to Server").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Server address: ");
            ui.text_edit_singleline(&mut menu_state.addr)
        });
        ui.horizontal(|ui| {
            ui.label("Display name: ");
            ui.text_edit_singleline(&mut menu_state.user)
        });
        ui.horizontal(|ui| {
            ui.label("Server password: ");
            ui.text_edit_singleline(&mut menu_state.pass)
        });

        if ui.button("Connect").clicked() {
            app_state.set(MainState::InGame);
        }
    });
}
