#![warn(clippy::all, rust_2018_idioms)]

pub mod editor;
pub mod selector;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub path: String,
}
