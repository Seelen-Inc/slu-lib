use std::ops::{Deref, DerefMut};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::resource::{ResourceId, ResourceMetadata};

use super::WidgetId;

#[derive(Debug, Clone, Hash, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
pub struct PluginId(ResourceId);

impl Deref for PluginId {
    type Target = ResourceId;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PluginId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<&str> for PluginId {
    fn from(value: &str) -> Self {
        Self(ResourceId::from(value))
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export)]
pub struct Plugin {
    pub id: PluginId,
    pub metadata: ResourceMetadata,
    #[serde(default = "Plugin::default_icon")]
    pub icon: String,
    pub target: WidgetId,
    pub plugin: serde_json::Value,
}

impl Plugin {
    pub fn default_icon() -> String {
        "PiPuzzlePieceDuotone".to_string()
    }
}
