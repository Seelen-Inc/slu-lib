import { SeelenCommand, SeelenEvent } from '../handlers/mod.ts';
import { List } from '../utils/List.ts';
import { createInstanceInvoker, createInstanceOnEvent } from '../utils/State.ts';

declare global {
  interface ArgsByCommand {
    [SeelenCommand.SystemGetMonitors]: null;
  }
  interface ReturnByCommand {
    [SeelenCommand.SystemGetMonitors]: ConnectedMonitor[];
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
}

export class ConnectedMonitorList extends List<ConnectedMonitor> {
  static readonly getAsync = createInstanceInvoker(this, SeelenCommand.SystemGetMonitors);
  static readonly onChange = createInstanceOnEvent(this, SeelenEvent.SystemMonitorsChanged);
}
