use egui::{
    Align2, Area, Button, Color32, Context, Frame, Margin,
    RichText, Stroke, Vec2,
};

use crate::game::ui::{command::UiCommand, screens::{Screen, ScreenId}};

use super::MenuTheme;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainMenuAction {
    SINGLE_PLAYER,
    MULTIPLAYER,
    SETTINGS,
    EXTRAS,
    QUIT,
    NONE,
}

pub struct MainMenu {
    pub title: String,
    pub subtitle: String,
    pub version: String,

    pub theme: MenuTheme,
}

impl Default for MainMenu {
    fn default() -> Self {
        Self {
            title: "ULTIMATE".into(),
            subtitle: "A Rust-powered fighting game".into(),
            version: "v0.1.0".into(),
            theme: MenuTheme::dark(),
        }
    }
}

impl Screen for MainMenu {
    fn ui(&mut self, ui: &mut egui::Ui) -> Vec<UiCommand> {
        let theme = &self.theme;

        let mut action = UiCommand::None;

        // Center menu
        Area::new("main_menu".into())
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ui, |ui| {
                ui.set_width(360.0);

                Frame::NONE
                    .fill(theme.panel)
                    .corner_radius(12.0)
                    .inner_margin(Margin::same(40))
                    .show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            // Title
                            ui.label(
                                RichText::new(&self.title)
                                    .size(theme.title_size)
                                    .strong()
                                    .color(theme.text),
                            );

                            ui.add_space(6.0);

                            // Subtitle
                            ui.label(
                                RichText::new(&self.subtitle)
                                    .size(14.0)
                                    .color(theme.muted_text),
                            );

                            ui.add_space(35.0);

                            // Buttons
                            if Self::menu_button(
                                ui,
                                "SINGLE PLAYER",
                                theme,
                            ) {
                                action = UiCommand::Push(ScreenId::SINGLE_PLAYER);
                            }

                            ui.add_space(theme.spacing);

                            if Self::menu_button(
                                ui,
                                "MULTIPLAYER",
                                theme,
                            ) {
                                // set server browser as default entry
                                action = UiCommand::Push(ScreenId::SERVER_BROWSER);
                            }

                            ui.add_space(theme.spacing);

                            if Self::menu_button(
                                ui,
                                "SETTINGS",
                                theme,
                            ) {
                                action = UiCommand::Push(ScreenId::SETTINGS);
                            }

                            ui.add_space(theme.spacing);

                            if Self::menu_button(
                                ui,
                                "EXTRAS",
                                theme,
                            ) {
                                action = UiCommand::Push(ScreenId::EXTRAS);
                            }

                            ui.add_space(theme.spacing);

                            if Self::menu_button(
                                ui,
                                "QUIT",
                                theme,
                            ) {
                                action = UiCommand::Quit;
                            }

                            ui.add_space(30.0);

                            // Version
                            ui.label(
                                RichText::new(&self.version)
                                    .size(11.0)
                                    .color(theme.muted_text),
                            );
                        });
                    });
            });

        return vec![action];
    }
}

impl MainMenu {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    fn menu_button(
        ui: &mut egui::Ui,
        text: &str,
        theme: &MenuTheme,
    ) -> bool {
        let button = Button::new(
            RichText::new(text)
                .size(15.0)
                .strong()
                .color(theme.text),
        )
        .min_size(theme.button_size)
        .fill(theme.button)
        .stroke(Stroke::NONE)
        .corner_radius(8.0);

        ui.add(button)
            .on_hover_ui(|ui| {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            })
            .clicked()
    }
}
