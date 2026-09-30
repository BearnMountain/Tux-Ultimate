use std::{collections::btree_map::Range, ops::RangeBounds};

use egui::{Align, Color32, CornerRadius, Layout, Pos2, Rect, Response, Shape, Stroke, WidgetText, WidgetType::TextEdit, vec2};

use crate::game::ui::screens::MenuTheme;

pub fn input_field(
    ui: &mut egui::Ui,
    theme: &MenuTheme,
    label: &str,
    value: &mut String,
    char_limit: Option<usize>,
) -> Response {
    let mut response = None;

    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
        ui.label(label);

        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            let mut edit = egui::TextEdit::singleline(value);

            if let Some(limit) = char_limit {
                edit = edit.char_limit(limit);
            }
            response = Some(ui.add(edit));
        });
    });

    return response.unwrap();
}

pub fn combo_box(
    ui: &mut egui::Ui,
    theme: &MenuTheme,
    label: &str,
    selected: &mut usize,
    items: &[&str],
) -> bool {
    let accent: Color32 = theme.accent;
    let bg: Color32 = theme.background;
    let mut changed = false;

    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
        ui.label(label);
        ui.with_layout(
            egui::Layout::right_to_left(egui::Align::Center),
            |ui| {
                ui.scope(|ui| {
                    let visuals = ui.visuals_mut();

                    // customs for each mode of the ui
                    for w in [
                        &mut visuals.widgets.inactive,
                        &mut visuals.widgets.hovered,
                        &mut visuals.widgets.active,
                        &mut visuals.widgets.open,
                    ] {
                        w.bg_fill = Color32::TRANSPARENT;
                        w.weak_bg_fill = Color32::TRANSPARENT;
                        w.bg_stroke = Stroke::NONE;
                        w.corner_radius = CornerRadius::ZERO;
                        w.expansion = 0.0;
                        w.fg_stroke = Stroke::new(1.0, accent); // text + icon color
                    }

                    // dropdown list style
                    visuals.window_fill = bg;
                    visuals.window_stroke = Stroke::new(1.0, accent);
                    visuals.window_corner_radius = CornerRadius::ZERO;
                    visuals.selection.bg_fill = accent.gamma_multiply(0.15);
                    visuals.selection.stroke = Stroke::new(1.0, accent);

                    ui.spacing_mut().button_padding = vec2(0.0, 3.0);

                    let inner = egui::ComboBox::from_id_salt(label)
                        .width(ui.available_width() * 0.3)
                        .selected_text(WidgetText::from(items[*selected]).color(accent))
                        // custom outlined down-triangle instead of the default chevron
                        .icon(move |ui, rect, _visuals, is_open| {
                            let c = rect.center();
                            let s = 4.5;
                            let pts = if is_open {
                                vec![
                                    Pos2::new(c.x - s, c.y + s * 0.6),
                                    Pos2::new(c.x + s, c.y + s * 0.6),
                                    Pos2::new(c.x, c.y - s * 0.6),
                                ]
                            } else {
                                vec![
                                    Pos2::new(c.x - s, c.y - s * 0.6),
                                    Pos2::new(c.x + s, c.y - s * 0.6),
                                    Pos2::new(c.x, c.y + s * 0.6),
                                ]
                            };
                            ui.painter().add(Shape::convex_polygon(
                                pts,
                                Color32::TRANSPARENT,
                                Stroke::new(1.5, accent),
                            ));
                        })
                        .show_ui(ui, |ui| {
                            for (i, item) in items.iter().enumerate() {
                                if ui.selectable_value(selected, i, *item).changed() {
                                    changed = true;
                                }
                            }
                        });

                    // --- the underline ---
                    let r: Rect = inner.response.rect;
                    ui.painter().line_segment(
                        [r.left_bottom(), r.right_bottom()],
                        Stroke::new(1.0, accent),
                    );
                });
        });

    });

    return changed;
}

pub fn checkbox(
    ui: &mut egui::Ui,
    theme: &MenuTheme,
    label: &str,
    value: &mut bool,
) {
    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
        ui.label(label);
        ui.with_layout(
            egui::Layout::right_to_left(egui::Align::Center),
            |ui| {
                let size = 16.0;
                let (rect, response) = ui.allocate_exact_size(
                    egui::vec2(size, size), 
                    egui::Sense::click(),
                );

                // toggle checkbox
                if response.clicked() {
                    *value = !*value;
                }

                // draw checkbox
                let painter = ui.painter();
                painter.rect_stroke(
                    rect, 
                    0.0, 
                    egui::Stroke::new(
                        1.5,
                        ui.visuals().text_color(),
                    ),
                    egui::StrokeKind::Inside,
                );

                // draw inner square if checked
                if *value {
                    let margin = 4.0;
                    painter.rect_filled(
                        rect.shrink(margin),
                        0.0,
                        ui.visuals().text_color(),
                    );
                }
            },
        );
    });
}

pub fn stepper(
    ui: &mut egui::Ui,
    theme: &MenuTheme,
    label: &str,
    value: &mut i32,
) {
    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
        ui.label(label);
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if button_right_arrow(ui, egui::vec2(10.0, 10.0)).clicked() {
                *value = *value + 1;
            }

            let mut input = value.to_string();
            ui.text_edit_singleline(&mut input);
            if let Ok(i) = input.parse::<i32>() {
                *value = i;
            } else if input.len() == 0 {
                *value = 0;
            }
            if button_left_arrow(ui, egui::vec2(10.0, 10.0)).clicked() {
                *value = *value - 1;
            }
        });
    });
}

pub fn button(
    ui: &mut egui::Ui, 
    theme: &MenuTheme,
    label: &str, 
    size: egui::Vec2,
) -> bool {
    let available = ui.available_rect_before_wrap();
    let pos = available.center() - size / 2.0;
    let rect = egui::Rect::from_min_size(pos, size);

    let response = ui.interact(
        rect,
        ui.make_persistent_id(label),
        egui::Sense::click(),
    );

    let color = if response.hovered() {
        egui::Color32::from_rgb(90, 110, 90)
    } else {
        egui::Color32::from_rgb(60, 75, 60)
    };

    ui.painter().rect_filled(
        rect,
        8.0,
        color,
    );

    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(16.0),
        egui::Color32::WHITE,
    );

    return response.clicked();
}

pub fn slider_f32(
    ui: &mut egui::Ui,
    theme: &MenuTheme,
    label: &str,
    value: &mut f32,
    range: std::ops::RangeInclusive<f32>,
) -> egui::Response {
    let desired_size = ui.available_size();
    let (rect, mut response) =
        ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());

    // Split label + value
    let label_width = (rect.width() * 0.3).clamp(60.0, 160.0);
    let spacing = 12.0;

    let label_rect = egui::Rect::from_min_size(
        rect.min,
        egui::vec2(label_width, rect.height()),
    );
    let slider_rect = egui::Rect::from_min_max(
        egui::pos2(rect.left() + label_width + spacing, rect.top()),
        rect.max,
    );

    // Label
    ui.painter().text(
        egui::pos2(label_rect.left(), label_rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        egui::FontId::proportional(14.0),
        theme.text, 
    );

    // Change slider 
    if response.dragged() || response.clicked() {
        if let Some(pointer) = response.interact_pointer_pos() {
            let t = ((pointer.x - slider_rect.left()) / slider_rect.width())
                .clamp(0.0, 1.0);

            let min = *range.start();
            let max = *range.end();
            *value = min + (max - min) * t;
            response.mark_changed();
        }
    }

    let painter = ui.painter();
    let y = slider_rect.center().y;

    // Track
    painter.line_segment(
        [
            egui::pos2(slider_rect.left(), y),
            egui::pos2(slider_rect.right(), y),
        ],
        egui::Stroke::new(4.0, theme.border),
    );

    // Filled section
    let t = (*value - *range.start()) / (*range.end() - *range.start());
    let x = egui::lerp(slider_rect.left()..=slider_rect.right(), t);

    painter.line_segment(
        [egui::pos2(slider_rect.left(), y), egui::pos2(x, y)],
        egui::Stroke::new(4.0, theme.accent),
    );

    // Handle
    painter.rect_filled(
        egui::Rect::from_center_size(egui::pos2(x, y), egui::vec2(9.0, 18.0)),
        0.0,
        theme.accent,
    );

    return response;
}

pub fn slider_u32(
    ui: &mut egui::Ui,
    theme: &MenuTheme,
    label: &str,
    value: &mut u32,
    range: std::ops::RangeInclusive<u32>,
    height: f32,
) -> egui::Response {
    let desired_size = egui::vec2(ui.available_width(), height);
    let (rect, mut response) =
        ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());

    let label_width = (rect.width() * 0.3).clamp(60.0, 160.0);
    let spacing = 12.0;

    let label_rect = egui::Rect::from_min_size(rect.min, egui::vec2(label_width, rect.height()));
    let slider_rect = egui::Rect::from_min_max(
        egui::pos2(rect.left() + label_width + spacing, rect.top()),
        rect.max,
    );

    ui.painter().text(
        egui::pos2(label_rect.left(), label_rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        egui::FontId::proportional(14.0),
        theme.text,
    );

    let min = *range.start() as f32;
    let max = *range.end() as f32;

    if response.dragged() || response.clicked() {
        if let Some(pointer) = response.interact_pointer_pos() {
            let t = ((pointer.x - slider_rect.left()) / slider_rect.width()).clamp(0.0, 1.0);

            // Round, don't truncate, then clamp back into range as u32.
            let raw = min + (max - min) * t;
            let rounded = raw.round().clamp(min, max) as u32;

            if rounded != *value {
                *value = rounded;
                response.mark_changed();
            }
        }
    }

    let painter = ui.painter();
    let y = slider_rect.center().y;

    painter.line_segment(
        [egui::pos2(slider_rect.left(), y), egui::pos2(slider_rect.right(), y)],
        egui::Stroke::new(4.0, theme.border),
    );

    // Recompute t from the (now-integer) value for drawing, so the fill/handle
    // reflects the snapped position, not the raw pointer position.
    let t = (*value as f32 - min) / (max - min);
    let x = egui::lerp(slider_rect.left()..=slider_rect.right(), t);

    painter.line_segment(
        [egui::pos2(slider_rect.left(), y), egui::pos2(x, y)],
        egui::Stroke::new(4.0, theme.accent),
    );

    painter.rect_filled(
        egui::Rect::from_center_size(egui::pos2(x, y), egui::vec2(9.0, 18.0)),
        0.0,
        theme.accent,
    );

    response
}

// unique types for widgets
fn button_left_arrow(
    ui: &mut egui::Ui,
    dimensions: egui::Vec2,
) -> Response {
    let (rect, response) = ui.allocate_exact_size(
        dimensions,
        egui::Sense::click(),
    );
    let center = rect.center();

    let painter = ui.painter();

    let half = dimensions.x / 2.0;

    painter.add(Shape::line(
        vec![
            Pos2::new(center.x + half, center.y - half),
            Pos2::new(center.x - half, center.y),
            Pos2::new(center.x + half, center.y + half),
        ],
        Stroke::new(2.0, Color32::WHITE),
    ));

    return response;
}

fn button_right_arrow(
    ui: &mut egui::Ui, 
    dimensions: egui::Vec2,
) -> Response {
    let (rect, response) = ui.allocate_exact_size(
        dimensions,
        egui::Sense::click(),
    );
    let center = rect.center();

    let painter = ui.painter();

    let half = dimensions.x / 2.0;

    painter.add(egui::epaint::Shape::line(
        vec![
            Pos2::new(center.x - half, center.y - half),
            Pos2::new(center.x + half, center.y),
            Pos2::new(center.x - half, center.y + half),
        ],
        egui::Stroke::new(2.0, egui::Color32::WHITE),
    ));

    return response;
}



