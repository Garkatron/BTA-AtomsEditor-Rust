use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Field {
    pub label: Option<String>,
    pub description: Option<String>,
    pub editable: Option<bool>,
    pub template: Option<bool>,
    pub convert: Option<FieldType>,
    #[serde(flatten)]
    pub value: FieldValue,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct EnumField {
    pub value: String,
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
        #[serde(flatten)]
        value: ArrayValue,
    },
    #[serde(rename = "enum")]
    Enum {
        #[serde(default)]
        value: String,
        #[serde(default)]
        options: Vec<String>,
    },
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "array_type")]
pub enum ArrayValue {
    #[serde(rename = "enums")]
    Enums { items: Vec<EnumField> },

    #[serde(rename = "strings")]
    Strings { items: Vec<String> },

    #[serde(rename = "floats")]
    Floats { items: Vec<f64> },

    #[serde(rename = "integers")]
    Integers { items: Vec<i64> },

    #[serde(rename = "objects")]
    Objects { items: Vec<IndexMap<String, Field>> },

    #[serde(rename = "mixed")]
    Mixed { items: Vec<toml::Value> },
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
        ArrayValue::Strings { items: Vec::new() }
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
            convert: None,
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
            convert: None,
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
            convert: None,
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
            convert: None,
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
            convert: None,
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
            convert: None,
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
            convert: None,
        }
    }

    pub fn default_enum() -> Self {
        Self {
            label: None,
            description: None,
            editable: Some(true),
            value: FieldValue::Enum {
                value: String::new(),
                options: Vec::new(),
            },
            template: Some(false),
            convert: None,
        }
    }
}
