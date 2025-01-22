import type { WegItem, WegItems as IWegItems } from '@seelen-ui/types';
import { invoke, SeelenCommand, SeelenEvent } from '../handlers/mod.ts';
import { createInstanceInvoker, createInstanceOnEvent } from '../utils/State.ts';
import { enumFromUnion } from '../utils/enums.ts';
import { getCurrentWidget } from './widget.ts';

declare global {
  interface ArgsByCommand {
    [SeelenCommand.StateGetWegItems]: null;
    [SeelenCommand.WegGetItemsForWidget]: null;
    [SeelenCommand.StateWriteWegItems]: { items: IWegItems };
  }
  interface ReturnByCommand {
    [SeelenCommand.StateGetWegItems]: IWegItems;
    [SeelenCommand.WegGetItemsForWidget]: IWegItems;
    [SeelenCommand.StateWriteWegItems]: void;
  }
  interface PayloadByEvent {
    [SeelenEvent.StateWegItemsChanged]: IWegItems;
    [SeelenEvent.WegInstanceChanged]: IWegItems;
  }
}

export class WegItems {
  constructor(public inner: IWegItems) {}

  /** Will return the stored/saved weg items */
  static readonly getStored = createInstanceInvoker(this, SeelenCommand.StateGetWegItems);

  /** Event triggered when the file where the weg items are stored is changed */
  static readonly onStoredChange = createInstanceOnEvent(this, SeelenEvent.StateWegItemsChanged);

  /** Will return the weg items intance for the current widget */
  static readonly forCurrentWidget = createInstanceInvoker(
    this,
    SeelenCommand.WegGetItemsForWidget,
  );

  /** Event triggered when the weg items for the current widget are changed */
  static readonly forCurrentWidgetChange = createInstanceOnEvent(this, SeelenEvent.WegInstanceChanged, {
    target: { kind: 'Webview', label: getCurrentWidget().rawLabel },
  });

  /** Will store the weg items placeoments on disk */
  save(): Promise<void> {
    return invoke(SeelenCommand.StateWriteWegItems, { items: this.inner });
  }
}

// =================================================================================
//    From here some enums as helpers like @seelen-ui/types only contains types
// =================================================================================

const WegItemType = enumFromUnion<WegItem['type']>({
  Pinned: 'Pinned',
  Temporal: 'Temporal',
  Separator: 'Separator',
  Media: 'Media',
  StartMenu: 'StartMenu',
});

export { WegItemType };
