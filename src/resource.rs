use std::{collections::HashMap, hash::Hash, sync::OnceLock};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use url::Url;
use uuid::Uuid;

use crate::error::Result;

#[derive(
    Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema, TS,
)]
/// visual id composed of the creator username and the resource name
pub struct ResourceId(String);

static REGEX: OnceLock<regex::Regex> = OnceLock::new();

impl ResourceId {
    fn regex() -> &'static regex::Regex {
        REGEX.get_or_init(|| regex::Regex::new("^@[a-zA-Z0-9\\-]+\\/[a-zA-Z0-9\\-]+$").unwrap())
    }

    pub fn is_valid(&self) -> bool {
        Self::regex().is_match(&self.0)
    }

    /// Creator username of the resource
    ///
    /// # Safety
    ///
    /// The string is a valid resource id
    pub fn creator(&self) -> String {
        self.0
            .split('/')
            .next()
            .unwrap()
            .trim_start_matches('@')
            .to_string()
    }

    /// Name of the resource
    ///
    /// # Safety
    ///
    /// The string is a valid resource id
    pub fn name(&self) -> String {
        self.0.split('/').last().unwrap().to_string()
    }
}

impl Default for ResourceId {
    fn default() -> Self {
        Self("@unknown/unknown".to_owned())
    }
}

impl std::ops::Deref for ResourceId {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ResourceId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<String> for ResourceId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&String> for ResourceId {
    fn from(value: &String) -> Self {
        Self(value.clone())
    }
}

impl From<&str> for ResourceId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl std::fmt::Display for ResourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// =============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(untagged, rename_all = "camelCase")]
pub enum ResourceText {
    En(String),
    Localized(HashMap<String, String>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(default, rename_all = "camelCase")]
pub struct ResourceMetadata {
    /// The author name of the resource
    pub author: String,
    /// Map of language code to display name. Could be a string, mapped to `en`.
    pub display_name: ResourceText,
    /// Map of language code to description. Could be a string, mapped to `en`.
    pub description: ResourceText,
    pub portrait: Option<Url>,
    pub screenshots: Vec<Url>,
    pub tags: Vec<String>,
    #[serde(skip_deserializing)]
    pub filename: String,
    #[serde(skip_deserializing)]
    pub bundled: bool,
}

impl Default for ResourceMetadata {
    fn default() -> Self {
        Self {
            author: String::new(),
            display_name: ResourceText::Localized(HashMap::new()),
            description: ResourceText::Localized(HashMap::new()),
            portrait: None,
            screenshots: Vec::new(),
            tags: Vec::new(),
            filename: String::new(),
            bundled: false,
        }
    }
}

// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
pub enum ResourceKind {
    Widget,
    Plugin,
    Theme,
    IconPack,
}

// =============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
pub enum ResourceStatus {
    Draft,
    Reviewing,
    Published,
    Deleted,
}

// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
#[ts(export)]
pub struct Resource {
    pub _id: Uuid,
    pub creator_id: Uuid,
    /// visual id composed of creator username and resource name
    pub friendly_id: ResourceId,
    pub metadata: ResourceMetadata,
    pub created_at: String,
    pub updated_at: String,
    pub status: ResourceStatus,
    /// should be filled if `status == ResourceStatus::Deleted`
    pub deleted_at: Option<String>,
    pub version: u32,
    pub kind: ResourceKind,
    /// id of the document containing the resource
    pub data_id: Uuid,
}

impl Resource {
    pub fn verify(&self) -> Result<()> {
        if !self.friendly_id.is_valid() {
            return Err("invalid resource id".into());
        }

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
