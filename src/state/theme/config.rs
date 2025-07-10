use std::sync::LazyLock;

use schemars::JsonSchema;
use serde::{de::Visitor, Deserialize, Deserializer};
use url::Url;

use crate::{error::Result, resource::ResourceText};

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, TS)]
pub struct ThemeSettingsDefinition(Vec<ThemeConfigDefinition>);

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
pub enum ThemeConfigDefinition {
    Group {
        header: ResourceText,
        items: Vec<ThemeConfigDefinition>,
    },
    #[serde(untagged)]
    Item(Box<ThemeVariableDefinition>),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
#[serde(tag = "syntax")]
pub enum ThemeVariableDefinition {
    /// This config definition will allow to users write any string.\
    /// Css syntax: https://developer.mozilla.org/en-US/docs/Web/CSS/string \
    /// ### example:
    /// ```css
    /// --var-name: "user input"
    /// ```
    #[serde(rename = "<string>")]
    String(ThemeVariable<String>),
    /// This config definition will allow to users select a color and
    /// will be stored as a hex value, opacity is always allowed via UI.\
    /// Css syntax: https://developer.mozilla.org/en-US/docs/Web/CSS/color_value \
    /// ### example:
    /// ```css
    /// --var-name: #ff22ee
    /// --var-name: #ff22ee
    /// ```
    #[serde(rename = "<color>")]
    Color(ThemeVariable<String>),
    /// This will allow to the user set any lenght in any unit. (px, %, vw, etc).
    /// If you need force a specific unit, use Number instead lenght and on theme code makes the conversion.\
    /// Css syntax: https://developer.mozilla.org/en-US/docs/Web/CSS/length \
    /// ### example:
    /// ```css
    /// --var-name: 10px
    /// --var-name: 10%
    /// --var-name: 10vw
    /// ```
    #[serde(rename = "<length>")]
    Length(ThemeVariableWithUnit<f64>),
    /// This will allow to users to set any number, without units.
    /// Css syntax: https://developer.mozilla.org/en-US/docs/Web/CSS/number \
    /// ### example:
    /// ```css
    /// --var-name: 10
    /// ```
    #[serde(rename = "<number>")]
    Number(ThemeVariable<f64>),
    /// This will allow to users to set any url.\
    /// Css syntax: https://developer.mozilla.org/en-US/docs/Web/CSS/url_value \
    /// ### example:
    /// ```css
    /// --var-name: url("https://example.com/image.png")
    /// ```
    /// ### Warning: using this will make your resource dependent of the url and network connection.
    #[serde(rename = "<url>")]
    Url(ThemeVariable<Url>),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
pub struct ThemeVariable<T> {
    /// Label to show to the user on Settings.
    pub label: ResourceText,
    /// Extra details to show to the user under the label on Settings.
    pub description: Option<ResourceText>,
    /// Will be rendered as a icon with a tooltip side the label.
    pub tip: Option<ResourceText>,
    /// Css variable name, example: `--my-css-variable`
    pub name: CssVariableName,
    /// Initial variable value, if not manually set by the user.
    pub initial_value: T,
    /// If present, this will be rendered as a selector of options instead of an input.
    /// `initial_value` should be present in this list.
    pub options: Option<Vec<T>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
pub struct ThemeVariableWithUnit<T> {
    /// Label to show to the user on Settings.
    pub label: ResourceText,
    /// Extra details to show to the user under the label on Settings.
    pub description: Option<ResourceText>,
    /// Will be rendered as a icon with a tooltip side the label.
    pub tip: Option<ResourceText>,
    /// Css variable name, example: `--my-css-variable`
    pub name: CssVariableName,
    /// Initial variable value, if not manually set by the user.
    pub initial_value: T,
    /// Css unit, example: `px`
    pub initial_value_unit: String,
}

/// Valid CSS variable name that starts with `--` and follows CSS naming conventions
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, JsonSchema, TS)]
pub struct CssVariableName(String);

static CSS_VAR_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"^--[a-zA-Z_][\w-]*$").unwrap());

impl CssVariableName {
    /// Creates a new CssVariableName after validation
    pub fn from_string(name: &str) -> Result<Self> {
        if !CSS_VAR_REGEX.is_match(name) {
            return Err(format!(
                "Invalid CSS variable name '{name}'. Must start with '--' and follow CSS naming rules"
            )
            .into());
        }
        Ok(Self(name.to_string()))
    }
}

impl std::fmt::Display for CssVariableName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for CssVariableName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CssVariableNameVisitor;

        impl<'de> Visitor<'de> for CssVariableNameVisitor {
            type Value = CssVariableName;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    formatter,
                    "a valid CSS variable name starting with '--' and following CSS naming rules"
                )
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                CssVariableName::from_string(value).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_str(CssVariableNameVisitor)
    }
}
