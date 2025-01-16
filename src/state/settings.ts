import { SeelenCommand, SeelenEvent } from '../handlers/mod.ts';

import type {
  HideMode,
  SeelenLauncherMonitor,
  SeelenWegMode,
  SeelenWegSide,
  Settings as ISettings,
  UpdateChannel,
  VirtualDesktopStrategy,
} from '@seelen-ui/types';
import { createInstanceInvoker, createInstanceOnEvent } from '../utils/State.ts';
import { enumFromUnion } from '../utils/enums.ts';
import { invoke } from '../lib.ts';

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
