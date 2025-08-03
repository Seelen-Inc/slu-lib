use std::collections::HashMap;

use uuid::Uuid;

use crate::{identifier_impl, system_state::MonitorId};

#[derive(Debug, Default, Clone, Serialize, Deserialize, TS)]
#[serde(default, rename_all = "camelCase")]
#[ts(export)]
pub struct VirtualDesktops {
    /// Workspaces per monitor
    pub monitors: HashMap<MonitorId, Vec<DesktopWorkspace>>,
    /// Active workspace per monitor
    pub active_workspace: HashMap<MonitorId, WorkspaceId>,
}

impl VirtualDesktops {
    pub fn sanitize(&mut self) {
        // ensure monitors have at least one workspace
        for workspaces in &mut self.monitors.values_mut() {
            if workspaces.is_empty() {
                workspaces.push(DesktopWorkspace::create());
            }
        }

        // Verify the active workspace exists in this monitor's workspaces
        for (monitor_id, workspaces) in &self.monitors {
            let active_exists =
                self.active_workspace
                    .get(monitor_id)
                    .is_some_and(|active_workspace| {
                        workspaces.iter().any(|ws| &ws.id == active_workspace)
                    });

            if !active_exists {
                self.active_workspace
                    .insert(monitor_id.clone(), workspaces[0].id.clone());
            }
        }

        // Remove active workspaces for monitors that don't exist
        self.active_workspace
            .retain(|monitor_id, _| self.monitors.contains_key(monitor_id));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct DesktopWorkspace {
    pub id: WorkspaceId,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub windows: Vec<isize>,
}

impl DesktopWorkspace {
    pub fn create() -> Self {
        Self {
            id: WorkspaceId(Uuid::new_v4().to_string()),
            name: None,
            windows: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
pub struct WorkspaceId(pub String);

identifier_impl!(WorkspaceId, String);
