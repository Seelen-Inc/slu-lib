pub struct SeelenEvent;

macro_rules! slu_events_declaration {
    ($($name:ident = $value:literal,)*) => {
        #[allow(non_upper_case_globals)]
        impl SeelenEvent {
            $(
                pub const $name: &'static str = $value;
            )*

            #[allow(dead_code)]
            pub(crate) fn generate_ts_file(path: &str) {
                let content: Vec<String> = vec![
                    "// This file was generated via rust macros. Don't modify manually.".to_owned(),
                    "export enum SeelenEvent {".to_owned(),
                    $(
                        format!("  {} = '{}',", stringify!($name), Self::$name),
                    )*
                    "}\n".to_owned(),
                ];
                std::fs::write(path, content.join("\n")).unwrap();
            }
        }
    };
}

slu_events_declaration! {
    WorkspacesChanged = "workspaces-changed",
    ActiveWorkspaceChanged = "active-workspace-changed",

    GlobalFocusChanged = "global-focus-changed",
    GlobalMouseMove = "global-mouse-move",

    HandleLayeredHitboxes = "handle-layered",

    SystemMonitorsChanged = "system::monitors-changed",
    SystemLanguagesChanged = "system::languages-changed",

    UserChanged = "user-changed",
    UserFolderChanged = "user-folder-changed",

    MediaSessions = "media-sessions",
    MediaInputs = "media-inputs",
    MediaOutputs = "media-outputs",

    NetworkDefaultLocalIp = "network-default-local-ip",
    NetworkAdapters = "network-adapters",
    NetworkInternetConnection = "network-internet-connection",
    NetworkWlanScanned = "wlan-scanned",

    Notifications = "notifications",

    PowerStatus = "power-status",
    BatteriesStatus = "batteries-status",

    ColorsChanged = "colors-changed",

    TrayInfo = "tray-info",

    ToolbarOverlaped = "set-auto-hide",

    WegOverlaped = "set-auto-hide",
    WegInstanceChanged = "weg::instance-changed",

    WMSetReservation = "set-reservation",
    WMUpdateHeight = "update-height",
    WMUpdateWidth = "update-width",
    WMResetWorkspaceSize = "reset-workspace-size",
    WMFocus = "focus",
    WMSetActiveWorkspace = "set-active-workspace",
    WMAddWindow = "add-window",
    WMUpdateWindow = "update-window",
    WMRemoveWindow = "remove-window",

    WMForceRetiling = "wm-force-retiling",
    WMSetLayout = "wm-set-layout",
    WMSetOverlayVisibility = "wm-set-overlay-visibility",
    WMSetActiveWindow = "wm-set-active-window",

    WallStop = "wall-stop",

    StateSettingsChanged = "settings-changed",
    StateWegItemsChanged = "weg-items",
    StateToolbarItemsChanged = "toolbar-items",
    StateThemesChanged = "themes",
    StateSettingsByAppChanged = "settings-by-app",
    StateHistoryChanged = "history",
    StateIconPacksChanged = "icon-packs",
    StatePluginsChanged = "plugins-changed",
    StateWidgetsChanged = "widgets-changed",
    StateProfilesChanged = "profiles-changed",
}
