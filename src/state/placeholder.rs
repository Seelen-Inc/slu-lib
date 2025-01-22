use std::collections::{HashMap, HashSet};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::PluginId;

macro_rules! common_item {
    (
        $(
            $(#[$scope:meta])*
            struct $name:ident {
                $($rest:tt)*
            }
        )*
    ) => {
        $(
            #[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
            #[serde(default, rename_all = "camelCase")]
            pub struct $name {
                /// Id to identify the item, should be unique.
                pub id: String,
                /// Content to display in the item.
                ///
                /// Should follow the [mathjs expression syntax](https://mathjs.org/docs/expressions/syntax.html).
                ///
                $(#[$scope])*
                pub template: String,
                /// Content to display in tooltip of the item.
                ///
                /// Should follow the [mathjs expression syntax](https://mathjs.org/docs/expressions/syntax.html).
                ///
                $(#[$scope])*
                pub tooltip: Option<String>,
                /// Badge will be displayed over the item, useful as notifications.
                ///
                /// Should follow the [mathjs expression syntax](https://mathjs.org/docs/expressions/syntax.html).
                ///
                $(#[$scope])*
                pub badge: Option<String>,
                /// Deprecated use `onClickV2` instead.
                pub on_click: Option<String>,
                /// This code will be parsed and executed when the item is clicked.
                ///
                /// Should follow the [mathjs expression syntax](https://mathjs.org/docs/expressions/syntax.html).
                ///
                $(#[$scope])*
                pub on_click_v2: Option<String>,
                /// Styles to be added to the item. This follow the same interface of React's `style` prop.
                pub style: HashMap<String, Option<StyleValue>>,
                $($rest)*
            }
        )*
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(untagged)]
pub enum StyleValue {
    String(String),
    Number(serde_json::Number),
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
pub enum WorkspaceToolbarItemMode {
    #[default]
    Dotted,
    Named,
    Numbered,
}

common_item! {
    /// ## Base Item Scope
    /// Have all icons defined on [React Icons](https://react-icons.github.io/react-icons) as properties of the object.
    /// ```js
    /// const icon: object;
    /// ```
    /// Haves all environment variables defined on the system as properties of the object.
    /// ```js
    /// const env: object;
    /// ```
    /// Functions to add images to the item.
    /// ```js
    /// function getIcon(name: string, size: number = 16): string
    /// function imgFromUrl (url: string, size: number = 16): string
    /// function imgFromPath (path: string, size: number = 16): string
    /// function imgFromExe (exe_path: string, size: number = 16): string
    /// function t(path: string): string
    /// ```
    struct TextToolbarItem {}

    /// ## Generic Item Scope
    /// ```ts
    /// // the current focused window
    /// const window: {
    ///     name: string;
    ///     title: string;
    ///     exe: string | null;
    /// };
    /// ```
    struct GenericToolbarItem {}

    /// ## Date Item Scope
    /// ```ts
    /// const date: string; // the formatted date
    /// ```
    struct DateToolbarItem {}

    /// ## Power Item Scope
    /// ```ts
    /// interface PowerStatus {
    ///     acLineStatus: number;
    ///     batteryFlag: number;
    ///     batteryLifePercent: number;
    ///     systemStatusFlag: number;
    ///     batteryLifeTime: number;
    ///     batteryFullLifeTime: number;
    /// }
    ///
    /// interface Battery {
    ///     // Static info
    ///     vendor: string | null;
    ///     model: string | null;
    ///     serialNumber: string | null;
    ///     technology: string;
    ///
    ///     // Common information
    ///     state: string;
    ///     capacity: number;
    ///     temperature: number | null;
    ///     percentage: number;
    ///     cycleCount: number | null;
    ///     smartCharging: boolean;
    ///
    ///     // Energy stats
    ///     energy: number;
    ///     energyFull: number;
    ///     energyFullDesign: number;
    ///     energyRate: number;
    ///     voltage: number;
    ///
    ///     // Charge stats
    ///     timeToFull: number | null;
    ///     timeToEmpty: number | null;
    /// }
    ///
    /// const power: PowerStatus;
    /// const batteries: Battery[];
    /// const battery: Battery | null;
    /// ```
    struct PowerToolbarItem {}

    /// ## Network Item Scope
    /// ```ts
    /// interface NetworkInterface {
    ///     name: string;
    ///     description: string;
    ///     status: 'up' | 'down';
    ///     dnsSuffix: string;
    ///     type: string;
    ///     gateway: string | null;
    ///     mac: string;
    ///     ipv4: string | null;
    ///     ipv6: string | null;
    /// }
    /// const online: boolean;
    /// const interfaces: NetworkInterface[];
    /// const usingInterface: NetworkInterface | null;
    /// ```
    struct NetworkToolbarItem {
        /// Show Wi-fi selector popup on click]
        #[serde(default)]
        with_wlan_selector: bool,
    }

    /// ## Media Item Scope
    /// ```ts
    /// const volume: number; // output master volume from 0 to 1
    /// const isMuted: boolean; // output master volume is muted
    /// const inputVolume: number; // input master volume from 0 to 1
    /// const inputIsMuted: boolean; // input master volume is muted
    ///
    /// interface MediaSession {
    ///     id: string;
    ///     title: string;
    ///     author: string;
    ///     thumbnail: string | null; // path to temporal media session image
    ///     playing: boolean;
    ///     default: boolean;
    ///     owner: {
    ///         name: string;
    ///         iconPath: string | null;
    ///     } | null;
    /// }
    ///
    /// const mediaSession: MediaSession | null;
    /// ```
    struct MediaToolbarItem {
        /// Show media controls popup on click
        #[serde(default)]
        with_media_controls: bool,
    }

    /// ## Notifications Item Scope
    /// ```ts
    /// const count: number;
    /// ```
    struct UserToolbarItem {
        /// Show user control popup on click
        #[serde(default)]
        with_user_folder: bool,
    }


    /// ## Notifications Item Scope
    /// ```ts
    /// const count: number;
    /// ```
    struct NotificationsToolbarItem {}

    /// ## Workspace Item Scope
    /// this module does no expand the scope of the item
    struct TrayToolbarItem {}

    /// ## Device Item Scope
    /// this module does no expand the scope of the item
    struct DeviceToolbarItem {}

    /// ## Settings Item Scope
    /// this module does no expand the scope of the item
    struct SettingsToolbarItem {}

    /// ## Workspace Item Scope
    /// this module does no expand the scope of the item
    struct WorkspaceToolbarItem {
        mode: WorkspaceToolbarItemMode,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ToolbarItem {
    Text(TextToolbarItem),
    Generic(GenericToolbarItem),
    Date(DateToolbarItem),
    Power(PowerToolbarItem),
    Network(NetworkToolbarItem),
    Media(MediaToolbarItem),
    User(UserToolbarItem),
    Notifications(NotificationsToolbarItem),
    Tray(TrayToolbarItem),
    Device(DeviceToolbarItem),
    Settings(SettingsToolbarItem),
    Workspaces(WorkspaceToolbarItem),
}

impl ToolbarItem {
    pub fn id(&self) -> String {
        match self {
            ToolbarItem::Text(item) => item.id.clone(),
            ToolbarItem::Generic(item) => item.id.clone(),
            ToolbarItem::Date(item) => item.id.clone(),
            ToolbarItem::Power(item) => item.id.clone(),
            ToolbarItem::Network(item) => item.id.clone(),
            ToolbarItem::Media(item) => item.id.clone(),
            ToolbarItem::User(item) => item.id.clone(),
            ToolbarItem::Notifications(item) => item.id.clone(),
            ToolbarItem::Tray(item) => item.id.clone(),
            ToolbarItem::Device(item) => item.id.clone(),
            ToolbarItem::Settings(item) => item.id.clone(),
            ToolbarItem::Workspaces(item) => item.id.clone(),
        }
    }

    pub fn set_id(&mut self, id: String) {
        match self {
            ToolbarItem::Text(item) => item.id = id,
            ToolbarItem::Generic(item) => item.id = id,
            ToolbarItem::Date(item) => item.id = id,
            ToolbarItem::Power(item) => item.id = id,
            ToolbarItem::Network(item) => item.id = id,
            ToolbarItem::Media(item) => item.id = id,
            ToolbarItem::User(item) => item.id = id,
            ToolbarItem::Notifications(item) => item.id = id,
            ToolbarItem::Tray(item) => item.id = id,
            ToolbarItem::Device(item) => item.id = id,
            ToolbarItem::Settings(item) => item.id = id,
            ToolbarItem::Workspaces(item) => item.id = id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(untagged)]
pub enum ToolbarItem2 {
    Plugin(PluginId),
    Inline(ToolbarItem),
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export)]
pub struct Placeholder {
    /// Items to be displayed in the toolbar
    pub left: Vec<ToolbarItem2>,
    /// Items to be displayed in the toolbar
    pub center: Vec<ToolbarItem2>,
    /// Items to be displayed in the toolbar
    pub right: Vec<ToolbarItem2>,
}

impl Placeholder {
    fn sanitize_items(dict: &mut HashSet<String>, items: Vec<ToolbarItem2>) -> Vec<ToolbarItem2> {
        let mut result = Vec::new();
        for item in items {
            match item {
                ToolbarItem2::Plugin(id) => {
                    let str_id = id.to_string();
                    if !dict.contains(&str_id) && id.is_valid() {
                        dict.insert(str_id);
                        result.push(ToolbarItem2::Plugin(id));
                    }
                }
                ToolbarItem2::Inline(mut item) => {
                    if item.id().is_empty() {
                        item.set_id(uuid::Uuid::new_v4().to_string());
                    }
                    if !dict.contains(&item.id()) {
                        dict.insert(item.id());
                        result.push(ToolbarItem2::Inline(item));
                    }
                }
            }
        }
        result
    }

    pub fn sanitize(&mut self) {
        let mut dict = HashSet::new();
        self.left = Self::sanitize_items(&mut dict, std::mem::take(&mut self.left));
        self.center = Self::sanitize_items(&mut dict, std::mem::take(&mut self.center));
        self.right = Self::sanitize_items(&mut dict, std::mem::take(&mut self.right));
    }
}
