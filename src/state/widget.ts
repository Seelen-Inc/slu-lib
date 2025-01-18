import type { Widget, WidgetId } from '@seelen-ui/types';
import { SeelenCommand, SeelenEvent } from '../lib.ts';
import { List } from '../utils/List.ts';
import { createInstanceInvoker, createInstanceOnEvent } from '../utils/State.ts';

declare global {
  interface ArgsByCommand {
    [SeelenCommand.StateGetWidgets]: null;
  }
  interface ReturnByCommand {
    [SeelenCommand.StateGetWidgets]: Widget[];
  }
  interface PayloadByEvent {
    [SeelenEvent.StateWidgetsChanged]: Widget[];
  }
}

export const SeelenWegWidgetId: WidgetId = '@seelen/weg';
export const SeelenToolbarWidgetId: WidgetId = '@seelen/fancy-toolbar';
export const SeelenWindowManagerWidgetId: WidgetId = '@seelen/window-manager';
export const SeelenLauncherWidgetId: WidgetId = '@seelen/launcher';
export const SeelenWallWidgetId: WidgetId = '@seelen/wallpaper-manager';

export class WidgetList extends List<Widget> {
  static readonly getAsync = createInstanceInvoker(this, SeelenCommand.StateGetWidgets);
  static readonly onChange = createInstanceOnEvent(this, SeelenEvent.StateWidgetsChanged);
}
