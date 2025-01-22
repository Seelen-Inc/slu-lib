import { SeelenCommand } from '../../handlers/mod.ts';

import type { MonitorConfiguration as IMonitorConfiguration, WorkspaceIdentifierType } from '@seelen-ui/types';
import { createInstanceInvoker } from '../../utils/State.ts';
import { enumFromUnion } from '../../utils/enums.ts';

declare global {
  interface ArgsByCommand {
    [SeelenCommand.StateGetDefaultMonitorSettings]: null;
  }
  interface ReturnByCommand {
    [SeelenCommand.StateGetDefaultMonitorSettings]: IMonitorConfiguration;
  }
}

export class MonitorConfiguration {
  constructor(public inner: IMonitorConfiguration) {}
  static readonly default = createInstanceInvoker(
    this,
    SeelenCommand.StateGetDefaultMonitorSettings,
  );
}

// =================================================================================
//    From here some enums as helpers like @seelen-ui/types only contains types
// =================================================================================

const WorkspaceIdentifierType = enumFromUnion<WorkspaceIdentifierType>({
  Name: 'Name',
  Index: 'Index',
});

export { WorkspaceIdentifierType };
