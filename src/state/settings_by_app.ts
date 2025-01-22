import type { AppConfig, AppExtraFlag, AppIdentifierType, MatchingStrategy } from '@seelen-ui/types';
import { List } from '../utils/List.ts';
import { SeelenCommand, SeelenEvent } from '../handlers/mod.ts';
import { createInstanceInvoker } from '../utils/State.ts';
import { createInstanceOnEvent } from '../utils/State.ts';
import { enumFromUnion } from '../utils/enums.ts';

declare global {
  interface ArgsByCommand {
    [SeelenCommand.StateGetSpecificAppsConfigurations]: null;
  }
  interface ReturnByCommand {
    [SeelenCommand.StateGetSpecificAppsConfigurations]: AppConfig[];
  }
  interface PayloadByEvent {
    [SeelenEvent.StateSettingsByAppChanged]: AppConfig[];
  }
}

export class AppConfigurationList extends List<AppConfig> {
  static readonly getAsync = createInstanceInvoker(this, SeelenCommand.StateGetSpecificAppsConfigurations);
  static readonly onChange = createInstanceOnEvent(this, SeelenEvent.StateSettingsByAppChanged);
}

// =================================================================================
//    From here some enums as helpers like @seelen-ui/types only contains types
// =================================================================================

const AppExtraFlag = enumFromUnion<AppExtraFlag>({
  Float: 'float',
  Force: 'force',
  Unmanage: 'unmanage',
  Pinned: 'pinned',
  Hidden: 'hidden',
});

const AppIdentifierType = enumFromUnion<AppIdentifierType>({
  Exe: 'Exe',
  Class: 'Class',
  Title: 'Title',
  Path: 'Path',
});

const MatchingStrategy = enumFromUnion<MatchingStrategy>({
  Equals: 'Equals',
  StartsWith: 'StartsWith',
  EndsWith: 'EndsWith',
  Contains: 'Contains',
  Regex: 'Regex',
});

export { AppExtraFlag, AppIdentifierType, MatchingStrategy };
