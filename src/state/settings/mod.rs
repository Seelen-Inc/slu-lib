/* In this file we use #[serde_alias(SnakeCase)] as backward compatibility from versions below v1.9.8 */
pub mod by_widget;
pub mod declaration;
pub mod settings_by_monitor;

use by_widget::SettingsByWidget;
pub use settings_by_monitor::*;

use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_alias::serde_alias;
use ts_rs::TS;

use crate::rect::Rect;

use super::{PluginId, WidgetId};

// ============== Fancy Toolbar Settings ==============

#[serde_alias(SnakeCase)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export)]
pub struct FancyToolbarSettings {
    /// enable or disable the fancy toolbar
    pub enabled: bool,
    /// height of the fancy toolbar
    pub height: u32,
    /// hide mode
    pub hide_mode: HideMode,
    /// enables the logic which persists last overlapped HWND to enchance multimonitor overlap feature
    pub use_multi_monitor_overlap_logic: bool,
    /// delay to show the toolbar on Mouse Hover in milliseconds
    pub delay_to_show: u32,
    /// delay to hide the toolbar on Mouse Leave in milliseconds
    pub delay_to_hide: u32,
}

impl Default for FancyToolbarSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            height: 30,
            hide_mode: HideMode::Never,
            use_multi_monitor_overlap_logic: false,
            delay_to_show: 100,
            delay_to_hide: 800,
        }
    }
}

// ============== SeelenWeg Settings ==============

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
pub enum SeelenWegMode {
    #[serde(alias = "Full-Width")]
    FullWidth,
    #[serde(alias = "Min-Content")]
    MinContent,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
pub enum WegTemporalItemsVisibility {
    All,
    OnMonitor,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
pub enum WegPinnedItemsVisibility {
    Always,
    WhenPrimary,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
pub enum HideMode {
    /// never hide
    Never,
    /// auto-hide always on
    Always,
    /// auto-hide only if is overlaped by the focused window
    #[serde(alias = "On-Overlap")]
    OnOverlap,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
pub enum SeelenWegSide {
    Left,
    Right,
    Top,
    Bottom,
}

#[serde_alias(SnakeCase)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export)]
pub struct SeelenWegSettings {
    /// enable or disable the seelenweg
    pub enabled: bool,
    /// Dock/Taskbar mode
    pub mode: SeelenWegMode,
    /// When to hide the dock
    pub hide_mode: HideMode,
    /// Which temporal items to show on the dock instance (this can be overridden per monitor)
    pub temporal_items_visibility: WegTemporalItemsVisibility,
    /// Determines is the pinned item should be shown or not (this can be overridden per monitor).
    pub pinned_items_visibility: WegPinnedItemsVisibility,
    /// Dock position
    pub position: SeelenWegSide,
    /// enables the logic which persists last overlapped HWND to enchance multimonitor overlap feature
    pub use_multi_monitor_overlap_logic: bool,
    /// enable or disable separators visibility
    pub visible_separators: bool,
    /// item size in px
    pub size: u32,
    /// zoomed item size in px
    pub zoom_size: u32,
    /// Dock/Taskbar margin in px
    pub margin: u32,
    /// Dock/Taskbar padding in px
    pub padding: u32,
    /// space between items in px
    pub space_between_items: u32,
    /// delay to show the toolbar on Mouse Hover in milliseconds
    pub delay_to_show: u32,
    /// delay to hide the toolbar on Mouse Leave in milliseconds
    pub delay_to_hide: u32,
}

impl Default for SeelenWegSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            mode: SeelenWegMode::MinContent,
            hide_mode: HideMode::OnOverlap,
            position: SeelenWegSide::Bottom,
            use_multi_monitor_overlap_logic: false,
            visible_separators: true,
            temporal_items_visibility: WegTemporalItemsVisibility::All,
            pinned_items_visibility: WegPinnedItemsVisibility::Always,
            size: 40,
            zoom_size: 70,
            margin: 8,
            padding: 8,
            space_between_items: 8,
            delay_to_show: 100,
            delay_to_hide: 800,
        }
    }
}

impl SeelenWegSettings {
    /// total height or width of the dock, depending on the Position
    pub fn total_size(&self) -> u32 {
        self.size + (self.padding * 2) + (self.margin * 2)
    }
}

// ============== Window Manager Settings ==============

#[serde_alias(SnakeCase)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
#[serde(default, rename_all = "camelCase")]
pub struct Border {
    pub enabled: bool,
    pub width: f64,
    pub offset: f64,
}

#[serde_alias(SnakeCase)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
#[serde(default, rename_all = "camelCase")]
pub struct FloatingWindowSettings {
    pub width: f64,
    pub height: f64,
}

#[serde_alias(SnakeCase)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export)]
pub struct WindowManagerSettings {
    /// enable or disable the window manager
    pub enabled: bool,
    /// enable or disable auto stacking by category
    pub auto_stacking_by_category: bool,
    /// window manager border
    pub border: Border,
    /// the resize size in % to be used when resizing via cli
    pub resize_delta: f32,
    /// default gap between containers
    pub workspace_gap: u32,
    /// default workspace padding
    pub workspace_padding: u32,
    /// default workspace margin
    #[serde(alias = "global_work_area_offset")]
    pub workspace_margin: Rect,
    /// floating window settings
    pub floating: FloatingWindowSettings,
    /// default layout
    pub default_layout: PluginId,
}

impl Default for Border {
    fn default() -> Self {
        Self {
            enabled: true,
            width: 3.0,
            offset: 0.0,
        }
    }
}

impl Default for FloatingWindowSettings {
    fn default() -> Self {
        Self {
            width: 800.0,
            height: 500.0,
        }
    }
}

impl Default for WindowManagerSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            auto_stacking_by_category: true,
            border: Border::default(),
            resize_delta: 10.0,
            workspace_gap: 10,
            workspace_padding: 10,
            workspace_margin: Rect::default(),
            floating: FloatingWindowSettings::default(),
            default_layout: "@default/wm-bspwm".into(),
        }
    }
}

// ================= Seelen Launcher ================

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
pub enum SeelenLauncherMonitor {
    Primary,
    #[serde(alias = "Mouse-Over")]
    MouseOver,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, TS)]
#[serde(default, rename_all = "camelCase")]
pub struct SeelenLauncherRunner {
    pub id: String,
    pub label: String,
    pub program: String,
    pub readonly: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export)]
pub struct SeelenLauncherSettings {
    pub enabled: bool,
    pub monitor: SeelenLauncherMonitor,
    pub runners: Vec<SeelenLauncherRunner>,
}

impl Default for SeelenLauncherSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            monitor: SeelenLauncherMonitor::MouseOver,
            runners: vec![
                SeelenLauncherRunner {
                    id: "RUN".to_owned(),
                    label: "t:app_launcher.runners.explorer".to_owned(),
                    program: "explorer.exe".to_owned(),
                    readonly: true,
                },
                SeelenLauncherRunner {
                    id: "CMD".to_owned(),
                    label: "t:app_launcher.runners.cmd".to_owned(),
                    program: "cmd.exe".to_owned(),
                    readonly: true,
                },
            ],
        }
    }
}

impl SeelenLauncherSettings {
    pub fn sanitize(&mut self) {
        let mut dict = HashSet::new();
        self.runners
            .retain(|runner| !runner.program.is_empty() && dict.insert(runner.program.clone()));
        for runner in &mut self.runners {
            if runner.id.is_empty() {
                runner.id = uuid::Uuid::new_v4().to_string();
            }
        }
    }
}

// ================= Seelen Wall ================

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
pub struct SeelenWallWallpaper {
    pub id: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export)]
pub struct SeelenWallSettings {
    pub enabled: bool,
    pub backgrounds: Vec<SeelenWallWallpaper>,
    /// update interval in seconds
    pub interval: u32,
    pub randomize: bool,
}

impl Default for SeelenWallSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            backgrounds: vec![],
            interval: 60,
            randomize: false,
        }
    }
}

impl SeelenWallSettings {
    pub fn sanitize(&mut self) {
        self.backgrounds.retain(|b| b.path.exists());
    }
}

// ============== Ahk Variables ==============

#[macro_export]
macro_rules! define_struct_and_hashmap {
    (
        $($field:ident),*
    ) => {
        #[serde_alias(SnakeCase)]
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
        #[serde(default, rename_all = "camelCase")]
        pub struct AhkVarList {
            $(
                pub $field: AhkVar,
            )*
        }

        impl AhkVarList {
            pub fn as_hash_map(&self) -> HashMap<String, AhkVar> {
                let mut map = HashMap::new();
                $(
                    map.insert(
                        stringify!($field).to_string(),
                        self.$field.clone()
                    );
                )*
                map
            }
        }
    };
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
pub struct AhkVar {
    pub fancy: String,
    pub ahk: String,
    #[serde(default)]
    pub readonly: bool,
    #[serde(default)]
    pub devonly: bool,
}

impl AhkVar {
    pub fn new(f: &str, ahk: &str) -> Self {
        Self {
            fancy: f.to_string(),
            ahk: ahk.to_string(),
            readonly: false,
            devonly: false,
        }
    }

    pub fn readonly(mut self) -> Self {
        self.readonly = true;
        self
    }

    pub fn devonly(mut self) -> Self {
        self.devonly = true;
        self
    }
}

define_struct_and_hashmap![
    toggle_launcher,
    reserve_top,
    reserve_bottom,
    reserve_left,
    reserve_right,
    reserve_float,
    reserve_stack,
    focus_top,
    focus_bottom,
    focus_left,
    focus_right,
    focus_latest,
    increase_width,
    decrease_width,
    increase_height,
    decrease_height,
    restore_sizes,
    start_weg_app_0,
    start_weg_app_1,
    start_weg_app_2,
    start_weg_app_3,
    start_weg_app_4,
    start_weg_app_5,
    start_weg_app_6,
    start_weg_app_7,
    start_weg_app_8,
    start_weg_app_9,
    switch_workspace_0,
    switch_workspace_1,
    switch_workspace_2,
    switch_workspace_3,
    switch_workspace_4,
    switch_workspace_5,
    switch_workspace_6,
    switch_workspace_7,
    switch_workspace_8,
    switch_workspace_9,
    move_to_workspace_0,
    move_to_workspace_1,
    move_to_workspace_2,
    move_to_workspace_3,
    move_to_workspace_4,
    move_to_workspace_5,
    move_to_workspace_6,
    move_to_workspace_7,
    move_to_workspace_8,
    move_to_workspace_9,
    send_to_workspace_0,
    send_to_workspace_1,
    send_to_workspace_2,
    send_to_workspace_3,
    send_to_workspace_4,
    send_to_workspace_5,
    send_to_workspace_6,
    send_to_workspace_7,
    send_to_workspace_8,
    send_to_workspace_9,
    misc_open_settings,
    misc_toggle_lock_tracing,
    misc_toggle_win_event_tracing
];

impl Default for AhkVarList {
    fn default() -> Self {
        // some aproaches taken in care to write default values
        // Alt -> when modifying the current window/workspace/monitor
        // Shift -> when sending/moving something of place
        // Ctrl -> alternative to already used keys
        Self {
            // launcher
            toggle_launcher: AhkVar::new("Win + S", "#s").readonly(),
            // wm
            reserve_top: AhkVar::new("Win + Shift + I", "#+i"),
            reserve_bottom: AhkVar::new("Win + Shift + K", "#+k"),
            reserve_left: AhkVar::new("Win + Shift + J", "#+j"),
            reserve_right: AhkVar::new("Win + Shift + L", "#+l"),
            reserve_float: AhkVar::new("Win + Shift + U", "#+u"),
            reserve_stack: AhkVar::new("Win + Shift + O", "#+o"),

            focus_top: AhkVar::new("Win + Alt + I", "#!i"),
            focus_bottom: AhkVar::new("Win + Alt + K", "#!k"),
            focus_left: AhkVar::new("Win + Alt + J", "#!j"),
            focus_right: AhkVar::new("Win + Alt + L", "#!l"),
            focus_latest: AhkVar::new("Win + Alt + U", "#!u"),

            increase_width: AhkVar::new("Win + Alt + =", "#!="),
            decrease_width: AhkVar::new("Win + Alt + -", "#!-"),
            increase_height: AhkVar::new("Ctrl + Win + Alt + =", "^#!="),
            decrease_height: AhkVar::new("Ctrl + Win + Alt + -", "^#!-"),
            restore_sizes: AhkVar::new("Win + Alt + 0", "#!0"),
            // weg
            start_weg_app_0: AhkVar::new("Win + 1", "#1"),
            start_weg_app_1: AhkVar::new("Win + 2", "#2"),
            start_weg_app_2: AhkVar::new("Win + 3", "#3"),
            start_weg_app_3: AhkVar::new("Win + 4", "#4"),
            start_weg_app_4: AhkVar::new("Win + 5", "#5"),
            start_weg_app_5: AhkVar::new("Win + 6", "#6"),
            start_weg_app_6: AhkVar::new("Win + 7", "#7"),
            start_weg_app_7: AhkVar::new("Win + 8", "#8"),
            start_weg_app_8: AhkVar::new("Win + 9", "#9"),
            start_weg_app_9: AhkVar::new("Win + 0", "#0"),
            // virtual desktops
            switch_workspace_0: AhkVar::new("Alt + 1", "!1"),
            switch_workspace_1: AhkVar::new("Alt + 2", "!2"),
            switch_workspace_2: AhkVar::new("Alt + 3", "!3"),
            switch_workspace_3: AhkVar::new("Alt + 4", "!4"),
            switch_workspace_4: AhkVar::new("Alt + 5", "!5"),
            switch_workspace_5: AhkVar::new("Alt + 6", "!6"),
            switch_workspace_6: AhkVar::new("Alt + 7", "!7"),
            switch_workspace_7: AhkVar::new("Alt + 8", "!8"),
            switch_workspace_8: AhkVar::new("Alt + 9", "!9"),
            switch_workspace_9: AhkVar::new("Alt + 0", "!0"),
            move_to_workspace_0: AhkVar::new("Alt + Shift + 1", "!+1"),
            move_to_workspace_1: AhkVar::new("Alt + Shift + 2", "!+2"),
            move_to_workspace_2: AhkVar::new("Alt + Shift + 3", "!+3"),
            move_to_workspace_3: AhkVar::new("Alt + Shift + 4", "!+4"),
            move_to_workspace_4: AhkVar::new("Alt + Shift + 5", "!+5"),
            move_to_workspace_5: AhkVar::new("Alt + Shift + 6", "!+6"),
            move_to_workspace_6: AhkVar::new("Alt + Shift + 7", "!+7"),
            move_to_workspace_7: AhkVar::new("Alt + Shift + 8", "!+8"),
            move_to_workspace_8: AhkVar::new("Alt + Shift + 9", "!+9"),
            move_to_workspace_9: AhkVar::new("Alt + Shift + 0", "!+0"),
            send_to_workspace_0: AhkVar::new("Win + Shift + 1", "#+1"),
            send_to_workspace_1: AhkVar::new("Win + Shift + 2", "#+2"),
            send_to_workspace_2: AhkVar::new("Win + Shift + 3", "#+3"),
            send_to_workspace_3: AhkVar::new("Win + Shift + 4", "#+4"),
            send_to_workspace_4: AhkVar::new("Win + Shift + 5", "#+5"),
            send_to_workspace_5: AhkVar::new("Win + Shift + 6", "#+6"),
            send_to_workspace_6: AhkVar::new("Win + Shift + 7", "#+7"),
            send_to_workspace_7: AhkVar::new("Win + Shift + 8", "#+8"),
            send_to_workspace_8: AhkVar::new("Win + Shift + 9", "#+9"),
            send_to_workspace_9: AhkVar::new("Win + Shift + 0", "#+0"),
            // miscellaneous
            misc_open_settings: AhkVar::new("Ctrl + Win + I", "^#i").readonly(),
            misc_toggle_lock_tracing: AhkVar::new("Ctrl + Win + Alt + T", "^#!t")
                .readonly()
                .devonly(),
            misc_toggle_win_event_tracing: AhkVar::new("Ctrl + Win + Alt + L", "^#!l")
                .readonly()
                .devonly(),
        }
    }
}

// ========================== Seelen Updates ==============================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
pub enum UpdateChannel {
    Release,
    Beta,
    Nightly,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
pub struct UpdaterSettings {
    pub channel: UpdateChannel,
}

impl Default for UpdaterSettings {
    fn default() -> Self {
        Self {
            channel: UpdateChannel::Release,
        }
    }
}

// ======================== Final Settings Struct ===============================

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
pub enum VirtualDesktopStrategy {
    Native,
    Seelen,
}

#[serde_alias(SnakeCase)]
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export)]
pub struct Settings {
    /// @deprecated since v2.1.0, will be removed in v3.0.0
    #[ts(skip)]
    #[serde(skip_serializing)]
    fancy_toolbar: Option<FancyToolbarSettings>,
    ///@deprecated since v2.1.0, will be removed in v3.0.0
    #[ts(skip)]
    #[serde(skip_serializing)]
    seelenweg: Option<SeelenWegSettings>,
    /// @deprecated since v2.1.0, will be removed in v3.0.0
    #[ts(skip)]
    #[serde(skip_serializing)]
    window_manager: Option<WindowManagerSettings>,
    /// @deprecated since v2.1.0, will be removed in v3.0.0
    #[ts(skip)]
    #[serde(skip_serializing)]
    wall: Option<SeelenWallSettings>,
    /// @deprecated since v2.1.0, will be removed in v3.0.0
    #[ts(skip)]
    #[serde(skip_serializing)]
    launcher: Option<SeelenLauncherSettings>,
    /// list of monitors and their configurations
    pub monitors_v2: HashMap<String, MonitorConfiguration>,
    /// enable or disable ahk
    pub ahk_enabled: bool,
    /// ahk variables
    pub ahk_variables: AhkVarList,
    /// list of selected themes
    #[serde(alias = "selected_theme")]
    pub selected_themes: Vec<String>,
    /// list of selected icon packs
    pub icon_packs: Vec<String>,
    /// enable or disable dev tools tab in settings
    pub dev_tools: bool,
    /// language to use, if null the system locale is used
    pub language: Option<String>,
    /// MomentJS date format
    pub date_format: String,
    /// what virtual desktop implementation will be used, in case Native is not available we use Seelen
    pub virtual_desktop_strategy: VirtualDesktopStrategy,
    /// Updater Settings
    pub updater: UpdaterSettings,
    /// Custom settings for widgets
    pub by_widget: SettingsByWidget,
}

impl Default for Settings {
    fn default() -> Self {
        let mut settings_by_widget = SettingsByWidget::default();
        settings_by_widget
            .set_config(WidgetId::known_toolbar(), FancyToolbarSettings::default())
            .unwrap();
        settings_by_widget
            .set_config(WidgetId::known_weg(), SeelenWegSettings::default())
            .unwrap();
        settings_by_widget
            .set_config(WidgetId::known_wm(), WindowManagerSettings::default())
            .unwrap();
        settings_by_widget
            .set_config(WidgetId::known_wall(), SeelenWallSettings::default())
            .unwrap();
        settings_by_widget
            .set_config(
                WidgetId::known_launcher(),
                SeelenLauncherSettings::default(),
            )
            .unwrap();

        Self {
            fancy_toolbar: None,
            seelenweg: None,
            window_manager: None,
            wall: None,
            launcher: None,
            // ---
            ahk_enabled: true,
            selected_themes: vec!["default".to_string()],
            icon_packs: vec!["system".to_string()],
            monitors_v2: HashMap::new(),
            ahk_variables: AhkVarList::default(),
            dev_tools: false,
            language: Some(Self::get_system_language()),
            date_format: "ddd D MMM, hh:mm A".to_owned(),
            virtual_desktop_strategy: VirtualDesktopStrategy::Native,
            updater: UpdaterSettings::default(),
            by_widget: settings_by_widget,
        }
    }
}

impl Settings {
    pub fn get_locale() -> Option<String> {
        sys_locale::get_locale()
    }

    pub fn get_system_language() -> String {
        match sys_locale::get_locale() {
            Some(l) => l.split('-').next().unwrap_or("en").to_string(),
            None => "en".to_string(),
        }
    }

    pub fn fancy_toolbar(&self) -> FancyToolbarSettings {
        self.by_widget.get_config(&WidgetId::known_toolbar())
    }

    pub fn fancy_toolbar_mut(&mut self) -> &mut serde_json::Map<String, serde_json::Value> {
        self.by_widget
            .get_raw_config_mut(&WidgetId::known_toolbar())
            .unwrap()
    }

    pub fn seelenweg(&self) -> SeelenWegSettings {
        self.by_widget.get_config(&WidgetId::known_weg())
    }

    pub fn window_manager(&self) -> WindowManagerSettings {
        self.by_widget.get_config(&WidgetId::known_wm())
    }

    pub fn window_manager_mut(&mut self) -> &mut serde_json::Map<String, serde_json::Value> {
        self.by_widget
            .get_raw_config_mut(&WidgetId::known_wm())
            .unwrap()
    }

    pub fn wall(&self) -> SeelenWallSettings {
        self.by_widget.get_config(&WidgetId::known_wall())
    }

    pub fn launcher(&self) -> SeelenLauncherSettings {
        self.by_widget.get_config(&WidgetId::known_launcher())
    }

    /// Migrate old settings (before v2.1.0) (will be removed in v3.0.0)
    pub fn migrate(&mut self) -> Result<(), serde_json::Error> {
        let dict = &mut self.by_widget;
        if let Some(tb) = self.fancy_toolbar.take() {
            dict.set_config(WidgetId::known_toolbar(), tb)?;
        }
        if let Some(weg) = self.seelenweg.take() {
            dict.set_config(WidgetId::known_weg(), weg)?;
        }
        if let Some(wm) = self.window_manager.take() {
            dict.set_config(WidgetId::known_wm(), wm)?;
        }
        if let Some(wall) = self.wall.take() {
            dict.set_config(WidgetId::known_wall(), wall)?;
        }
        if let Some(launcher) = self.launcher.take() {
            dict.set_config(WidgetId::known_launcher(), launcher)?;
        }
        Ok(())
    }

    pub fn sanitize(&mut self) -> Result<(), serde_json::Error> {
        self.by_widget.sanitize();

        let mut wall = self.wall();
        wall.sanitize();
        self.by_widget.set_config(WidgetId::known_wall(), wall)?;

        let mut launcher = self.launcher();
        launcher.sanitize();
        self.by_widget
            .set_config(WidgetId::known_launcher(), launcher)?;

        if self.language.is_none() {
            self.language = Some(Self::get_system_language());
        }

        let default_theme = "default".to_owned();
        if !self.selected_themes.contains(&default_theme) {
            self.selected_themes.insert(0, default_theme);
        }

        let default_icon_pack = "system".to_owned();
        if !self.icon_packs.contains(&default_icon_pack) {
            self.icon_packs.insert(0, default_icon_pack);
        }

        for m in self.monitors_v2.values_mut() {
            m.sanitize();
        }

        Ok(())
    }
}
