mod icon_pack;
mod placeholder;
mod plugin;
mod profile;
mod settings;
mod settings_by_app;
mod settings_by_monitor;
mod theme;
mod weg_items;
mod widget;
mod wm_layout;

pub use icon_pack::*;
pub use placeholder::*;
pub use plugin::*;
pub use profile::*;
pub use settings::*;
pub use settings_by_app::*;
pub use settings_by_monitor::*;
pub use theme::*;
pub use weg_items::*;
pub use widget::*;
pub use wm_layout::*;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ResourceId(String);

impl ResourceId {
    pub fn is_valid(&self) -> bool {
        let regex =
            regex::Regex::new("^@[a-zA-Z0-9_\\-]+\\/[a-zA-Z0-9_\\-]+$").expect("Invalid regex");
        regex.is_match(&self.0)
    }

    /// Creator username of the resource
    ///
    /// # Safety
    ///
    /// The string is a valid resource id
    pub fn creator(&self) -> String {
        self.0.split('/').next().unwrap().to_string()
    }
}

impl From<String> for ResourceId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for ResourceId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl std::fmt::Display for ResourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(default, rename_all = "camelCase")]
pub struct ResourceMetadata {
    pub display_name: String,
    pub author: String,
    pub description: String,
    pub filename: String,
    pub tags: Vec<String>,
}

impl Default for ResourceMetadata {
    fn default() -> Self {
        Self {
            display_name: "Unknown".to_string(),
            author: "Unknown".to_string(),
            description: String::new(),
            filename: String::new(),
            tags: Vec::new(),
        }
    }
}
