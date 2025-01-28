use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use ts_rs::TS;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum FolderType {
    Unknown,
    Recent,
    Downloads,
    Documents,
    Pictures,
    Videos,
    Music,
}

impl FolderType {
    pub fn values() -> [FolderType; 6] {
        [
            FolderType::Recent,
            FolderType::Downloads,
            FolderType::Documents,
            FolderType::Pictures,
            FolderType::Videos,
            FolderType::Music,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct File {
    pub path: PathBuf,
    pub last_access_time: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct FolderChangedArgs {
    pub of_folder: FolderType,
    pub content: Option<Vec<File>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct User {
    pub name: String,
    pub domain: String,
    pub profile_home_path: PathBuf,
    pub email: Option<String>,
    pub one_drive_path: Option<PathBuf>,
    pub profile_picture_path: Option<PathBuf>,
}
