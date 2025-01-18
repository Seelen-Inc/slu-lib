use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Wsd = Widget Settings Declaration
macro_rules! wsd_item {
    (
        $(
            $(#[$scope:meta])*
            struct $name:ident {
                type: $item:ident,
                $($rest:tt)*
            }
        )*
    ) => {
        $(
            #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
            #[serde(rename_all = "camelCase")]
            pub struct $name {
                /// Unique key for this item, used to identify it in the settings
                key: String,
                /// Label for this item could start with the prefix `t::` for translation
                ///
                /// example: `t::obj.inner.translation_key`
                label: String,
                /// This setting could be set by monitor on the settings by monitor section.
                allow_set_by_monitor: bool,
                /// Keys of items to be set in order to enable this item.
                ///
                /// it uses js logic (!!value) to determine if the item is enabled
                dependencies: Vec<String>,
                $($rest)*
            }
        )*

        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
        #[serde(tag = "type", rename_all = "kebab-case")]
        pub enum WsdItem {
            $(
                $item($name),
            )*
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
pub enum WsdItemSelectSubtype {
    List,
    Inline,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
pub struct WsdItemSelectOption {
    label: String,
    /// The value to be set when this option is selected, should be unique
    value: String,
}

wsd_item! {
    struct WsdItemSwitch {
        type: Switch,
        default: Option<bool>,
    }

    struct WsdItemSelect {
        type: Select,
        default: Option<String>,
        subtype: WsdItemSelectSubtype,
        options: Vec<WsdItemSelectOption>
    }

    struct WsdItemInputText {
        type: InputText,
        default: Option<String>,
    }

    struct WsdItemInputNumber {
        type: InputNumber,
        default: Option<i32>,
        min: Option<i32>,
        max: Option<i32>,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
pub enum WsdGroupEntry {
    SubGroup(WsdGroup),
    Config(WsdItem),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
pub struct WsdGroup {
    /// Header configuration normally is a switch to enable or disable the group.
    ///
    /// **Note**: This property is ignored if the group is the root. Should be used on SubGroups.
    header: Option<WsdItem>,
    settings: Vec<WsdGroupEntry>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, TS)]
#[ts(export)]
pub struct WidgetSettingsDeclarationList(Vec<WsdGroup>);
