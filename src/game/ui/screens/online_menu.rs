use egui::{CentralPanel, Color32, Context, Frame, Margin, Panel};

use super::MenuTheme;
use crate::game::client::net::server_entry::ServerEntry;


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
    pub servers: Vec<ServerEntry>,
    pub active_server: u32,
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
            servers: vec![
                ServerEntry {
                    thumb_color: Color32::from_rgb(70, 70, 60),
                    name: "20 PLAYERS VS BOTS *** WIN EZ XP]".into(),
                    map: "NOSHAHR CANALS".into(),
                    mode: "CONQUEST".into(),
                    players: 18,
                    max_players: 20,
                    ping: 4,
                    host: "—".into(),
                    subtitle: String::new(),
                    tags: vec![],
                    map_preview_color: Color32::from_rgb(90, 90, 80),
                    next_map: String::new(),
                },
                ServerEntry {
                    thumb_color: Color32::from_rgb(150, 120, 90),
                    name: "HARDCORE TDM NOSHAR 3000 TICKETS".into(),
                    map: "ARICA HARBOR".into(),
                    mode: "CUSTOM".into(),
                    players: 30,
                    max_players: 32,
                    ping: 4,
                    host: "TP_McLouvin".into(),
                    subtitle: "hardcore tdm 3000 tickets".into(),
                    tags: vec![
                        ("2042", "CUSTOM LOGIC"),
                        ("NO VEHICLES", "HEALTH/DMG MODIFIERS"),
                        ("VEHICLE MODIFIERS", "LIMITED HUD"),
                        ("PvP AI", "SYM. PLAYER COUNT"),
                    ],
                    map_preview_color: Color32::from_rgb(160, 130, 100),
                    next_map: "NOSHAHR CANALS".into(),
                },
                ServerEntry {
                    thumb_color: Color32::from_rgb(60, 90, 70),
                    name: "2042 TDM 32 PLAYERS".into(),
                    map: "KALEIDOSCOPE".into(),
                    mode: "CUSTOM".into(),
                    players: 25,
                    max_players: 32,
                    ping: 4,
                    host: "—".into(),
                    subtitle: String::new(),
                    tags: vec![],
                    map_preview_color: Color32::from_rgb(70, 100, 80),
                    next_map: String::new(),
                },
                ServerEntry {
                    thumb_color: Color32::from_rgb(80, 80, 90),
                    name: "PEOPLE VS BOTS".into(),
                    map: "BATTLE OF THE BULGE".into(),
                    mode: "CONQUEST".into(),
                    players: 24,
                    max_players: 32,
                    ping: 4,
                    host: "—".into(),
                    subtitle: String::new(),
                    tags: vec![],
                    map_preview_color: Color32::from_rgb(90, 90, 100),
                    next_map: String::new(),
                },
                ServerEntry {
                    thumb_color: Color32::from_rgb(140, 70, 50),
                    name: "BCL CQ HARDCORE 128".into(),
                    map: "HOURGLASS".into(),
                    mode: "CONQUEST".into(),
                    players: 118,
                    max_players: 128,
                    ping: 4,
                    host: "—".into(),
                    subtitle: String::new(),
                    tags: vec![],
                    map_preview_color: Color32::from_rgb(150, 80, 60),
                    next_map: String::new(),
                },
            ],
            active_server: 0,
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

        Panel::top("top_bar")
            .resizable(false)
            .show(ui, |ui| {
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
                            "Settings"
                        ).clicked()
                    {
                        self.active_online_tab = OnlineTab::HOST_SERVER;
                    }
                });

                CentralPanel::default().show(ui, |ui| {
                    match self.active_online_tab {
                        OnlineTab::SERVER_BROWSER => {
                            ui.heading("Servers");
                            ui.label("Select a server from the list.");
                        }

                        OnlineTab::DIRECT_CONNECT => {
                            ui.heading("Players");
                            ui.label("Player information goes here.");
                        }

                        OnlineTab::HOST_SERVER => {
                            ui.heading("Settings");
                            ui.label("Settings go here.");
                        }
                    }
                });
            });

        return OnlineMenuAction::NONE;
    }
}
