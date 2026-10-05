use egui::{CentralPanel, Color32, Panel, accesskit::Role::Table};
use egui_extras::{Column, TableBuilder};

use crate::game::{client::net::server_entry::ServerEntry, ui::{command::UiCommand, screens::Screen}};

pub struct ServerBrowser {
    pub servers: Vec<ServerEntry>,
    pub active_server: usize,
}

impl Screen for ServerBrowser {
    fn ui(&mut self, ui: &mut egui::Ui) -> Vec<UiCommand> {


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

    // pub fn ui(&mut self, ui: &mut egui::Ui) {
        // let size = ui.available_size();
        // let table_width = size.x * 0.55;
        // let detail_width = size.x - table_width;
        //
        // let table_name = table_width * 0.35;
        // let table_player_count = table_width * 0.35;
        //
        // ui.horizontal(|ui|{
        //     ui.allocate_ui_with_layout(
        //         egui::vec2(table_width, size.y),
        //         egui::Layout::top_down(egui::Align::LEFT),
        //         |ui| {
        //             TableBuilder::new(ui)
        //                 .striped(true)
        //                 .resizable(false)
        //                 .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        //                 .column(Column::initial(table_name).resizable(true))
        //                 .column(Column::initial(table_player_count).resizable(true))
        //                 .column(Column::remainder())
        //                 .header(30.0, |mut header| {
        //                     header.col(|ui| {
        //                         ui.strong("Server Name");
        //                     });
        //
        //                     header.col(|ui| {
        //                         ui.strong("Players");
        //                     });
        //
        //                     header.col(|ui| {
        //                         ui.strong("Ping");
        //                     });
        //                 })
        //                 .body(|mut body| {
        //                     for (index, server) in self.servers.iter().enumerate() {
        //                         let selected = self.active_server == index;
        //
        //                         body.row(32.0, |mut row| {
        //                             row.col(|ui| {
        //                                 ui.label(&server.name);
        //                             });
        //
        //                             row.col(|ui| {
        //                                 ui.label(format!(
        //                                     "{}/{}",
        //                                     server.players,
        //                                     server.max_players
        //                                 ));
        //                             });
        //
        //                             row.col(|ui| {
        //                                 ui.label(format!("{} ms", server.ping));
        //                             });
        //
        //                             if row.response().clicked() {
        //                                 self.active_server = index;
        //                             }
        //                         });
        //                     }
        //                 });
        //         },
        //     );
        //
        //     ui.allocate_ui_with_layout(
        //         egui::vec2(detail_width, size.y),
        //         egui::Layout::top_down(egui::Align::LEFT),
        //         |ui| {
        //             ui.heading("Details");
        //             ui.label("Nothing selected");
        //         },
        //     );
        // });
        //
        //
        //
    // }
}
