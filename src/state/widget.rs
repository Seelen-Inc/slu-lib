use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::{declaration::WidgetSettingsDeclarationList, ResourceId};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct WidgetId(pub ResourceId);

impl WidgetId {
    pub fn known_weg() -> Self {
        Self(ResourceId("@seelen/weg".to_string()))
    }
    pub fn known_toolbar() -> Self {
        Self(ResourceId("@seelen/fancy-toolbar".to_string()))
    }
    pub fn known_wm() -> Self {
        Self(ResourceId("@seelen/window-manager".to_string()))
    }
    pub fn known_launcher() -> Self {
        Self(ResourceId("@seelen/launcher".to_string()))
    }
    pub fn known_wall() -> Self {
        Self(ResourceId("@seelen/wallpaper-manager".to_string()))
    }
}

impl From<ResourceId> for WidgetId {
    fn from(value: ResourceId) -> Self {
        Self(value)
    }
}

impl From<&str> for WidgetId {
    fn from(value: &str) -> Self {
        Self(ResourceId::from(value))
    }
}

impl std::fmt::Display for WidgetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Widget {
    pub id: WidgetId,
    #[serde(default)]
    pub settings: WidgetSettingsDeclarationList,
    pub js: Option<String>,
    pub css: Option<String>,
    pub html: Option<String>,
}
