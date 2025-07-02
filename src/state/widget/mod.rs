pub mod declaration;

use std::{fs::File, path::Path};

use declaration::WidgetSettingsDeclarationList;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    error::Result,
    resource::{ConcreteResource, ResourceMetadata, SluResource, SluResourceFile, WidgetId},
    utils::search_for_metadata_file,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export)]
pub struct Widget {
    /// Resource id ex: `@seelen/weg`
    pub id: WidgetId,
    /// Optional icon to be used on settings. This have to be a valid react icon name.\
    /// You can find all icons here: https://react-icons.github.io/react-icons/.
    pub icon: Option<String>,
    /// Widget metadata, as texts, tags, images, etc.
    pub metadata: ResourceMetadata,
    /// How many instances are allowed of this widget.
    pub instances: WidgetInstanceType,
    /// Widget settings declaration, this is esentially a struct to be used by an
    /// builder to create the widget settings UI on the Settings window.
    pub settings: WidgetSettingsDeclarationList,
    /// Optional widget js code
    pub js: Option<String>,
    /// Optional widget css
    pub css: Option<String>,
    /// Optional widget html
    pub html: Option<String>,
}

impl SluResource for Widget {
    fn metadata(&self) -> &ResourceMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut ResourceMetadata {
        &mut self.metadata
    }

    fn load_from_file(path: &Path) -> Result<Self> {
        let extension = path
            .extension()
            .ok_or("Invalid widget path extension")?
            .to_string_lossy();

        let resource = match extension.as_ref() {
            "yml" | "yaml" => serde_yaml::from_reader(File::open(path)?)?,
            "json" | "jsonc" => serde_json::from_reader(File::open(path)?)?,
            "slu" => match SluResourceFile::load(path)?.concrete()? {
                ConcreteResource::Widget(widget) => widget,
                _ => return Err("Resource file is not a widget".into()),
            },
            _ => {
                return Err("Invalid widget extension".into());
            }
        };
        Ok(resource)
    }

    fn load_from_folder(path: &Path) -> Result<Widget> {
        let file = search_for_metadata_file(path).ok_or("No metadata file found")?;
        let mut widget = Self::load_from_file(&file)?;

        for stem in ["index.js", "main.js", "mod.js"] {
            if path.join(stem).exists() {
                widget.js = Some(std::fs::read_to_string(path.join(stem))?);
                break;
            }
        }

        for stem in ["index.css", "main.css", "mod.css"] {
            if path.join(stem).exists() {
                widget.css = Some(std::fs::read_to_string(path.join(stem))?);
                break;
            }
        }

        for stem in ["index.html", "main.html", "mod.html"] {
            if path.join(stem).exists() {
                widget.html = Some(std::fs::read_to_string(path.join(stem))?);
                break;
            }
        }

        Ok(widget)
    }

    fn validate(&self) -> Result<()> {
        if self.settings.there_are_duplicates() {
            return Err("Widget settings declaration have duplicated keys".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, JsonSchema, TS)]
pub enum WidgetInstanceType {
    /// Default behavior, only one instance of this widget is allowed.
    /// This is useful for widgets intended to work as custom config window.
    #[default]
    Single,
    /// The widget is allowed to have multiple instances.\
    /// This allow to the user manually create more instances of this same widget.
    Multiple,
    /// Seelen UI will create an instance of this widget per each monitor connected.\
    /// This can be configured by the user using per monitor settings.\
    ReplicaByMonitor,
}
