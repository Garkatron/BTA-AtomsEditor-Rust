use egui::Ui;

use crate::form::form::{Document, Field, FieldValue};

pub struct Form {
    document: Document,
}

impl Form {
    pub fn new(document: Document) -> Self {
        Self { document }
    }

    pub fn show_state(&mut self, ui: &mut Ui) {
        for (key, field) in self.document.fields.iter_mut() {
            Self::show_fields(field, ui, key, 0);
        }
    }

    pub fn show_fields(field: &mut Field, ui: &mut Ui, key: &str, level: usize) {
        let indent = "  ".repeat(level);

        match &mut field.value {
            FieldValue::Array { items } => {
                // format!("{}{}:", indent, key)
                ui.label(field.label.as_deref().unwrap_or(key));
                ui.indent(key, |ui| {
                    for f in items {
                        for (child_key, child_field) in f.iter_mut() {
                            Self::show_fields(child_field, ui, child_key, level + 1);
                        }
                    }
                });
            }
            FieldValue::Boolean { value, default } => {
                if *value == false {
                    if let Some(def) = default {
                        // value = def;
                    }
                }

                ui.vertical(|ui| {
                    ui.add(egui::Checkbox::new(
                        value,
                        field.label.as_deref().unwrap_or(key),
                    ));
                });
            }
            FieldValue::Float { value, default } => {
                ui.label(field.label.as_deref().unwrap_or(key));
                if *value == 0.0 {
                    if let Some(def) = default {
                        // value = def;
                    }
                }

                ui.vertical(|ui| {
                    ui.add(
                        egui::Slider::new(value, 0.0..=100.0).clamping(egui::SliderClamping::Edits),
                    );
                });
            }
            FieldValue::Integer { value, default } => {
                ui.label(field.label.as_deref().unwrap_or(key));
                if *value == 0 {
                    if let Some(def) = default {
                        // value = def;
                    }
                }

                ui.vertical(|ui| {
                    ui.add(egui::Slider::new(value, 0..=100).clamping(egui::SliderClamping::Edits));
                });
            }
            FieldValue::Table { children } => {
                // ui.label(format!("{}{}:", indent, key));
                ui.label(field.label.as_deref().unwrap_or(key));
                ui.indent(key, |ui| {
                    for (child_key, child_field) in children.iter_mut() {
                        Self::show_fields(child_field, ui, child_key, level + 1);
                    }
                });
            }
            FieldValue::String { value, default } => {
                ui.horizontal(|ui| {
                    ui.label(field.label.as_deref().unwrap_or(key));
                    ui.add(
                        egui::TextEdit::multiline(value)
                            .desired_width(f32::INFINITY)
                            .font(egui::TextStyle::Monospace),
                    );
                });
            }
            _ => {}
        }
    }
}
