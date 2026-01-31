use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

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
        children: IndexMap<String, Field>,
    },
    #[serde(rename = "array")]
    Array {
        #[serde(default)]
        value: ArrayValue,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ArrayValue {
    Strings(Vec<String>),
    Numbers(Vec<f64>),
    Objects(Vec<IndexMap<String, Field>>),
    Mixed(Vec<toml::Value>),
}

#[derive(Serialize, Deserialize, Debug)]
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
