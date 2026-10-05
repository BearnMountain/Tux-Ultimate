use egui::{CentralPanel, Color32, Context, Frame, Margin, Panel};

use super::MenuTheme;
use crate::game::{
    client::net::server_entry::ServerEntry, server::net::config::GameServerConfig, ui::{command::UiCommand, screens::{Screen, ScreenId}}, 
};

pub mod server_browser;
pub mod direct_connect;
pub mod host_server;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum OnlineTab {
    SERVER_BROWSER,
    DIRECT_CONNECT,
    HOST_SERVER,
}

pub struct OnlineMenu {
    pub title: String,    
    pub theme: MenuTheme,
    pub online_tabs: Vec<String>,
    pub active_online_tab: OnlineTab,
}

impl Screen for OnlineMenu {
    fn ui(&mut self, ui: &mut egui::Ui) -> Vec<UiCommand> {

        self.menu_selector(ui);
        let command = match self.active_online_tab {
            OnlineTab::SERVER_BROWSER => UiCommand::Push(ScreenId::SERVER_BROWSER),
            OnlineTab::DIRECT_CONNECT => UiCommand::Push(ScreenId::DIRECT_CONNECT),
            OnlineTab::HOST_SERVER => UiCommand::Push(ScreenId::HOST_LOBBY),
        };

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
            active_online_tab: OnlineTab::SERVER_BROWSER,
        };
    }
}

impl OnlineMenu {
    pub fn new() -> Self {
        return Self {
            ..Default::default()
        };
    }

    fn menu_selector(&mut self, ui: &mut egui::Ui) {
        Panel::top("top_bar").resizable(false).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("My Game");
                ui.separator();

                if ui.selectable_label(
                        self.active_online_tab == OnlineTab::SERVER_BROWSER, 
                        "Server Browser"
                    ).clicked()
                {
                    self.active_online_tab = OnlineTab::SERVER_BROWSER;
                }

                if ui.selectable_label(
                        self.active_online_tab == OnlineTab::DIRECT_CONNECT, 
                        "Direct Connect"
                    ).clicked()
                {
                    self.active_online_tab = OnlineTab::DIRECT_CONNECT;
                }

                if ui.selectable_label(
                        self.active_online_tab == OnlineTab::HOST_SERVER, 
                        "Host Server"
                    ).clicked()
                {
                    self.active_online_tab = OnlineTab::HOST_SERVER;
                }
            });
        });
    }
}
