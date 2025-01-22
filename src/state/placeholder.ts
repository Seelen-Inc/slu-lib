import type { DateUpdateInterval, Placeholder, ToolbarItem, WorkspaceToolbarItemMode } from '@seelen-ui/types';
import { List } from '../utils/List.ts';
import { SeelenCommand, SeelenEvent } from '../lib.ts';
import { createInstanceInvoker, createInstanceOnEvent } from '../utils/State.ts';
import { enumFromUnion } from '../utils/enums.ts';

declare global {
  interface ArgsByCommand {
    [SeelenCommand.StateGetPlaceholders]: null;
  }
  interface ReturnByCommand {
    [SeelenCommand.StateGetPlaceholders]: Placeholder[];
  }
  interface PayloadByEvent {
    [SeelenEvent.StatePlaceholdersChanged]: Placeholder[];
  }
}

export class PlaceholderList extends List<Placeholder> {
  static readonly getAsync = createInstanceInvoker(this, SeelenCommand.StateGetPlaceholders);
  static readonly onChange = createInstanceOnEvent(this, SeelenEvent.StatePlaceholdersChanged);
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

const DateUpdateInterval = enumFromUnion<DateUpdateInterval>({
  Millisecond: 'millisecond',
  Second: 'second',
  Minute: 'minute',
  Hour: 'hour',
  Day: 'day',
});

export { DateUpdateInterval, ToolbarModuleType, WorkspaceToolbarItemMode };
