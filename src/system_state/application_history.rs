use std::path::PathBuf;

use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ApplicationHistoryEntry {
    pub application: FocusedApp,
    pub focus_date: u64,
    pub is_seelen: bool,
    pub on_monitor: String
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct FocusedApp {
    pub hwnd: isize,
    pub title: String,
    pub name: String,
    pub exe: Option<PathBuf>,
    pub umid: Option<String>,
}
