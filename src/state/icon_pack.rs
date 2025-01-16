use std::{collections::HashMap, path::PathBuf};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::ResourceMetadata;

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize, JsonSchema, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export)]
pub struct IconPack {
    pub info: ResourceMetadata,
    /// Key can be user model id, filename or a full path.
    ///
    /// Value is the path to the icon relative to the icon pack folder.
    pub apps: HashMap<String, PathBuf>,
}
