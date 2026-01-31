use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Field {
    pub label: Option<String>,
    pub description: Option<String>,
    pub editable: Option<bool>,
    pub template: Option<bool>,
    #[serde(flatten)]
    pub value: FieldValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FieldType {
    String,
    Integer,
    Float,
    Boolean,
    Image,
    Table,
    Array,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum FieldValue {
    #[serde(rename = "string")]
    String {
        value: String,
        default: Option<String>,
    },
    #[serde(rename = "image")]
    Image {
        value: String,
        #[serde(skip)]
        texture: Option<egui::TextureHandle>,
    },
    #[serde(rename = "float")]
    Float { value: f64, default: Option<f64> },
    #[serde(rename = "integer")]
    Integer { value: i64, default: Option<i64> },
    #[serde(rename = "boolean")]
    Boolean { value: bool, default: Option<bool> },
    #[serde(rename = "table")]
    Table {
        #[serde(flatten)]
        children: IndexMap<String, Field>,
    },
    #[serde(rename = "array")]
    Array {
        #[serde(default)]
        value: ArrayValue,
    },
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ArrayValue {
    Strings(Vec<String>),
    Numbers(Vec<f64>),
    Objects(Vec<IndexMap<String, Field>>),
    Mixed(Vec<toml::Value>),
}

#[derive(Serialize, Deserialize)]
pub struct Document {
    #[serde(flatten)]
    pub fields: IndexMap<String, Field>,
}

impl Document {
    pub fn new(fields: IndexMap<String, Field>) -> Self {
        Self { fields }
    }
    pub fn from_toml(toml_str: &str) -> Result<Self, toml::de::Error> {
        let doc: Document = toml::from_str(toml_str)?;
        Ok(doc)
    }
}

impl Default for ArrayValue {
    fn default() -> Self {
        ArrayValue::Strings(Vec::new())
    }
}

impl Field {
    pub fn default_string() -> Self {
        Self {
            label: None,
            description: None,
            editable: Some(true),
            template: Some(false),
            value: FieldValue::String {
                value: String::new(),
                default: None,
            },
        }
    }

    pub fn default_float() -> Self {
        Self {
            label: None,
            description: None,
            editable: Some(true),
            value: FieldValue::Float {
                value: 0.0,
                default: None,
            },
            template: Some(false),
        }
    }

    pub fn default_integer() -> Self {
        Self {
            label: None,
            description: None,
            editable: Some(true),
            value: FieldValue::Integer {
                value: 0,
                default: None,
            },
            template: Some(false),
        }
    }

    pub fn default_boolean() -> Self {
        Self {
            label: None,
            description: None,
            editable: Some(true),
            value: FieldValue::Boolean {
                value: false,
                default: None,
            },
            template: Some(false),
        }
    }

    pub fn default_image() -> Self {
        Self {
            label: None,
            description: None,
            editable: Some(true),
            value: FieldValue::Image {
                value: String::new(),
                texture: None,
            },
            template: Some(false),
        }
    }

    pub fn default_table() -> Self {
        Self {
            label: None,
            description: None,
            editable: Some(true),
            value: FieldValue::Table {
                children: IndexMap::new(),
            },
            template: Some(false),
        }
    }

    pub fn default_array() -> Self {
        Self {
            label: None,
            description: None,
            editable: Some(true),
            value: FieldValue::Array {
                value: ArrayValue::default(),
            },
            template: Some(false),
        }
    }
}
