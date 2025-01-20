import { SeelenCommand, SeelenEvent } from '../../handlers/mod.ts';

import type {
  FancyToolbarSettings,
  HideMode,
  SeelenLauncherMonitor,
  SeelenLauncherSettings,
  SeelenWallSettings,
  SeelenWegMode,
  SeelenWegSettings,
  SeelenWegSide,
  Settings as ISettings,
  UpdateChannel,
  VirtualDesktopStrategy,
  WidgetId,
  WindowManagerSettings,
} from '@seelen-ui/types';
import { createInstanceInvoker, createInstanceOnEvent } from '../../utils/State.ts';
import { enumFromUnion } from '../../utils/enums.ts';
import { invoke } from '../../lib.ts';
import {
  SeelenLauncherWidgetId,
  SeelenToolbarWidgetId,
  SeelenWallWidgetId,
  SeelenWegWidgetId,
  SeelenWindowManagerWidgetId,
} from '../widget.ts';

declare global {
  interface ArgsByCommand {
    [SeelenCommand.StateGetSettings]: { path?: string };
    [SeelenCommand.StateWriteSettings]: { settings: ISettings };
    [SeelenCommand.StateGetDefaultSettings]: null;
  }
  interface ReturnByCommand {
    [SeelenCommand.StateGetSettings]: ISettings;
    [SeelenCommand.StateWriteSettings]: void;
    [SeelenCommand.StateGetDefaultSettings]: ISettings;
  }
  interface PayloadByEvent {
    [SeelenEvent.StateSettingsChanged]: ISettings;
  }
}

export class Settings {
  constructor(public inner: ISettings) {}
  static readonly default = createInstanceInvoker(this, SeelenCommand.StateGetDefaultSettings);
  static readonly getAsync = createInstanceInvoker(this, SeelenCommand.StateGetSettings);
  static readonly onChange = createInstanceOnEvent(this, SeelenEvent.StateSettingsChanged);

  static async loadCustom(path: string): Promise<Settings> {
    return new this(await invoke(SeelenCommand.StateGetSettings, { path }));
  }

  getWidgetConfig(id: WidgetId): Record<string, unknown> | undefined {
    return this.inner.byWidget[id];
  }

  private getBundledWidgetConfig<T extends Record<string, unknown>>(id: WidgetId): T {
    const config = this.inner.byWidget[id];
    if (!config) throw new Error('Bundled widget settings not found');
    return config! as T;
  }

  get fancyToolbar(): FancyToolbarSettings {
    return this.getBundledWidgetConfig(SeelenToolbarWidgetId);
  }

  get seelenweg(): SeelenWegSettings {
    return this.getBundledWidgetConfig(SeelenWegWidgetId);
  }

  get windowManager(): WindowManagerSettings {
    return this.getBundledWidgetConfig(SeelenWindowManagerWidgetId);
  }

  get launcher(): SeelenLauncherSettings {
    return this.getBundledWidgetConfig(SeelenLauncherWidgetId);
  }

  get wall(): SeelenWallSettings {
    return this.getBundledWidgetConfig(SeelenWallWidgetId);
  }

  /** Will store the settings on disk */
  save(): Promise<void> {
    return invoke(SeelenCommand.StateWriteSettings, { settings: this.inner });
  }
}

// =================================================================================
//    From here some enums as helpers like @seelen-ui/types only contains types
// =================================================================================

const VirtualDesktopStrategy = enumFromUnion<VirtualDesktopStrategy>({
  Native: 'Native',
  Seelen: 'Seelen',
});

const SeelenWegMode = enumFromUnion<SeelenWegMode>({
  FullWidth: 'FullWidth',
  MinContent: 'MinContent',
});

const HideMode = enumFromUnion<HideMode>({
  Never: 'Never',
  Always: 'Always',
  OnOverlap: 'OnOverlap',
});

const SeelenWegSide = enumFromUnion<SeelenWegSide>({
  Left: 'Left',
  Right: 'Right',
  Top: 'Top',
  Bottom: 'Bottom',
});

const SeelenLauncherMonitor = enumFromUnion<SeelenLauncherMonitor>({
  Primary: 'Primary',
  MouseOver: 'MouseOver',
});

const UpdateChannel = enumFromUnion<UpdateChannel>({
  Release: 'Release',
  Beta: 'Beta',
  Nightly: 'Nightly',
});

export { HideMode, SeelenLauncherMonitor, SeelenWegMode, SeelenWegSide, UpdateChannel, VirtualDesktopStrategy };
export * from './settings_by_monitor.ts';
export * from './declaration.ts';
