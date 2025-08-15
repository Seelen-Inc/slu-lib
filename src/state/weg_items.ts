import type { MonitorId, WegItem, WegItems as IWegItems } from '@seelen-ui/types';
import { invoke, SeelenCommand } from '../handlers/mod.ts';
import { newFromInvoke } from '../utils/State.ts';
import type { Enum } from '../utils/enums.ts';

export class WegItems {
  constructor(public inner: IWegItems) {}

  /** Will return the weg items state without filtering by monitor */
  static getNonFiltered(): Promise<WegItems> {
    return newFromInvoke(this, SeelenCommand.StateGetWegItems);
  }

  /** Will return the weg items state for a specific monitor */
  static getForMonitor(monitorId: MonitorId): Promise<WegItems> {
    return newFromInvoke(this, SeelenCommand.StateGetWegItems, { monitorId });
  }

  /** Will store the weg items placeoments on disk */
  save(): Promise<void> {
    return invoke(SeelenCommand.StateWriteWegItems, { items: this.inner });
  }
}

// =================================================================================
//    From here some enums as helpers like @seelen-ui/types only contains types
// =================================================================================

const WegItemType: Enum<WegItem['type']> = {
  Pinned: 'Pinned',
  Temporal: 'Temporal',
  Separator: 'Separator',
  Media: 'Media',
  StartMenu: 'StartMenu',
};

export { WegItemType };
