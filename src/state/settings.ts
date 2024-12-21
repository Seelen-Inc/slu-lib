import { SeelenCommand, SeelenEvent } from '../handlers/mod.ts';

import type {
  HideMode,
  SeelenLauncherMonitor,
  SeelenWegItemDisplayOption,
  SeelenWegMode,
  SeelenWegSide,
  Settings as ISettings,
  UpdateChannel,
  VirtualDesktopStrategy,
} from '@seelen-ui/types';
import { createInstanceInvoker, createInstanceOnEvent } from '../utils/State.ts';
import { enumFromUnion } from '../utils/enums.ts';

declare global {
  interface ArgsByCommand {
    [SeelenCommand.StateGetSettings]: null;
    [SeelenCommand.StateGetDefaultSettings]: null;
  }
  interface ReturnByCommand {
    [SeelenCommand.StateGetSettings]: ISettings;
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

const SeelenWegItemDisplayOption = enumFromUnion<SeelenWegItemDisplayOption>({
  PrimaryScreenAll: 'PrimaryScreenAll',
  AllOnAll: 'AllOnAll',
  Minimal: 'Minimal',
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

export {
  HideMode,
  SeelenLauncherMonitor,
  SeelenWegItemDisplayOption,
  SeelenWegMode,
  SeelenWegSide,
  UpdateChannel,
  VirtualDesktopStrategy,
};
