use egui::{FontId, Ui};

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

    pub fn show_fields(
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

        // Espaciado según nivel de anidamiento
        let indent = 8.0 * level as f32;

        match &mut field.value {
            FieldValue::Array { value } => {
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new(field.label.as_deref().unwrap_or(key))
                        .font(label_font.clone()),
                );

                if let Some(desc) = field.description.as_deref() {
                    ui.add_space(2.0);
                    ui.label(egui::RichText::new(desc).font(description_font.clone()));
                }

                ui.collapsing(key, |ui| {
                    if let Some(desc) = field.description.as_deref() {
                        ui.add_space(2.0);
                        ui.small(desc);
                        ui.separator();
                    }

                    match value {
                        ArrayValue::Strings(strings) => {
                            for (i, s) in strings.iter_mut().enumerate() {
                                ui.add_space(2.0);
                                ui.horizontal(|ui| {
                                    ui.add_space(indent);
                                    ui.label(
                                        egui::RichText::new(format!("[{}]", i))
                                            .font(label_font.clone()),
                                    );
                                    ui.add(
                                        egui::TextEdit::singleline(s)
                                            .font(text_font.clone())
                                            .desired_width(f32::INFINITY),
                                    );
                                });
                            }
                        }
                        ArrayValue::Numbers(numbers) => {
                            for (i, n) in numbers.iter_mut().enumerate() {
                                ui.add_space(2.0);
                                ui.horizontal(|ui| {
                                    ui.add_space(indent);
                                    ui.label(
                                        egui::RichText::new(format!("[{}]", i))
                                            .font(label_font.clone()),
                                    );
                                    ui.add(egui::DragValue::new(n));
                                });
                            }
                        }
                        ArrayValue::Objects(objects) => {
                            for (i, obj) in objects.iter_mut().enumerate() {
                                ui.add_space(2.0);
                                ui.label(
                                    egui::RichText::new(format!("Item {}", i))
                                        .font(label_font.clone()),
                                );
                                ui.indent(format!("item_{}", i), |ui| {
                                    for (child_key, child_field) in obj.iter_mut() {
                                        Self::show_fields(
                                            child_field,
                                            ui,
                                            child_key,
                                            level + 1,
                                            form_config,
                                        );
                                    }
                                });
                            }
                        }
                        ArrayValue::Mixed(_) => {
                            ui.add_space(2.0);
                            ui.label("Mixed array (not editable)");
                        }
                    }
                });

                ui.add_space(4.0); // espacio al final del array
            }

            FieldValue::Boolean { value, default: _ } => {
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

            FieldValue::Float { value, default: _ } => {
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

            FieldValue::Integer { value, default: _ } => {
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

            FieldValue::Table { children } => {
                ui.add_space(4.0);
                let header_text = field.label.as_deref().unwrap_or(key);
                ui.collapsing(
                    egui::RichText::new(header_text).font(header_font.clone()),
                    |ui| {
                        if let Some(desc) = field.description.as_deref() {
                            ui.add_space(1.0);
                            ui.label(egui::RichText::new(desc).font(description_font.clone()));
                            ui.separator();
                        }
                        for (child_key, child_field) in children.iter_mut() {
                            Self::show_fields(child_field, ui, child_key, level + 1, form_config);
                        }
                    },
                );
                ui.add_space(4.0);
            }

            FieldValue::String { value, default } => {
                if value.is_empty() {
                    if let Some(def) = default {
                        *value = def.clone();
                    }
                }

                ui.add_space(4.0);
                ui.vertical(|ui| {
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
                });
                ui.add_space(4.0);
            }

            _ => {}
        }
    }
}
