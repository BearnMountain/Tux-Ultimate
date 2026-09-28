use egui::{Response, WidgetType::TextEdit};

use crate::game::ui::screens::MenuTheme;



pub fn input_field(
    ui: &mut egui::Ui,
    theme: &MenuTheme,
    label: &str,
    value: &mut String,
    char_limit: Option<usize>,
) -> Response {
    let mut response = None;

    ui.horizontal(|ui| {
        ui.label(label);

        let mut edit = egui::TextEdit::singleline(value);

        if let Some(limit) = char_limit {
            edit = edit.char_limit(limit);
        }
        response = Some(ui.add(edit));
    });

    return response.unwrap();
}
