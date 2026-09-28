use egui::{FontId, RichText};
use image::Frame;

use crate::{engine::ui::{util, widgets}, game::{server::net::config::GameServerConfig, ui::screens::MenuTheme}};

pub struct HostServer {
    pub lan_only: bool,
    pub server_config: GameServerConfig,

    pub theme: MenuTheme,

    // helps with ui
    category_list: Vec<String>,
    active_category: u8, // relates to index in category_list
}

impl HostServer {
    pub fn new() -> Self {
        return Self {
            lan_only: false,
            server_config: GameServerConfig::default(),
            theme: MenuTheme::dark(),
            category_list: vec![
                "Server Options".into(),
                "Game Content".into(),
                "Game Options".into(),
                "Host".into(),
            ],
            active_category: 0,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // dividing area for each box
        
        egui::CentralPanel::default().show(ui, |ui| {
            let size = ui.available_size();
            let header_h = ui.available_size().y * 0.2;
            let category_w = ui.available_size().x * 0.35;
            let config_w = ui.available_size().x - category_w;

            ui.allocate_ui_with_layout(
                egui::vec2(size.x, header_h), 
                egui::Layout::top_down(egui::Align::TOP), 
                |ui| {
                    ui.set_min_size(egui::vec2(size.x, header_h));
                    self.header_panel(ui);
                }
            );

            let body_h = ui.available_height(); // already shrunk by header_h, no manual subtraction needed
            ui.horizontal(|ui| {
                ui.allocate_ui_with_layout(
                    egui::vec2(category_w, body_h),
                    egui::Layout::top_down(egui::Align::TOP),
                    |ui| {
                        ui.set_min_size(egui::vec2(category_w, body_h));
                        self.category_panel(ui);
                    },
                );

                ui.allocate_ui_with_layout(
                    egui::vec2(config_w, body_h),
                    egui::Layout::top_down(egui::Align::TOP),
                    |ui| {
                        ui.set_min_size(egui::vec2(config_w, body_h));
                        self.configuration_panel(ui);
                    },
                );
            });
        });
    }

    fn header_panel(&mut self, ui: &mut egui::Ui) {
        // ai slop filler
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.label(
                RichText::new(util::tracked("PLAY"))
                    .color(self.theme.text)
                    .font(FontId::proportional(24.0)),
            );
            ui.add_space(10.0);
            ui.label(RichText::new("\u{203A}").color(self.theme.text).size(22.0));
            ui.add_space(10.0);
            ui.label(
                RichText::new(util::tracked("CUSTOM GAME"))
                    .color(self.theme.text)
                    .strong()
                    .font(FontId::proportional(24.0)),
            );
        });
    }

    fn category_panel(&mut self, ui: &mut egui::Ui) {
        let size = ui.available_size();
        
        for (i, item) in self.category_list.iter().enumerate() {
            let button = egui::Button::new(
                egui::RichText::new(item).size(18.0)
            ).frame(false);

            let response = ui.add(button);
            if response.hovered() {
                let rect = response.rect;

                ui.painter().rect_filled(
                    rect,
                    0.0,
                    ui.visuals().selection.bg_fill,
                );
            } 
            if response.clicked() {
                println!("{i}");
            }

            // Separator
            let y = response.rect.bottom();

            ui.painter().line_segment(
                [
                    egui::pos2(response.rect.left(), y),
                    egui::pos2(response.rect.right(), y),
                ],
                egui::Stroke::new(
                    1.0,
                    ui.visuals().widgets.noninteractive.bg_stroke.color,
                ),
            );
        }
    }

    fn configuration_panel(&mut self, ui: &mut egui::Ui) {

        match self.active_category {
            // ----- Server Options -----
            // server name, port, lobby options, player count, etc
            0 => {
                widgets::input_field(
                    ui, 
                    &self.theme, 
                    "Server Name: ".into(), 
                    &mut self.server_config.server_name, 
                    Some(10),
                );
            },

            // ----- Game Content -----
            // map selection/voting, character limits etc
            1 => {

            },

            // ----- Game Options -----
            // scalars, lives, round length, sudden death length, rounds, gamemode
            2 => {

            },


            // ----- Host -----
            // new lobby window, ip:port exposed
            3 => {

            },
            _ => { println!("error"); },
        }
    }
}
