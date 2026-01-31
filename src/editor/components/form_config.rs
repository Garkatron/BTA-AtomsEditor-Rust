use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FormConfig {
    pub font_size_header: f32,
    pub font_size_label: f32,
    pub font_size_description: f32,
    pub font_size_text: f32,
}

impl Default for FormConfig {
    fn default() -> Self {
        Self {
            font_size_header: 22.0,
            font_size_label: 18.0,
            font_size_description: 16.0,
            font_size_text: 15.0,
        }
    }
}
