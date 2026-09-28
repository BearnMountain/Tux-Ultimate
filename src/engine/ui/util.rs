

// ----- Layout -----
/// splits panel into top and buttom
pub fn split_v(ui: &mut egui::Ui, top_frac: f32, top: impl FnOnce(&mut egui::Ui), bottom: impl FnOnce(&mut egui::Ui)) {
    let rect = ui.available_rect_before_wrap();
    let top_h = rect.height() * top_frac;

    let top_rect = egui::Rect::from_min_size(rect.min, egui::vec2(rect.width(), top_h));
    let bottom_rect = egui::Rect::from_min_size(
        rect.min + egui::vec2(0.0, top_h),
        egui::vec2(rect.width(), rect.height() - top_h),
    );

    ui.scope_builder(egui::UiBuilder::new().max_rect(top_rect), top);
    ui.scope_builder(egui::UiBuilder::new().max_rect(bottom_rect), bottom);

    ui.allocate_rect(rect, egui::Sense::hover());
}

/// splits panel into left and right
pub fn split_h(ui: &mut egui::Ui, left_frac: f32, left: impl FnOnce(&mut egui::Ui), right: impl FnOnce(&mut egui::Ui)) {
    let rect = ui.available_rect_before_wrap();
    let left_w = rect.width() * left_frac;

    let left_rect = egui::Rect::from_min_size(rect.min, egui::vec2(left_w, rect.height()));
    let right_rect = egui::Rect::from_min_size(
        rect.min + egui::vec2(left_w, 0.0),
        egui::vec2(rect.width() - left_w, rect.height()),
    );

    ui.scope_builder(egui::UiBuilder::new().max_rect(left_rect), left);
    ui.scope_builder(egui::UiBuilder::new().max_rect(right_rect), right);

    ui.allocate_rect(rect, egui::Sense::hover());
}

// ----- Font -----

// crude letter-spacing: egui has no tracking property, so we fake it by
/// inserting thin spaces between characters for headers/titles.
pub fn tracked(s: &str) -> String {
    s.chars()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join("\u{2009}") // thin space
}
