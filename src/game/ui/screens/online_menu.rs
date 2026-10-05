use egui::{CentralPanel, Color32, Context, Frame, Margin, Panel};

use super::MenuTheme;
use crate::game::{
    client::net::server_entry::ServerEntry, 
    server::net::config::GameServerConfig, 
    ui::{
        screens::{
            host_server::HostServer, 
            lobby_menu::LobbyMenu, 
            server::ServerUi, 
            server_browser::ServerBrowser
        }
    }
};

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
pub enum OnlineMenuAction {
    SERVER_BROWSER(u32),
    DIRECT_CONNECT(u32),
    HOST_SERVER(GameServerConfig),
    NONE,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum OnlineTab {
    SERVER_BROWSER,
    DIRECT_CONNECT,
    HOST_SERVER,
}

#[allow(non_camel_case_types)]
enum OnlineState {
    MENU,
    CONNECTING,
    LOBBY, 
    LOADING,
    IN_GAME,
    NONE,
}

pub struct OnlineMenu {
    pub title: String,    
    pub theme: MenuTheme,
    pub online_tabs: Vec<String>,
    pub active_online_tab: OnlineTab,

    pub online_state: OnlineState,

    pub server_browser: ServerBrowser,
    pub host_server: HostServer,

    pub server_ui: ServerUi,
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
            online_state: OnlineState::MENU,
            server_browser: ServerBrowser::new(),
            host_server: HostServer::new(),
            server_ui: ServerUi::new(),
        };
    }
}

impl OnlineMenu {
    pub fn new() -> Self {
        return Self {
            ..Default::default()
        };
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> OnlineState {
        let mut action = OnlineState::NONE;

        match self.online_state {
            OnlineState::MENU => {
                if self.menu_state(ui) != OnlineMenuAction::NONE {
                    action = OnlineState::CONNECTING;
                }
            },
            OnlineState::CONNECTING => {

            },
            OnlineState::LOBBY => todo!(),
            OnlineState::LOADING => todo!(),
            OnlineState::IN_GAME => todo!(),
            OnlineState::NONE => todo!(),
        }

        return action;
    }

    fn menu_state(&mut self, ui: &mut egui::Ui) -> OnlineMenuAction {
        let mut action = OnlineMenuAction::NONE;

        self.menu_selector(ui);
        match self.active_online_tab {
            OnlineTab::SERVER_BROWSER => {
                self.server_browser.ui(ui);
                // action = OnlineMenuAction::SERVER_BROWSER(0);
            },
            OnlineTab::DIRECT_CONNECT => {},
            OnlineTab::HOST_SERVER => {
                // declared to host server
                if self.host_server.ui(ui) {
                    action = OnlineMenuAction::HOST_SERVER(
                        self.host_server.server_config.clone()
                    );
                    // self.host_server.set
                }
            },
        }

        return action;
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
