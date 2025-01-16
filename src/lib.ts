import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import type { WidgetId } from '@seelen-ui/types';
import { decodeBase64Url } from '@std/encoding/base64url';

interface WidgetInformation {
  id: WidgetId;
  /** decoded webview label */
  label: string;
  /** base64 url encoded label (used as identifier of the webview) */
  rawLabel: string;
  params: Readonly<Record<string, string>>;
}

let CURRENT_WIDGET_INFORMATION: Readonly<WidgetInformation>;
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
  if (!CURRENT_WIDGET_INFORMATION) {
    CURRENT_WIDGET_INFORMATION = _getCurrentWidget();
  }
  return CURRENT_WIDGET_INFORMATION;
}

export * from './state/mod.ts';
export * from './system_state/mod.ts';
export * from './utils/mod.ts';
export * from './handlers/mod.ts';
