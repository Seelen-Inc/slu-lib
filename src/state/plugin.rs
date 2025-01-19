use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::{ResourceId, ResourceMetadata, WidgetId};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
pub struct PluginId(pub ResourceId);

impl From<ResourceId> for PluginId {
    fn from(value: ResourceId) -> Self {
        PluginId(value)
    }
}

impl std::fmt::Display for PluginId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Plugin {
    pub id: PluginId,
    #[serde(default = "Plugin::default_icon")]
    pub icon: String,
    pub target: WidgetId,
    pub metadata: ResourceMetadata,
    pub plugin: serde_json::Value,
    #[serde(skip_deserializing)]
    pub bundled: bool,
}

impl Plugin {
    pub fn default_icon() -> String {
        "PiPuzzlePieceDuotone".to_string()
    }
}
