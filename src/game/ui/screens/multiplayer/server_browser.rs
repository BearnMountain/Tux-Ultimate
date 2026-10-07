use egui::{CentralPanel, Color32, Panel, accesskit::Role::Table};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};

use crate::game::{client::net::server_entry::ServerEntry, ui::{command::UiCommand, screens::Screen}};

pub struct ServerBrowser {
    pub servers: Vec<ServerEntry>,
    pub active_server: usize,
}

impl Screen for ServerBrowser {
    fn ui(&mut self, ui: &mut egui::Ui) -> Vec<UiCommand> {
        StripBuilder::new(ui)
            .size(Size::relative(0.7)) // browser gets 70% 
            .size(Size::remainder())
            .horizontal(|mut strip| {
                strip.cell(|ui| { self.browser_panel(ui); });
                strip.cell(|ui| { self.info_panel(ui); });
            });
       
        return vec![UiCommand::None];
    }
}

impl ServerBrowser {
    pub fn new() -> Self {

        return Self {
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

    fn browser_panel(&mut self, ui: &mut egui::Ui) {
        TableBuilder::new(ui)
            .striped(true)
            .resizable(false)
            .sense(egui::Sense::click())
            .column(Column::remainder())
            .column(Column::auto())
            .column(Column::auto())
            .header(25.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Server Name");
                });
                header.col(|ui| {
                    ui.strong("Players");
                });
                header.col(|ui| {
                    ui.strong("Ping");
                });
            })
            .body(|mut body| {
                for (i, server) in self.servers.iter().enumerate() {
                    body.row(30.0, |mut row| {
                        // Make the whole row clickable.
                        row.set_selected(self.active_server == i);

                        row.col(|ui| {
                            if ui.label(&server.name).clicked() {
                                self.active_server = i;
                            }
                        });

                        row.col(|ui| {
                            if ui.label(
                                format!("{}/{}", 
                                    server.players, 
                                    server.max_players)
                                ).clicked()
                            {
                                self.active_server = i;
                            }
                        });

                        row.col(|ui| {
                            if ui.label(format!("{} ms", server.ping)).clicked() {
                                self.active_server = i;
                            }
                        });

                        if row.response().clicked() {
                            self.active_server = i;
                        }
                    });
                }
            });
    }

    fn info_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Details");

        let server_info = &self.servers[self.active_server];
        ui.label(format!("{}", server_info.map));


    // pub thumb_color: Color32, // placeholder swap-in for a TextureHandle
    // pub name: String,
    // pub map: String,
    // pub mode: String,
    // pub players: u32,
    // pub max_players: u32,
    // pub ping: u8, // 1..=4 bars
    // pub host: String,
    // pub subtitle: String,
    // pub tags: Vec<(&'static str, &'static str)>, // (left col, right col)
    // pub map_preview_color: Color32,
    // pub next_map: String,


    }
}
