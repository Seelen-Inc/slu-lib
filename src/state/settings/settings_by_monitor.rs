use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::rect::Rect;

use super::{
    by_widget::SettingsByWidget, SeelenWallWallpaper, WegPinnedItemsVisibility,
    WegTemporalItemsVisibility, WidgetId,
};

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
    pub backgrounds: Option<Vec<SeelenWallWallpaper>>,
}

impl Default for SeelenWallSettingsByMonitor {
    fn default() -> Self {
        Self {
            enabled: true,
            backgrounds: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, TS)]
#[ts(export)]
pub enum WorkspaceIdentifierType {
    #[serde(alias = "name")]
    Name,
    #[serde(alias = "index")]
    Index,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct WorkspaceIdentifier {
    pub id: String,
    pub kind: WorkspaceIdentifierType,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct WorkspaceConfiguration {
    pub identifier: WorkspaceIdentifier,
    pub layout: Option<String>,
    pub backgrounds: Option<Vec<SeelenWallWallpaper>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, TS)]
#[serde(default, rename_all = "camelCase")]
pub struct MonitorConfiguration {
    /// @deprecated since v2.1.0, will be removed in v3.0.0
    #[ts(skip)]
    #[serde(skip_serializing)]
    tb: Option<FancyToolbarSettingsByMonitor>,
    /// @deprecated since v2.1.0, will be removed in v3.0.0
    #[ts(skip)]
    #[serde(skip_serializing)]
    weg: Option<SeelenWegSettingsByMonitor>,
    /// @deprecated since v2.1.0, will be removed in v3.0.0
    #[ts(skip)]
    #[serde(skip_serializing)]
    wm: Option<WindowManagerSettingsByMonitor>,
    /// @deprecated since v2.1.0, will be removed in v3.0.0
    #[ts(skip)]
    #[serde(skip_serializing)]
    wall: Option<SeelenWallSettingsByMonitor>,
    // ---
    /// dictionary of settings by widget
    pub by_widget: SettingsByWidget,
    /// list of settings by workspace on this monitor
    pub workspaces_v2: Vec<WorkspaceConfiguration>,
}

impl Default for MonitorConfiguration {
    fn default() -> Self {
        let mut settings_by_widget = SettingsByWidget::default();
        settings_by_widget
            .set_config(
                WidgetId::known_toolbar(),
                FancyToolbarSettingsByMonitor::default(),
            )
            .unwrap();
        settings_by_widget
            .set_config(WidgetId::known_weg(), SeelenWegSettingsByMonitor::default())
            .unwrap();
        settings_by_widget
            .set_config(
                WidgetId::known_wm(),
                WindowManagerSettingsByMonitor::default(),
            )
            .unwrap();
        settings_by_widget
            .set_config(
                WidgetId::known_wall(),
                SeelenWallSettingsByMonitor::default(),
            )
            .unwrap();

        Self {
            tb: None,
            weg: None,
            wm: None,
            wall: None,
            by_widget: settings_by_widget,
            workspaces_v2: Vec::new(),
        }
    }
}

impl MonitorConfiguration {
    pub fn tb(&self) -> FancyToolbarSettingsByMonitor {
        self.by_widget.get_config(&WidgetId::known_toolbar())
    }

    pub fn weg(&self) -> SeelenWegSettingsByMonitor {
        self.by_widget.get_config(&WidgetId::known_weg())
    }

    pub fn wm(&self) -> WindowManagerSettingsByMonitor {
        self.by_widget.get_config(&WidgetId::known_wm())
    }

    pub fn wall(&self) -> SeelenWallSettingsByMonitor {
        self.by_widget.get_config(&WidgetId::known_wall())
    }

    /// Migrate old settings (before v2.1.0) (will be removed in v3.0.0)
    pub fn migrate(&mut self) -> Result<(), serde_json::Error> {
        let dict = &mut self.by_widget;
        if let Some(tb) = self.tb.take() {
            dict.set_config(WidgetId::known_toolbar(), tb)?;
        }
        if let Some(weg) = self.weg.take() {
            dict.set_config(WidgetId::known_weg(), weg)?;
        }
        if let Some(wm) = self.wm.take() {
            dict.set_config(WidgetId::known_wm(), wm)?;
        }
        if let Some(wall) = self.wall.take() {
            dict.set_config(WidgetId::known_wall(), wall)?;
        }
        Ok(())
    }

    pub fn sanitize(&mut self) {
        self.by_widget.sanitize();
    }
}
