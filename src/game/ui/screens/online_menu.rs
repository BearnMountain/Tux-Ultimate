use egui::{CentralPanel, Color32, Context, Frame, Margin, Panel};

use super::MenuTheme;
use crate::game::{client::net::server_entry::ServerEntry, ui::screens::server_browser::ServerBrowser};



#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OnlineMenuAction {
    SERVER_BROWSER(u32),
    DIRECT_CONNECT(u32),
    HOST_SERVER(u32),
    NONE,
}

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

    pub server_browser: ServerBrowser,
}

impl Default for OnlineMenu {
    fn default() -> Self {
        return Self {
            title: "DEDICATED SERVER".into(),
            theme: MenuTheme::default(),
            online_tabs: vec![
                "SERVER BROWSER".into(),
                "DIRECT CONNECT".into(),
                "HOST SERVER".into(),
            ],
            active_online_tab: OnlineTab::SERVER_BROWSER,
            server_browser: ServerBrowser::new(),
        };
    }
}

impl OnlineMenu {
    pub fn new() -> Self {
        return Self {
            ..Default::default()
        };
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> OnlineMenuAction {
        let mut action = OnlineMenuAction::NONE;

        self.menu_selector(ui);
        match self.active_online_tab {
            OnlineTab::SERVER_BROWSER => {
                self.server_browser.ui(ui);
                action = OnlineMenuAction::SERVER_BROWSER(0);
            },
            OnlineTab::DIRECT_CONNECT => {},
            OnlineTab::HOST_SERVER => {},
        }

        return OnlineMenuAction::NONE;
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
