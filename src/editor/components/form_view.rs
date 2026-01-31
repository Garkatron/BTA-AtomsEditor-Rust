use egui::{FontId, TextureOptions, Ui};
use image::imageops::FilterType;
use indexmap::IndexMap;
use toml::Value;

use crate::editor::{
    components::form_config::FormConfig,
    form::form::{ArrayValue, Document, EnumField, Field, FieldType, FieldValue},
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

    // ==============================================
    // FIELD RENDERING
    // ==============================================

    fn show_fields(
        field: &mut Field,
        ui: &mut Ui,
        key: &str,
        level: usize,
        form_config: &FormConfig,
    ) {
        let fonts = FormFonts::from_config(form_config);
        let editable = field.editable.unwrap_or(false);
        let template = field.template.unwrap_or(false);
        let indent = 8.0 * level as f32;

        match &mut field.value {
            FieldValue::Array { value } => {
                let label = field.label.clone();
                let description = field.description.clone();

                Self::show_array(
                    ui,
                    value,
                    key,
                    label.as_deref(),
                    description.as_deref(),
                    fonts,
                    editable,
                    template,
                    indent,
                    level,
                    form_config,
                );
            }

            FieldValue::Boolean { .. }
            | FieldValue::Float { .. }
            | FieldValue::Integer { .. }
            | FieldValue::String { .. }
            | FieldValue::Image { .. }
            | FieldValue::Enum { .. }
            | FieldValue::Table { .. } => {
                Self::show_field_value(field, ui, key, level, editable, template, fonts);
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
        template: bool,
        fonts: FormFonts,
    ) {
        let indent = 8.0 * level as f32;
        let label = field.label.as_deref().unwrap_or(key);

        match &mut field.value {
            FieldValue::Boolean { value, .. } => {
                Self::render_boolean(ui, value, label, &field.description, fonts);
            }

            FieldValue::Float { value, .. } => {
                Self::render_float(ui, value, label, &field.description, fonts);
            }

            FieldValue::Integer { value, .. } => {
                Self::render_integer(ui, value, label, &field.description, fonts);
            }

            FieldValue::String { value, default } => {
                Self::render_string(ui, value, default, label, &field.description, indent, fonts);
            }

            FieldValue::Table { children } => {
                Self::render_table(
                    ui,
                    children,
                    label,
                    &field.description,
                    level,
                    fonts,
                    editable,
                    template,
                );
            }

            FieldValue::Image { value, texture } => {
                Self::render_image(ui, value, texture, label, fonts);
            }

            FieldValue::Enum { value, options } => {
                Self::render_enum(ui, value, options, label, &field.description, fonts, indent);
            }

            _ => {}
        }
    }

    // ==============================================
    // INDIVIDUAL FIELD RENDERERS
    // ==============================================

    fn render_enum(
        ui: &mut Ui,
        value: &mut String,
        options: &mut Vec<String>,
        label: &str,
        description: &Option<String>,
        fonts: FormFonts,
        indent: f32,
    ) {
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.add_space(indent);
            ui.label(egui::RichText::new(label).font(fonts.label));

            if !options.is_empty() && !options.contains(value) {
                *value = options[0].clone();
            }

            let combo_id = ui.make_persistent_id(format!("enum_combo_{}", label));

            egui::ComboBox::from_id_salt(combo_id)
                .selected_text(if value.is_empty() {
                    "Select..."
                } else {
                    value.as_str()
                })
                .show_ui(ui, |ui| {
                    for option in options.iter() {
                        ui.selectable_value(value, option.clone(), option.as_str());
                    }
                });
        });
        Self::render_description(ui, description, &fonts.description);
        ui.add_space(4.0);
    }
    fn render_boolean(
        ui: &mut Ui,
        value: &mut bool,
        label: &str,
        description: &Option<String>,
        fonts: FormFonts,
    ) {
        ui.add_space(4.0);
        ui.vertical(|ui| {
            ui.add(egui::Checkbox::new(
                value,
                egui::RichText::new(label).font(fonts.label),
            ));
            Self::render_description(ui, description, &fonts.description);
        });
        ui.add_space(4.0);
    }

    fn render_float(
        ui: &mut Ui,
        value: &mut f64,
        label: &str,
        description: &Option<String>,
        fonts: FormFonts,
    ) {
        ui.add_space(4.0);
        ui.vertical(|ui| {
            ui.label(egui::RichText::new(label).font(fonts.label));
            ui.add_space(2.0);
            ui.add(egui::Slider::new(value, 0.0..=100.0).clamping(egui::SliderClamping::Edits));
            Self::render_description(ui, description, &fonts.description);
        });
        ui.add_space(4.0);
    }

    fn render_integer(
        ui: &mut Ui,
        value: &mut i64,
        label: &str,
        description: &Option<String>,
        fonts: FormFonts,
    ) {
        ui.add_space(4.0);
        ui.vertical(|ui| {
            ui.label(egui::RichText::new(label).font(fonts.label));
            ui.add_space(2.0);
            ui.add(egui::Slider::new(value, 0..=100).clamping(egui::SliderClamping::Edits));
            Self::render_description(ui, description, &fonts.description);
        });
        ui.add_space(4.0);
    }

    fn render_string(
        ui: &mut Ui,
        value: &mut String,
        default: &Option<String>,
        label: &str,
        description: &Option<String>,
        indent: f32,
        fonts: FormFonts,
    ) {
        if value.is_empty() {
            if let Some(def) = default {
                *value = def.clone();
            }
        }

        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.add_space(indent);
            ui.label(egui::RichText::new(label).font(fonts.label));
            ui.add(
                egui::TextEdit::singleline(value)
                    .font(fonts.text)
                    .desired_width(f32::INFINITY),
            );
        });
        Self::render_description(ui, description, &fonts.description);
        ui.add_space(4.0);
    }

    fn render_table(
        ui: &mut Ui,
        children: &mut IndexMap<String, Field>,
        label: &str,
        description: &Option<String>,
        level: usize,
        fonts: FormFonts,
        editable: bool,
        template: bool,
    ) {
        ui.add_space(4.0);
        ui.collapsing(
            egui::RichText::new(label)
                .font(fonts.label.clone())
                .strong(),
            |ui| {
                if let Some(desc) = description.as_deref() {
                    ui.add_space(1.0);
                    ui.label(
                        egui::RichText::new(desc)
                            .font(fonts.description.clone())
                            .color(egui::Color32::GRAY),
                    );
                    ui.separator();
                }
                let mut remove_key: Option<String> = None;
                for (child_key, child_field) in children.iter_mut() {
                    ui.horizontal(|ui| {
                        Self::show_fields(
                            child_field,
                            ui,
                            child_key,
                            level + 1,
                            &FormConfig::default(),
                        );
                        if editable {
                            if ui
                                .add(
                                    egui::Button::new("✕")
                                        .fill(egui::Color32::TRANSPARENT)
                                        .stroke(egui::Stroke::NONE),
                                )
                                .clicked()
                            {
                                remove_key = Some(child_key.to_string());
                            }
                        }
                    });
                }
                if let Some(key) = remove_key {
                    children.shift_remove(&key);
                }
                if editable {
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Add field:");
                        let combo_id = ui.make_persistent_id(format!("field_type_combo_{}", label));
                        let text_id = ui.make_persistent_id(format!("new_key_text_{}", label));

                        let mut selected = ui.data_mut(|d| {
                            d.get_persisted::<FieldType>(combo_id)
                                .unwrap_or(FieldType::String)
                        });

                        if !template {
                            egui::ComboBox::from_id_salt(combo_id)
                                .selected_text(format!("{:?}", selected))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut selected, FieldType::String, "String");
                                    ui.selectable_value(
                                        &mut selected,
                                        FieldType::Integer,
                                        "Integer",
                                    );
                                    ui.selectable_value(&mut selected, FieldType::Float, "Float");
                                    ui.selectable_value(
                                        &mut selected,
                                        FieldType::Boolean,
                                        "Boolean",
                                    );
                                    ui.selectable_value(&mut selected, FieldType::Image, "Image");
                                    ui.selectable_value(&mut selected, FieldType::Table, "Table");
                                    ui.selectable_value(&mut selected, FieldType::Array, "Array");
                                });
                            ui.data_mut(|d| d.insert_persisted(combo_id, selected));
                        }

                        let mut new_key = ui.data_mut(|d| {
                            d.get_persisted::<String>(text_id)
                                .unwrap_or_else(|| String::from("value"))
                        });

                        ui.text_edit_singleline(&mut new_key);
                        ui.data_mut(|d| d.insert_persisted(text_id, new_key.clone()));

                        if ui.button("Add").clicked() {
                            let mut new_field = if template {
                                children
                                    .get_index(0)
                                    .map(|(_, f)| f.clone())
                                    .unwrap_or_else(Field::default_string)
                            } else {
                                match selected {
                                    FieldType::String => Field::default_string(),
                                    FieldType::Integer => Field::default_integer(),
                                    FieldType::Float => Field::default_float(),
                                    FieldType::Boolean => Field::default_boolean(),
                                    FieldType::Image => Field::default_image(),
                                    FieldType::Table => Field::default_table(),
                                    FieldType::Array => Field::default_array(),
                                }
                            };
                            new_field.label = Some(new_key.clone());
                            children.insert(new_key.clone(), new_field);
                            ui.data_mut(|d| d.insert_persisted(text_id, String::from("value")));
                        }
                    });
                }
            },
        );
    }

    fn render_image(
        ui: &mut Ui,
        value: &mut String,
        texture: &mut Option<egui::TextureHandle>,
        label: &str,
        fonts: FormFonts,
    ) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                if ui.button("Add texture").clicked() {
                    Self::handle_image_upload(ui, value, texture);
                }

                if !value.is_empty() {
                    ui.label(egui::RichText::new(label).font(fonts.text));
                }
            });

            if let Some(tex) = texture.as_ref() {
                ui.add_space(2.0);
                ui.add(egui::Image::new(tex).max_width(128.0));
            }
        });
    }

    fn handle_image_upload(
        ui: &mut Ui,
        value: &mut String,
        texture: &mut Option<egui::TextureHandle>,
    ) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Images", &["png"])
            .pick_file()
        {
            *value = path.to_string_lossy().to_string();

            if let Ok(img) = image::open(&path) {
                let mut img = img.to_rgba8();
                let (width, height) = (img.width(), img.height());
                let min_size = 128;

                if width < min_size || height < min_size {
                    let scale_w = min_size.max(width);
                    let scale_h = min_size.max(height);
                    img = image::imageops::resize(&img, scale_w, scale_h, FilterType::Nearest);
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

    fn render_description(ui: &mut Ui, description: &Option<String>, font: &FontId) {
        if let Some(desc) = description.as_deref() {
            ui.add_space(1.0);
            ui.label(
                egui::RichText::new(desc)
                    .font(font.clone())
                    .color(egui::Color32::GRAY),
            );
        }
    }

    // ==============================================
    // ARRAY RENDERING
    // ==============================================

    fn show_array(
        ui: &mut Ui,
        value: &mut ArrayValue,
        key: &str,
        label: Option<&str>,
        description: Option<&str>,
        fonts: FormFonts,
        editable: bool,
        template: bool,
        indent: f32,
        level: usize,
        form_config: &FormConfig,
    ) {
        ui.add_space(4.0);
        ui.label(egui::RichText::new(label.unwrap_or(key)).font(fonts.label.clone()));

        if let Some(desc) = description {
            ui.add_space(2.0);
            ui.label(
                egui::RichText::new(desc)
                    .font(fonts.description.clone())
                    .color(egui::Color32::GRAY),
            );
        }

        ui.collapsing(key, |ui| match value {
            ArrayValue::Strings { items } => {
                Self::render_array_strings(ui, items, fonts, editable, indent);
            }
            ArrayValue::Floats { items } => {
                Self::render_array_floats(ui, items, fonts, editable, indent);
            }

            ArrayValue::Integers { items } => {
                Self::render_array_integers(ui, items, fonts, editable, indent);
            }
            ArrayValue::Objects { items } => {
                Self::render_array_objects(ui, items, level, form_config, fonts, editable, indent);
            }
            ArrayValue::Mixed { items } => {
                Self::render_array_mixed(ui, items, fonts, indent);
            }
            ArrayValue::Enums { items } => {
                Self::render_array_enums(ui, items, fonts, editable, template, indent);
            }
        });

        ui.add_space(4.0);
    }

    fn render_array_strings(
        ui: &mut Ui,
        strings: &mut Vec<String>,
        fonts: FormFonts,
        editable: bool,
        indent: f32,
    ) {
        let mut remove_index: Option<usize> = None;

        for (i, s) in strings.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.add_space(indent);
                ui.label(egui::RichText::new(format!("[{}]", i)).font(fonts.label.clone()));
                ui.add(
                    egui::TextEdit::singleline(s)
                        .font(fonts.text.clone())
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

    fn render_array_floats(
        ui: &mut Ui,
        numbers: &mut Vec<f64>,
        fonts: FormFonts,
        editable: bool,
        indent: f32,
    ) {
        let mut remove_index: Option<usize> = None;

        for (i, n) in numbers.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.add_space(indent);
                ui.label(egui::RichText::new(format!("[{}]", i)).font(fonts.label.clone()));
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

    fn render_array_integers(
        ui: &mut Ui,
        numbers: &mut Vec<i64>,
        fonts: FormFonts,
        editable: bool,
        indent: f32,
    ) {
        let mut remove_index: Option<usize> = None;

        for (i, n) in numbers.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.add_space(indent);
                ui.label(egui::RichText::new(format!("[{}]", i)).font(fonts.label.clone()));
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
            numbers.push(0);
        }
    }

    fn render_array_objects(
        ui: &mut Ui,
        objects: &mut Vec<IndexMap<String, Field>>,
        level: usize,
        form_config: &FormConfig,
        fonts: FormFonts,
        editable: bool,
        indent: f32,
    ) {
        let mut remove_index: Option<usize> = None;

        for (i, obj) in objects.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.add_space(indent);
                ui.label(egui::RichText::new(format!("Item {}", i)).font(fonts.label.clone()));
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

    fn render_array_mixed(ui: &mut Ui, values: &mut Vec<Value>, fonts: FormFonts, indent: f32) {
        for (i, v) in values.iter().enumerate() {
            ui.horizontal(|ui| {
                ui.add_space(indent);
                ui.label(
                    egui::RichText::new(format!("[{}]: {}", i, v))
                        .font(fonts.text.clone())
                        .color(egui::Color32::LIGHT_GRAY),
                );
            });
        }
    }

    fn render_array_enums(
        ui: &mut Ui,
        enums: &mut Vec<EnumField>,
        fonts: FormFonts,
        editable: bool,
        template: bool,
        indent: f32,
    ) {
        let mut remove_index: Option<usize> = None;

        for (i, enum_field) in enums.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.add_space(indent);
                ui.label(egui::RichText::new(format!("[{}]", i)).font(fonts.label.clone()));

                if !enum_field.options.is_empty() && !enum_field.options.contains(&enum_field.value)
                {
                    enum_field.value = enum_field.options[0].clone();
                }

                let combo_id =
                    ui.make_persistent_id(format!("array_enum_{}_{}", i, enum_field.value));

                egui::ComboBox::from_id_salt(combo_id)
                    .selected_text(if enum_field.value.is_empty() {
                        "Select..."
                    } else {
                        enum_field.value.as_str()
                    })
                    .show_ui(ui, |ui| {
                        for option in enum_field.options.iter() {
                            ui.selectable_value(
                                &mut enum_field.value,
                                option.clone(),
                                option.as_str(),
                            );
                        }
                    });

                if editable && ui.button("X").clicked() {
                    remove_index = Some(i);
                }
            });
        }

        if let Some(i) = remove_index {
            enums.remove(i);
        }

        if editable && ui.button("Add").clicked() {
            let new_enum = if template && !enums.is_empty() {
                enums[0].clone()
            } else {
                EnumField {
                    value: String::new(),
                    options: Vec::new(),
                }
            };
            enums.push(new_enum);
        }
    }
}

// ==============================================
// HELPER STRUCTURES
// ==============================================

#[derive(Clone)]
struct FormFonts {
    header: FontId,
    label: FontId,
    text: FontId,
    description: FontId,
}

impl FormFonts {
    fn from_config(config: &FormConfig) -> Self {
        Self {
            header: FontId::proportional(config.font_size_header),
            label: FontId::proportional(config.font_size_label),
            text: FontId::monospace(config.font_size_text),
            description: FontId::monospace(config.font_size_description),
        }
    }
}
