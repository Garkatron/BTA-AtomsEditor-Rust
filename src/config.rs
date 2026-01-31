use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub font_size_header: f32,
    pub font_size_label: f32,
    pub font_size_text: f32,
}
