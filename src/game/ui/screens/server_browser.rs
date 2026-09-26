use egui::{CentralPanel, Color32, Panel};

use crate::game::client::net::server_entry::ServerEntry;




pub struct ServerBrowser {
    pub servers: Vec<ServerEntry>,
    pub active_server: u32,
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

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // CentralPanel::default().show(ui, |ui| {
        //
        // });
        CentralPanel::default().show(ui, |ui| {
            ui.columns(2, |columns| {
                columns[0].heading("Entries");
                columns[0].separator();

                for entry in &self.servers {
                    if columns[0].button(entry.name.clone()).clicked() {
                        // Select entry later
                    }
                }

                columns[1].heading("Details");
                columns[1].label("Nothing selected");
            });
        });


    }
}
