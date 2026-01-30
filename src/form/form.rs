use std::collections::HashMap;

use serde::{Deserialize, Serialize, de::Error};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Field {
    pub label: Option<String>,
    pub description: Option<String>,
    #[serde(flatten)]
    pub value: FieldValue,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum FieldValue {
    #[serde(rename = "string")]
    String {
        value: String,
        default: Option<String>,
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
        children: HashMap<String, Field>,
    },
    #[serde(rename = "array")]
    Array { items: Vec<HashMap<String, Field>> },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    #[serde(flatten)]
    pub fields: HashMap<String, Field>,
}

impl Document {
    pub fn new(fields: HashMap<String, Field>) -> Self {
        Self { fields }
    }
    pub fn from_toml(toml_str: &str) -> Result<Self, toml::de::Error> {
        let doc: Document = toml::from_str(toml_str)?;
        Ok(doc)
    }
}
