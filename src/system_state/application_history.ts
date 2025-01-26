import { getCurrentWidget, invoke, SeelenCommand, SeelenEvent, subscribe } from '../lib.ts';
import { createInstanceInvoker, createInstanceOnEvent } from '../utils/State.ts';
import { List } from '../utils/List.ts';
import { type EventCallback } from '@tauri-apps/api/event';
import type { ApplicationHistoryEntry, FocusedApp } from '@seelen-ui/types';

declare global {
  interface ArgsByCommand {
    [SeelenCommand.GetApplicationHistory]: null;
    [SeelenCommand.GetApplicationHistoryOnMonitor]: null;
    [SeelenCommand.SetApplicationHistoryLimit]: { capacity: number };
  }
  interface ReturnByCommand {
    [SeelenCommand.GetApplicationHistory]: ApplicationHistoryEntry[];
    [SeelenCommand.GetApplicationHistoryOnMonitor]: ApplicationHistoryEntry[];
    [SeelenCommand.SetApplicationHistoryLimit]: null;
  }
  interface PayloadByEvent {
    [SeelenEvent.GlobalFocusChanged]: FocusedApp;
    [SeelenEvent.GlobalHistoryChanged]: ApplicationHistoryEntry[];
    [SeelenEvent.HistoryChangedOnMonitor]: ApplicationHistoryEntry[];
  }
}

export class ApplicationHistory extends List<ApplicationHistoryEntry> {
  static readonly getAsync = createInstanceInvoker(this, SeelenCommand.GetApplicationHistory);
  static readonly onChange = createInstanceOnEvent(this, SeelenEvent.GlobalHistoryChanged);
  static readonly onFocusChanged = (subscribtion: EventCallback<FocusedApp>) =>
    subscribe(SeelenEvent.GlobalFocusChanged, subscribtion);

  static readonly getCurrentMonitorHistoryAsync = createInstanceInvoker(
    this,
    SeelenCommand.GetApplicationHistoryOnMonitor,
  );
  static readonly onCurrentMonitorHistoryChanged = createInstanceOnEvent(this, SeelenEvent.HistoryChangedOnMonitor, {
    target: { kind: 'Webview', label: getCurrentWidget().rawLabel },
  });

  static readonly setLimitAsync = async function setLimitAsync(capacity: number) {
    await invoke(SeelenCommand.SetApplicationHistoryLimit, { capacity });
  };
}
