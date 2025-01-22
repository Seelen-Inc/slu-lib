import type { Placeholder, ToolbarItem, WorkspaceToolbarItemMode } from '@seelen-ui/types';
import { enumFromUnion } from '../utils/enums.ts';
import type { SeelenCommand, SeelenEvent } from '../handlers/mod.ts';

declare global {
  interface ArgsByCommand {
    [SeelenCommand.StateGetToolbarItems]: null;
  }
  interface ReturnByCommand {
    [SeelenCommand.StateGetToolbarItems]: Placeholder;
  }
  interface PayloadByEvent {
    [SeelenEvent.StateToolbarItemsChanged]: Placeholder;
  }
}

// =================================================================================
//    From here some enums as helpers like @seelen-ui/types only contains types
// =================================================================================

const ToolbarModuleType = enumFromUnion<ToolbarItem['type']>({
  Text: 'text',
  Generic: 'generic',
  Date: 'date',
  Power: 'power',
  Network: 'network',
  Media: 'media',
  Tray: 'tray',
  User: 'user',
  Notifications: 'notifications',
  Device: 'device',
  Settings: 'settings',
  Workspaces: 'workspaces',
});

const WorkspaceToolbarItemMode = enumFromUnion<WorkspaceToolbarItemMode>({
  Dotted: 'dotted',
  Named: 'named',
  Numbered: 'numbered',
});

export { ToolbarModuleType, WorkspaceToolbarItemMode };
