use egui::{FontId, TextureOptions, Ui};
use image::imageops::FilterType;

use crate::{
    components::form_config::FormConfig,
    form::form::{ArrayValue, Document, Field, FieldValue},
};

pub struct Form {
    document: Document,
}

impl Form {
    pub fn new(document: Document) -> Self {
        Self { document }
    }

    pub fn show_state(&mut self, ui: &mut Ui, form_config: &FormConfig) {
        for (key, field) in self.document.fields.iter_mut() {
            Self::show_fields(field, ui, key, 0, form_config);
        }
    }

    // --------------------------
    // Helpers
    // --------------------------

    fn show_fields(
        field: &mut Field,
        ui: &mut Ui,
        key: &str,
        level: usize,
        form_config: &FormConfig,
    ) {
        let header_font = FontId::proportional(form_config.font_size_header);
        let label_font = FontId::proportional(form_config.font_size_label);
        let text_font = FontId::monospace(form_config.font_size_text);
        let description_font = FontId::monospace(form_config.font_size_description);
        let editable = field.editable.unwrap_or(false);
        let indent = 8.0 * level as f32;

        match &mut field.value {
            FieldValue::Array { value } => {
                Self::show_array(
                    ui,
                    field,
                    value,
                    key,
                    label_font,
                    text_font,
                    header_font,
                    description_font,
                    editable,
                    indent,
                    level,
                    form_config,
                );
            }

            FieldValue::String { .. } => {
                Self::show_field_value(
                    field,
                    ui,
                    key,
                    level,
                    editable,
                    label_font,
                    text_font,
                    header_font,
                    description_font,
                );
            }

            FieldValue::Boolean { .. }
            | FieldValue::Float { .. }
            | FieldValue::Integer { .. }
            | FieldValue::Image { .. }
            | FieldValue::Table { .. } => {
                Self::show_field_value(
                    field,
                    ui,
                    key,
                    level,
                    editable,
                    label_font,
                    text_font,
                    header_font,
                    description_font,
                );
            }

            _ => {}
        }
    }

    fn show_field_value(
        field: &mut Field,
        ui: &mut Ui,
        key: &str,
        level: usize,
        editable: bool,
        label_font: FontId,
        text_font: FontId,
        header_font: FontId,
        description_font: FontId,
    ) {
        let indent = 8.0 * level as f32;

        match &mut field.value {
            FieldValue::Boolean { value, .. } => {
                ui.add_space(4.0);
                ui.vertical(|ui| {
                    ui.add(egui::Checkbox::new(
                        value,
                        egui::RichText::new(field.label.as_deref().unwrap_or(key))
                            .font(label_font.clone()),
                    ));
                    if let Some(desc) = field.description.as_deref() {
                        ui.add_space(1.0);
                        ui.label(egui::RichText::new(desc).font(description_font.clone()));
                    }
                });
                ui.add_space(4.0);
            }

            FieldValue::Float { value, .. } => {
                ui.add_space(4.0);
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new(field.label.as_deref().unwrap_or(key))
                            .font(label_font.clone()),
                    );
                    ui.add_space(2.0);
                    ui.add(egui::Slider::new(value, 0.0..=100.0).clamp_to_range(true));
                    if let Some(desc) = field.description.as_deref() {
                        ui.add_space(1.0);
                        ui.label(egui::RichText::new(desc).font(description_font.clone()));
                    }
                });
                ui.add_space(4.0);
            }

            FieldValue::Integer { value, .. } => {
                ui.add_space(4.0);
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new(field.label.as_deref().unwrap_or(key))
                            .font(label_font.clone()),
                    );
                    ui.add_space(2.0);
                    ui.add(egui::Slider::new(value, 0..=100).clamp_to_range(true));
                    if let Some(desc) = field.description.as_deref() {
                        ui.add_space(1.0);
                        ui.label(egui::RichText::new(desc).font(description_font.clone()));
                    }
                });
                ui.add_space(4.0);
            }

            FieldValue::String { value, default } => {
                if value.is_empty() {
                    if let Some(def) = default {
                        *value = def.clone();
                    }
                }
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.add_space(indent);
                    ui.label(
                        egui::RichText::new(field.label.as_deref().unwrap_or(key))
                            .font(label_font.clone()),
                    );
                    ui.add(
                        egui::TextEdit::singleline(value)
                            .font(text_font.clone())
                            .desired_width(f32::INFINITY),
                    );
                });
                if let Some(desc) = field.description.as_deref() {
                    ui.add_space(1.0);
                    ui.label(egui::RichText::new(desc).font(description_font.clone()));
                }
                ui.add_space(4.0);
            }

            FieldValue::Table { children } => {
                ui.add_space(4.0);
                ui.collapsing(
                    egui::RichText::new(field.label.as_deref().unwrap_or(key))
                        .font(header_font.clone()),
                    |ui| {
                        if let Some(desc) = field.description.as_deref() {
                            ui.add_space(1.0);
                            ui.label(egui::RichText::new(desc).font(description_font.clone()));
                            ui.separator();
                        }
                        for (child_key, child_field) in children.iter_mut() {
                            Self::show_fields(
                                child_field,
                                ui,
                                child_key,
                                level + 1,
                                &FormConfig {
                                    ..Default::default()
                                },
                            );
                        }
                    },
                );
                ui.add_space(4.0);
            }

            FieldValue::Image { value, texture } => {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Add texture").clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("Images", &["png"])
                                .pick_file()
                            {
                                *value = path.to_string_lossy().to_string();

                                if let Ok(img) = image::open(&path) {
                                    let mut img = img.to_rgba8();
                                    let width = img.width();
                                    let height = img.height();
                                    let min_size = 128;

                                    if width < min_size || height < min_size {
                                        let scale_w = min_size.max(width);
                                        let scale_h = min_size.max(height);
                                        img = image::imageops::resize(
                                            &img,
                                            scale_w,
                                            scale_h,
                                            FilterType::Nearest,
                                        );
                                    }

                                    let size = [img.width() as _, img.height() as _];
                                    let pixels = img.into_raw();

                                    *texture = Some(ui.ctx().load_texture(
                                        value.clone(),
                                        egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
                                        TextureOptions::NEAREST,
                                    ));
                                }
                            }
                        }

                        if !value.is_empty() {
                            ui.label(
                                egui::RichText::new(field.label.as_deref().unwrap_or(key))
                                    .font(text_font.clone()),
                            );
                        }
                    });

                    if let Some(tex) = texture.as_ref() {
                        ui.add_space(2.0);
                        ui.add(egui::Image::new(tex).max_width(128.0));
                    }
                });
            }

            _ => {}
        }
    }

    fn show_array(
        ui: &mut Ui,
        field: &mut Field,
        value: &mut ArrayValue,
        key: &str,
        label_font: FontId,
        text_font: FontId,
        header_font: FontId,
        description_font: FontId,
        editable: bool,
        indent: f32,
        level: usize,
        form_config: &FormConfig,
    ) {
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new(field.label.as_deref().unwrap_or(key)).font(label_font.clone()),
        );

        if let Some(desc) = field.description.as_deref() {
            ui.add_space(2.0);
            ui.label(egui::RichText::new(desc).font(description_font.clone()));
        }

        ui.collapsing(key, |ui| match value {
            ArrayValue::Strings(strings) => Self::show_array_strings(
                ui,
                strings,
                label_font.clone(),
                text_font.clone(),
                editable,
                indent,
            ),
            ArrayValue::Numbers(numbers) => {
                Self::show_array_numbers(ui, numbers, label_font.clone(), editable, indent)
            }
            ArrayValue::Objects(objects) => Self::show_array_objects(
                ui,
                objects,
                field,
                level,
                form_config,
                label_font.clone(),
                text_font.clone(),
                editable,
                indent,
            ),
            ArrayValue::Mixed(values) => {
                for (i, v) in values.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.add_space(indent);
                        ui.label(
                            egui::RichText::new(format!("[{}]: {}", i, v))
                                .font(text_font.clone())
                                .color(egui::Color32::LIGHT_GRAY),
                        );
                    });
                }
            }
        });

        ui.add_space(4.0);
    }

    // --------------------------
    // Array helpers
    // --------------------------
    fn show_array_strings(
        ui: &mut Ui,
        strings: &mut Vec<String>,
        label_font: FontId,
        text_font: FontId,
        editable: bool,
        indent: f32,
    ) {
        let mut remove_index: Option<usize> = None;
        for (i, s) in strings.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.add_space(indent);
                ui.label(egui::RichText::new(format!("[{}]", i)).font(label_font.clone()));
                ui.add(
                    egui::TextEdit::singleline(s)
                        .font(text_font.clone())
                        .desired_width(500.0),
                );
                if editable && ui.button("X").clicked() {
                    remove_index = Some(i);
                }
            });
        }
        if let Some(i) = remove_index {
            strings.remove(i);
        }
        if editable && ui.button("Add").clicked() {
            strings.push(String::new());
        }
    }

    fn show_array_numbers(
        ui: &mut Ui,
        numbers: &mut Vec<f64>,
        label_font: FontId,
        editable: bool,
        indent: f32,
    ) {
        let mut remove_index: Option<usize> = None;
        for (i, n) in numbers.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.add_space(indent);
                ui.label(egui::RichText::new(format!("[{}]", i)).font(label_font.clone()));
                ui.add(egui::DragValue::new(n));
                if editable && ui.button("X").clicked() {
                    remove_index = Some(i);
                }
            });
        }
        if let Some(i) = remove_index {
            numbers.remove(i);
        }
        if editable && ui.button("Add").clicked() {
            numbers.push(0.0);
        }
    }

    fn show_array_objects(
        ui: &mut Ui,
        objects: &mut Vec<std::collections::HashMap<String, Field>>,
        field: &Field,
        level: usize,
        form_config: &FormConfig,
        label_font: FontId,
        text_font: FontId,
        editable: bool,
        indent: f32,
    ) {
        let mut remove_index: Option<usize> = None;
        for (i, obj) in objects.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.add_space(indent);
                ui.label(egui::RichText::new(format!("Item {}", i)).font(label_font.clone()));
                if editable && ui.button("X").clicked() {
                    remove_index = Some(i);
                }
            });
            ui.indent(format!("item_{}", i), |ui| {
                for (child_key, child_field) in obj.iter_mut() {
                    Self::show_fields(child_field, ui, child_key, level + 1, form_config);
                }
            });
        }
        if let Some(i) = remove_index {
            objects.remove(i);
        }
        if editable && ui.button("Add").clicked() {
            objects.push(Default::default());
        }
    }
}
