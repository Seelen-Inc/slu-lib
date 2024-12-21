import { SeelenCommand, SeelenEvent } from '../lib.ts';
import { List } from '../utils/List.ts';
import { createInstanceInvoker, createInstanceOnEvent } from '../utils/State.ts';

declare global {
  interface ArgsByCommand {
    [SeelenCommand.SystemGetMonitors]: null;
  }
  interface ReturnByCommand {
    [SeelenCommand.SystemGetMonitors]: ConnectedMonitor[];
    [SeelenCommand.SystemGetCurrentMonitor]: ConnectedMonitor;
  }
  interface PayloadByEvent {
    [SeelenEvent.SystemMonitorsChanged]: ConnectedMonitor[];
  }
}

export interface ConnectedMonitor {
  id: string;
  name: string;
  width: number;
  height: number;
  dpi: number;
  isPrimary: boolean;
}

export class DisplayMonitor {
  constructor(public monitor: ConnectedMonitor) { }

  static readonly getCurrent = createInstanceInvoker(this, SeelenCommand.SystemGetCurrentMonitor);
}

export class ConnectedMonitorList extends List<ConnectedMonitor> {
  static readonly getAsync = createInstanceInvoker(this, SeelenCommand.SystemGetMonitors);
  static readonly onChange = createInstanceOnEvent(this, SeelenEvent.SystemMonitorsChanged);
}
