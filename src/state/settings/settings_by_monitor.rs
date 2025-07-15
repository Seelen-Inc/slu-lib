use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

use crate::{
    rect::Rect,
    resource::{WallpaperId, WidgetId},
};

use super::{ThirdPartyWidgetSettings, WegPinnedItemsVisibility, WegTemporalItemsVisibility};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export)]
pub struct FancyToolbarSettingsByMonitor {
    pub enabled: bool,
}

impl Default for FancyToolbarSettingsByMonitor {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export)]
pub struct SeelenWegSettingsByMonitor {
    pub enabled: bool,
    pub temporal_items_visibility: Option<WegTemporalItemsVisibility>,
    pub pinned_items_visibility: Option<WegPinnedItemsVisibility>,
}

impl Default for SeelenWegSettingsByMonitor {
    fn default() -> Self {
        Self {
            enabled: true,
            temporal_items_visibility: None,
            pinned_items_visibility: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export)]
pub struct WindowManagerSettingsByMonitor {
    pub enabled: bool,
    pub padding: Option<u32>,
    pub margin: Option<Rect>,
    pub gap: Option<u32>,
    pub layout: Option<String>,
}

impl Default for WindowManagerSettingsByMonitor {
    fn default() -> Self {
        Self {
            enabled: true,
            padding: None,
            margin: None,
            gap: None,
            layout: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export)]
pub struct SeelenWallSettingsByMonitor {
    pub enabled: bool,
    pub backgrounds: Option<Vec<WallpaperId>>,
}

impl Default for SeelenWallSettingsByMonitor {
    fn default() -> Self {
        Self {
            enabled: true,
            backgrounds: None,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema, TS)]
pub struct MonitorSettingsByWidget(HashMap<WidgetId, ThirdPartyWidgetSettings>);

impl MonitorSettingsByWidget {
    pub fn is_widget_enabled(&self, widget_id: &WidgetId) -> bool {
        self.0
            .get(widget_id)
            .is_none_or(|settings| settings.enabled)
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema, TS)]
#[serde(default, rename_all = "camelCase")]
pub struct MonitorConfiguration {
    /// dictionary of settings by widget
    pub by_widget: MonitorSettingsByWidget,
}
