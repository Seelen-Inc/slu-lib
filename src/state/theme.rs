use std::{collections::HashMap, path::Path};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    error::Result,
    resource::{ResourceId, ResourceMetadata},
};

use super::WidgetId;

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
pub struct ThemeId(ResourceId);

impl std::ops::Deref for ThemeId {
    type Target = ResourceId;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ThemeId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<&str> for ThemeId {
    fn from(value: &str) -> Self {
        Self(ResourceId::from(value))
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export)]
pub struct Theme {
    pub id: ThemeId,
    /// Metadata about the theme
    #[serde(alias = "info")]
    pub metadata: ResourceMetadata,
    /// Css Styles of the theme
    pub styles: HashMap<WidgetId, String>,
}

impl Theme {
    pub fn load_from_file(path: &Path) -> Result<Theme> {
        let extension = path.extension().ok_or("Invalid theme path extension")?;
        if extension != "yml" && extension != "yaml" {
            return Err("Invalid theme path extension".into());
        }
        let theme = serde_yaml::from_reader(std::fs::File::open(path)?)?;
        Ok(theme)
    }

    /// Load theme from a folder using old deprecated paths since v2.1.0 will be removed in v3
    fn load_old_folder_schema(path: &Path) -> Result<Theme> {
        if !path.is_dir() {
            return Err("Invalid theme path".into());
        }

        let mut theme = Self::load_from_file(&path.join("theme.yml"))?;
        theme.metadata.filename = path.file_name().unwrap().to_string_lossy().to_string();

        if path.join("theme.weg.css").exists() {
            theme.styles.insert(
                WidgetId::known_weg(),
                std::fs::read_to_string(path.join("theme.weg.css"))?,
            );
        }
        if path.join("theme.toolbar.css").exists() {
            theme.styles.insert(
                WidgetId::known_toolbar(),
                std::fs::read_to_string(path.join("theme.toolbar.css"))?,
            );
        }
        if path.join("theme.wm.css").exists() {
            theme.styles.insert(
                WidgetId::known_wm(),
                std::fs::read_to_string(path.join("theme.wm.css"))?,
            );
        }
        if path.join("theme.launcher.css").exists() {
            theme.styles.insert(
                WidgetId::known_launcher(),
                std::fs::read_to_string(path.join("theme.launcher.css"))?,
            );
        }
        if path.join("theme.wall.css").exists() {
            theme.styles.insert(
                WidgetId::known_wall(),
                std::fs::read_to_string(path.join("theme.wall.css"))?,
            );
        };
        Ok(theme)
    }

    pub fn load_from_folder(path: &Path) -> Result<Theme> {
        let mut theme = Self::load_old_folder_schema(path)?;

        for entry in path.read_dir()?.flatten() {
            let dir_path = entry.path();
            if !dir_path.is_dir() {
                continue;
            }
            let creator_username = entry.file_name();
            'inner: for entry in dir_path.read_dir()?.flatten() {
                let path = entry.path();
                if !path.is_file() {
                    continue 'inner;
                }
                let ext = match path.extension() {
                    Some(ext) => ext,
                    None => continue 'inner,
                };
                let resource_name = match path.file_stem() {
                    Some(name) => name,
                    None => continue 'inner,
                };
                if ext == "css" || ext == "scss" || ext == "sass" {
                    let css = if ext == "scss" || ext == "sass" {
                        grass::from_path(&path, &grass::Options::default())?
                    } else {
                        std::fs::read_to_string(&path)?
                    };
                    theme.styles.insert(
                        WidgetId::from(
                            format!(
                                "@{}/{}",
                                creator_username.to_string_lossy(),
                                resource_name.to_string_lossy()
                            )
                            .as_str(),
                        ),
                        css,
                    );
                }
            }
        }
        Ok(theme)
    }

    pub fn load(path: &Path) -> Result<Theme> {
        if path.is_dir() {
            return Self::load_from_folder(path);
        }
        Self::load_from_file(path)
    }
}
