// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { JsonValue } from './serde_json/JsonValue.ts';
import type { WidgetId } from './WidgetId.ts';

export type SettingsByWidget = { [key in WidgetId]?: { [key in string]?: JsonValue } };
