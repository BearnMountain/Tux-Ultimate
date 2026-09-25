use egui::{
    Align2, Area, Button, Color32, Context, Frame, Margin,
    RichText, Stroke, Vec2,
};

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

    pub theme: MainMenuTheme,
}

#[derive(Debug, Clone)]
pub struct MainMenuTheme {
    pub background: Color32,
    pub panel: Color32,
    pub text: Color32,
    pub muted_text: Color32,
    pub accent: Color32,
    pub button: Color32,
    pub button_hovered: Color32,

    pub title_size: f32,
    pub button_size: Vec2,
    pub spacing: f32,
}

impl Default for MainMenuTheme {
    fn default() -> Self {
        Self {
            background: Color32::from_rgb(12, 12, 16),
            panel: Color32::from_rgb(20, 20, 26),

            text: Color32::from_rgb(235, 235, 240),
            muted_text: Color32::from_rgb(140, 140, 150),

            accent: Color32::from_rgb(90, 150, 255),

            button: Color32::from_rgb(28, 28, 36),
            button_hovered: Color32::from_rgb(42, 42, 54),

            title_size: 52.0,
            button_size: Vec2::new(280.0, 48.0),
            spacing: 12.0,
        }
    }
}

impl Default for MainMenu {
    fn default() -> Self {
        Self {
            title: "ULTIMATE".into(),
            subtitle: "A Rust-powered fighting game".into(),
            version: "v0.1.0".into(),
            theme: MainMenuTheme::default(),
        }
    }
}

impl MainMenu {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    pub fn ui(&mut self, ctx: &Context) -> MainMenuAction {
        let theme = &self.theme;

        let mut action = MainMenuAction::NONE;

        // Center menu
        Area::new("main_menu".into())
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ctx, |ui| {
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
                                action = MainMenuAction::SINGLE_PLAYER;
                            }

                            ui.add_space(theme.spacing);

                            if Self::menu_button(
                                ui,
                                "MULTIPLAYER",
                                theme,
                            ) {
                                action = MainMenuAction::MULTIPLAYER;
                            }

                            ui.add_space(theme.spacing);

                            if Self::menu_button(
                                ui,
                                "SETTINGS",
                                theme,
                            ) {
                                action = MainMenuAction::SETTINGS;
                            }

                            ui.add_space(theme.spacing);

                            if Self::menu_button(
                                ui,
                                "EXTRAS",
                                theme,
                            ) {
                                action = MainMenuAction::EXTRAS;
                            }

                            ui.add_space(theme.spacing);

                            if Self::menu_button(
                                ui,
                                "QUIT",
                                theme,
                            ) {
                                action = MainMenuAction::QUIT;
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

        action
    }

    fn menu_button(
        ui: &mut egui::Ui,
        text: &str,
        theme: &MainMenuTheme,
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
