import { SeelenCommand, SeelenEvent } from '../lib.ts';
import { List } from '../utils/List.ts';
import { createInstanceInvoker, createInstanceOnEvent } from '../utils/State.ts';

declare global {
  interface ArgsByCommand {
    [SeelenCommand.SystemGetLanguages]: null;
  }
  interface ReturnByCommand {
    [SeelenCommand.SystemGetLanguages]: SystemLanguage[];
  }
  interface PayloadByEvent {
    [SeelenEvent.SystemLanguagesChanged]: SystemLanguage[];
  }
}

export interface KeyboardLayout {
  id: string;
  displayName: string;
}

export interface SystemLanguage {
  code: string;
  name: string;
  inputMethods: KeyboardLayout[];
}

export class LanguageList extends List<SystemLanguage> {
  static readonly getAsync = createInstanceInvoker(this, SeelenCommand.SystemGetLanguages);
  static readonly onChange = createInstanceOnEvent(this, SeelenEvent.SystemLanguagesChanged);
}
