use egui::{CentralPanel, Color32, Context, Frame, Margin, Panel};

use super::MenuTheme;
use crate::game::{
    client::net::server_entry::ServerEntry, server::net::config::GameServerConfig, ui::{command::UiCommand, screens::{Screen, ScreenId}}, 
};

pub mod server_browser;
pub mod direct_connect;
pub mod host_server;

pub struct OnlineMenu {
    pub title: String,    
    pub theme: MenuTheme,
    pub online_tabs: Vec<String>,
    pub active_online_tab: ScreenId,
}

impl Screen for OnlineMenu {
    fn ui(&mut self, ui: &mut egui::Ui) -> Vec<UiCommand> {
        let command = self.menu_selector(ui);
        match command {
            UiCommand::Push(screen_id) => self.active_online_tab = screen_id,
            _ => {},
        }

        return vec![command];
    }
}

impl Default for OnlineMenu {
    fn default() -> Self {
        return Self {
            title: "DEDICATED SERVER".into(),
            theme: MenuTheme::dark(),
            online_tabs: vec![
                "SERVER BROWSER".into(),
                "DIRECT CONNECT".into(),
                "HOST SERVER".into(),
            ],
            active_online_tab: ScreenId::SERVER_BROWSER,
        };
    }
}

impl OnlineMenu {
    pub fn new() -> Self {
        return Self {
            ..Default::default()
        };
    }

    fn menu_selector(&mut self, ui: &mut egui::Ui) -> UiCommand {
        let mut tab = UiCommand::None;

        Panel::top("top_bar").resizable(false).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("My Game");
                ui.separator();

                if ui.selectable_label(
                        self.active_online_tab == ScreenId::SERVER_BROWSER, 
                        "Server Browser"
                    ).clicked()
                {
                    tab = UiCommand::Push(ScreenId::SERVER_BROWSER);
                }

                if ui.selectable_label(
                        self.active_online_tab == ScreenId::DIRECT_CONNECT, 
                        "Direct Connect"
                    ).clicked()
                {
                    tab = UiCommand::Push(ScreenId::DIRECT_CONNECT);
                }

                if ui.selectable_label(
                        self.active_online_tab == ScreenId::HOST_SERVER, 
                        "Host Server"
                    ).clicked()
                {
                    tab = UiCommand::Push(ScreenId::HOST_SERVER);
                }
            });
        });

        return tab;
    }
}
