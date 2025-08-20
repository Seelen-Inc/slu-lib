mod resource_id;
pub use resource_id::{IconPackId, PluginId, ResourceId, ThemeId, WallpaperId, WidgetId};

use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

use base64::Engine;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use url::Url;
use uuid::Uuid;

use crate::{
    error::Result,
    state::{IconPack, Plugin, Theme, Wallpaper, Widget},
    utils::{search_for_metadata_file, TsUnknown},
};

// =============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(untagged)]
/// Map of language code as key an translated values. Could be a string, mapped to `en`.
pub enum ResourceText {
    En(String),
    Localized(HashMap<String, String>),
}

impl ResourceText {
    const MISSING_TEXT: &'static str = "!?";

    /// Returns the text by lang, uses `en` as fallback.
    /// If no text fallback found will return `!?`
    pub fn get(&self, lang: &str) -> &str {
        match self {
            ResourceText::En(value) => value,
            ResourceText::Localized(map) => match map.get(lang) {
                Some(value) => value,
                None => match map.get("en") {
                    Some(value) => value,
                    None => Self::MISSING_TEXT,
                },
            },
        }
    }
}

// =============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(default, rename_all = "camelCase")]
pub struct ResourceMetadata {
    /// Map of language code to display name. Could be a string, mapped to `en`.
    pub display_name: ResourceText,
    /// Map of language code to description. Could be a string, mapped to `en`.
    pub description: ResourceText,
    /// Portrait image with aspect ratio of 1/1
    pub portrait: Option<Url>,
    /// Banner image with aspect ratio of 21/9, this is used when promoting the resource.
    pub banner: Option<Url>,
    /// Screenshots should use aspect ratio of 16/9
    pub screenshots: Vec<Url>,
    /// tags are keywords to be used for searching and indexing
    pub tags: Vec<String>,
    /// App target version that this resource is compatible with.\
    /// Developers are responsible to update the resource so when resource does not
    /// match the current app version, the resource will be shown with a warning message
    pub app_target_version: Option<(u32, u32, u32)>,
    #[serde(flatten, skip_deserializing)]
    pub internal: InternalResourceMetadata,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
pub struct InternalResourceMetadata {
    pub path: PathBuf,
    pub filename: String,
    pub bundled: bool,
    pub downloaded_at: Option<DateTime<Utc>>,
}

impl Default for ResourceMetadata {
    fn default() -> Self {
        Self {
            display_name: ResourceText::Localized(HashMap::new()),
            description: ResourceText::Localized(HashMap::new()),
            portrait: None,
            banner: None,
            screenshots: Vec::new(),
            tags: Vec::new(),
            app_target_version: None,
            internal: InternalResourceMetadata::default(),
        }
    }
}

// =============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
pub enum ResourceKind {
    Theme,
    IconPack,
    Widget,
    Plugin,
    Wallpaper,
    SoundPack,
}

// =============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
pub enum ResourceStatus {
    /// Initial state
    Draft,
    /// Waiting for review
    Reviewing,
    /// review done and rejected
    Rejected,
    /// review done and approved
    Published,
    /// soft delete by user
    Deleted,
}

// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Resource {
    pub id: Uuid,
    pub creator_id: Uuid,
    /// visual id composed of creator username and resource name
    pub friendly_id: ResourceId,
    pub metadata: ResourceMetadata,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub status: ResourceStatus,
    pub rejected_reason: Option<String>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub reviewed_by: Option<Uuid>,
    /// should be filled if `status == ResourceStatus::Deleted`
    pub deleted_at: Option<DateTime<Utc>>,
    pub version: u32,
    pub kind: ResourceKind,
    /// id of the document containing the resource
    pub data_id: Uuid,
    pub stars: u32,
    pub downloads: u32,
}

impl Resource {
    pub fn verify(&self) -> Result<()> {
        if let ResourceText::Localized(map) = &self.metadata.display_name {
            if map.get("en").is_none() {
                return Err("missing mandatory english display name".into());
            }
        }

        if let ResourceText::Localized(map) = &self.metadata.description {
            if map.get("en").is_none() {
                return Err("missing mandatory english description".into());
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct SluResourceFile {
    pub version: u32,
    pub resource: Resource,
    pub data: TsUnknown,
}

pub enum ConcreteResource {
    Theme(Theme),
    Plugin(Plugin),
    IconPack(IconPack),
    Widget(Widget),
    Wallpaper(Wallpaper),
}

impl SluResourceFile {
    pub fn decode<R: Read + Seek>(mut reader: R) -> Result<Self> {
        let mut version = [0u8; 1];
        reader.read_exact(&mut version)?;

        match version[0] {
            1 => {
                reader.seek(SeekFrom::Current(3))?; // SLU mime type
            }
            2 => {
                reader.seek(SeekFrom::Current(3))?; // SLU mime type
                reader.seek(SeekFrom::Current(4))?; // 32 bits reserved
            }
            // todo for version 3, use zip crate
            _ => {
                return Err("unsupported slu file version".into());
            }
        }

        // read the rest of the body as content
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let decoded = base64::engine::general_purpose::STANDARD.decode(&buffer)?;
        Ok(serde_yaml::from_slice(&decoded)?)
    }

    pub fn encode<W: Write>(&self, mut writer: W) -> Result<()> {
        let data = serde_yaml::to_string(self)?;
        let encoded = base64::engine::general_purpose::STANDARD.encode(data);

        writer.write_all(&[2])?; // version
        writer.write_all("SLU".as_bytes())?; // SLU mime type
        writer.write_all(&[0u8; 4])?; // 32 bits reserved
        writer.write_all(encoded.as_bytes())?;
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        let mut decoded = Self::decode(&file)?;
        decoded.resource.verify()?;
        decoded.resource.metadata.internal.downloaded_at = Some(file.metadata()?.created()?.into());
        Ok(decoded)
    }

    pub fn store(&self, path: &Path) -> Result<()> {
        let mut file = File::create(path)?;
        self.encode(&mut file)
    }

    pub fn concrete(&self) -> Result<ConcreteResource> {
        let mut resource = serde_json::value::Map::new();

        resource.insert(
            "id".to_string(),
            serde_json::Value::String(self.resource.friendly_id.to_string()),
        );
        resource.insert(
            "metadata".to_string(),
            serde_json::to_value(&self.resource.metadata)?,
        );

        let data = self.data.0.as_object().ok_or("invalid data")?;
        resource.append(&mut data.clone());

        let concrete = match self.resource.kind {
            ResourceKind::Theme => {
                ConcreteResource::Theme(serde_json::from_value(resource.into())?)
            }
            ResourceKind::Plugin => {
                ConcreteResource::Plugin(serde_json::from_value(resource.into())?)
            }
            ResourceKind::IconPack => {
                ConcreteResource::IconPack(serde_json::from_value(resource.into())?)
            }
            ResourceKind::Widget => {
                ConcreteResource::Widget(serde_json::from_value(resource.into())?)
            }
            _ => return Err("unsupported resource kind".into()),
        };

        Ok(concrete)
    }
}

pub trait SluResource: Sized + Serialize {
    fn metadata(&self) -> &ResourceMetadata;
    fn metadata_mut(&mut self) -> &mut ResourceMetadata;

    fn load_from_file(path: &Path) -> Result<Self>;

    fn load_from_folder(path: &Path) -> Result<Self>;

    fn sanitize(&mut self) {}
    fn validate(&self) -> Result<()> {
        Ok(())
    }

    fn load(path: &Path) -> Result<Self> {
        let mut resource = if path.is_dir() {
            Self::load_from_folder(path)?
        } else {
            Self::load_from_file(path)?
        };

        let meta = resource.metadata_mut();
        meta.internal.path = path.to_path_buf();
        meta.internal.filename = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        resource.sanitize();
        resource.validate()?;
        Ok(resource)
    }

    fn save(&self) -> Result<()> {
        let mut save_path = self.metadata().internal.path.to_path_buf();
        if save_path.is_dir() {
            save_path = search_for_metadata_file(&save_path)
                .unwrap_or_else(|| save_path.join("metadata.yml"));
        }

        let extension = save_path
            .extension()
            .ok_or("Invalid path extension")?
            .to_string_lossy()
            .to_lowercase();

        match extension.as_str() {
            "slu" => {
                let mut slu_file = SluResourceFile::load(&save_path)?;
                slu_file.data = serde_json::to_value(self)?.into();
                slu_file.store(&save_path)?;
            }
            "yml" | "yaml" => {
                let file = File::create(save_path)?;
                serde_yaml::to_writer(file, self)?;
            }
            "json" | "jsonc" => {
                let file = File::create(save_path)?;
                serde_json::to_writer_pretty(file, self)?;
            }
            _ => {
                return Err("Unsupported path extension".into());
            }
        }
        Ok(())
    }

    fn delete(&self) -> Result<()> {
        let path = self.metadata().internal.path.to_path_buf();
        if path.is_dir() {
            std::fs::remove_dir_all(path)?;
        } else {
            std::fs::remove_file(path)?;
        }
        Ok(())
    }
}
