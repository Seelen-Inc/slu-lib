import type { Widget, WidgetId } from '@seelen-ui/types';
import { SeelenCommand, SeelenEvent } from '../handlers/mod.ts';
import { List } from '../utils/List.ts';
import { createInstanceInvoker, createInstanceOnEvent } from '../utils/State.ts';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { decodeBase64Url } from '@std/encoding/base64url';

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

interface WidgetInformation {
  id: WidgetId;
  /** decoded webview label */
  label: string;
  /** base64 url encoded label (used as identifier of the webview) */
  rawLabel: string;
  params: Readonly<Record<string, string>>;
}

const CURRENT_WIDGET_INFORMATION: { ref: Readonly<WidgetInformation> | null } = {
  ref: null,
};
function _getCurrentWidget(): Readonly<WidgetInformation> {
  const encodedLabel = getCurrentWebviewWindow().label;
  const decodedLabel = new TextDecoder().decode(decodeBase64Url(encodedLabel));
  const [id, query] = decodedLabel.split('?');
  const params = new URLSearchParams(query);

  return Object.freeze({
    id: id as WidgetId,
    label: decodedLabel,
    rawLabel: encodedLabel,
    params: Object.freeze(Object.fromEntries(params)),
  });
}

export function getCurrentWidget(): Readonly<WidgetInformation> {
  if (!CURRENT_WIDGET_INFORMATION.ref) {
    CURRENT_WIDGET_INFORMATION.ref = _getCurrentWidget();
  }
  return CURRENT_WIDGET_INFORMATION.ref;
}
