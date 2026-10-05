use std::cmp::max;

use egui::{FontId, RichText, ScrollArea};
use egui_extras::{Size, StripBuilder};
use image::Frame;

use crate::{engine::ui::{util, widgets}, game::{server::net::config::GameServerConfig, ui::{command::UiCommand, screens::{MenuTheme, Screen}}}};

pub struct HostServer {
    pub lan_only: bool,
    pub test: i32,
    pub server_config: GameServerConfig,

    pub theme: MenuTheme,

    // helps with ui
    category_list: Vec<String>,
    active_category: u8, // relates to index in category_list
    host_server_dialog: bool,
}

impl Screen for HostServer {
    fn ui(&mut self, ui: &mut egui::Ui) -> Vec<UiCommand> {
        // dividing area for each box
        egui::CentralPanel::default().show(ui, |ui| {
            StripBuilder::new(ui)
                .size(Size::relative(0.1)) // top: 10%
                .size(Size::remainder())
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        self.header_panel(ui);
                    });
                    strip.strip(|builder| { // bottom
                        builder
                            .size(Size::relative(0.4)) // 40% left
                            .size(Size::remainder()) // 60% right
                            .horizontal(|mut strip| {
                                strip.strip(|builder| {
                                    builder
                                        .size(Size::relative(0.9))
                                        .size(Size::remainder())
                                        .vertical(|mut strip| {
                                            strip.cell(|ui| {
                                                self.category_panel(ui);
                                            });
                                            strip.cell(|ui| {
                                                self.host_panel(ui);
                                            });
                                        });
                                });
                                strip.cell(|ui| {
                                    self.configuration_panel(ui);
                                });
                            });
                    })
                });
        });

        // start server
        if self.host_server_dialog {
            egui::Modal::new(egui::Id::new("host_dialog_popup"))
                .show(ui, |ui| {
                ui.heading("Confirm");

                ui.label("Are you sure you want to continue?");

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        self.host_server_dialog = false;
                    }

                    if ui.button("Confirm").clicked() {
                        self.host_server_dialog = false;
                    }
                });
            });
        }

        return vec![UiCommand::None];
    }
}

impl HostServer {
    pub fn new() -> Self {
        return Self {
            lan_only: false,
            test: 0,
            server_config: GameServerConfig::default(),
            theme: MenuTheme::dark(),
            category_list: vec![
                "Server Options".into(),
                "Game Content".into(),
                "Game Options".into(),
            ],
            active_category: 0,
            host_server_dialog: false,
        }
    }

    fn header_panel(&mut self, ui: &mut egui::Ui) {
        ui.label("PLAY");
        // ui.add_space(6.0);
        // ui.horizontal(|ui| {
        //     ui.label(
        //         RichText::new(util::tracked("PLAY"))
        //             .color(self.theme.text)
        //             .font(FontId::proportional(24.0)),
        //     );
        //     ui.add_space(10.0);
        //     ui.label(RichText::new("\u{203A}").color(self.theme.text).size(22.0));
        //     ui.add_space(10.0);
        //     ui.label(
        //         RichText::new(util::tracked("CUSTOM GAME"))
        //             .color(self.theme.text)
        //             .strong()
        //             .font(FontId::proportional(24.0)),
        //     );
        // });
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
                self.active_category = i as u8;
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

    fn host_panel(&mut self, ui: &mut egui::Ui) {
        let size = ui.available_size();

        if widgets::button(
            ui, 
            &self.theme, 
            "Host Server", 
            egui::vec2(size.x * 0.8, size.y * 0.8),
        ) {
            self.host_server_dialog = true;
        }
    }

    fn configuration_panel(&mut self, ui: &mut egui::Ui) {
        const entry_height: f32 = 60.0;

        ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
            let size = egui::vec2(ui.available_width(), 30.0);
            match self.active_category {
                // ----- Server Options -----
                // server name, port, lobby options, player count, etc
                0 => {
                    ui.allocate_ui(size, |ui| {
                        widgets::input_field(
                            ui, &self.theme, 
                            "Server Name", &mut self.server_config.server_name, 
                            Some(24),
                        );
                    });

                    ui.allocate_ui(size, |ui| {
                        let mut input: String = self.server_config.port.to_string();
                        widgets::input_field(
                            ui, &self.theme, 
                            "Port", &mut input, 
                            Some(24),
                        );
                        if let Ok(i) = input.parse::<u32>() {
                            self.server_config.port = i;
                        } else if input.len() == 0 {
                            self.server_config.port = 0;
                        }
                    });
                    ui.allocate_ui(size, |ui| {
                        let mut input: i32 = self.server_config.max_players as i32;
                        widgets::stepper(
                            ui, &self.theme, 
                            "Max Players", &mut input,
                        );
                        if input >= 0 && input <= 255 {
                            self.server_config.max_players = input as u8;
                        }
                    });
                    ui.allocate_ui(size, |ui| {
                        let mut input: i32 = self.server_config.tick_rate as i32;
                        widgets::stepper(
                            ui, &self.theme, 
                            "Tick Rate", &mut input,
                        );
                        if input >= 0 && input <= 255 {
                            self.server_config.tick_rate = input as u16;
                        }
                    });
                },

                // ----- Game Content -----
                // map selection/voting, character limits etc
                1 => {

                },

                // ----- Game Options -----
                // scalars, lives, round length, sudden death length, rounds, gamemode
                2 => {

                },

                _ => { println!("error"); },
            }

        });


    }
}
