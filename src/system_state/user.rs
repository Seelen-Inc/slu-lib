use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use ts_rs::TS;

const RECENT_FOLDER_RELATIVE_PATH: &str = "..\\..\\Roaming\\Microsoft\\Windows\\Recent";
const DOCUMENTS_FOLDER_RELATIVE_PATH: &str = "..\\..\\..\\Documents";
const DOWNLOADS_RELATIVE_PATH: &str = "..\\..\\..\\Downloads";
const PICTURES_RELATIVE_PATH: &str = "..\\..\\..\\Pictures";
const VIDEOS_RELATIVE_PATH: &str = "..\\..\\..\\Videos";
const MUSIC_RELATIVE_PATH: &str = "..\\..\\..\\Music";

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
    pub fn to_path(&self) -> PathBuf {
        match self {
            FolderType::Recent => std::env::temp_dir().join(RECENT_FOLDER_RELATIVE_PATH),
            FolderType::Downloads => std::env::temp_dir().join(DOWNLOADS_RELATIVE_PATH),
            FolderType::Documents => std::env::temp_dir().join(DOCUMENTS_FOLDER_RELATIVE_PATH),
            FolderType::Pictures => std::env::temp_dir().join(PICTURES_RELATIVE_PATH),
            FolderType::Videos => std::env::temp_dir().join(VIDEOS_RELATIVE_PATH),
            FolderType::Music => std::env::temp_dir().join(MUSIC_RELATIVE_PATH),
            FolderType::Unknown => std::env::temp_dir(),
        }
    }

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

    pub fn from_path(path: &Path) -> FolderType {
        for folder_type in FolderType::values() {
            if path.starts_with(folder_type.to_path()) {
                return folder_type;
            }
        }

        FolderType::Unknown
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
