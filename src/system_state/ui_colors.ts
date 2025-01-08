import { SeelenCommand, SeelenEvent } from '../handlers/mod.ts';
import { createInstanceInvoker, createInstanceOnEvent } from '../utils/State.ts';

declare global {
  interface ArgsByCommand {
    [SeelenCommand.SystemGetColors]: null;
  }
  interface ReturnByCommand {
    [SeelenCommand.SystemGetColors]: IUIColors;
  }
  interface PayloadByEvent {
    [SeelenEvent.ColorsChanged]: IUIColors;
  }
}

export interface IUIColors {
  background: string;
  foreground: string;
  accent_darkest: string;
  accent_darker: string;
  accent_dark: string;
  accent: string;
  accent_light: string;
  accent_lighter: string;
  accent_lightest: string;
  complement: string | null;
}

export class UIColors {
  constructor(public inner: IUIColors) {}

  static readonly getAsync = createInstanceInvoker(this, SeelenCommand.SystemGetColors);
  static readonly onChange = createInstanceOnEvent(this, SeelenEvent.ColorsChanged);

  static default(): UIColors {
    return new this({
      background: '#ffffff',
      foreground: '#000000',
      accent_darkest: '#990000',
      accent_darker: '#aa0000',
      accent_dark: '#bb0000',
      accent: '#cc0000',
      accent_light: '#dd0000',
      accent_lighter: '#ee0000',
      accent_lightest: '#ff0000',
      complement: null,
    });
  }

  setAssCssVariables(): void {
    for (const [key, value] of Object.entries(this.inner)) {
      if (typeof value !== 'string') {
        continue;
      }
      const hex = value.replace('#', '').slice(0, 6);
      const color = parseInt(hex, 16);
      const r = (color >> 16) & 255;
      const g = (color >> 8) & 255;
      const b = color & 255;
      // replace rust snake case with kebab case
      const name = key.replace('_', '-');
      document.documentElement.style.setProperty(`--config-${name}-color`, value.slice(0, 7));
      document.documentElement.style.setProperty(`--config-${name}-color-rgb`, `${r}, ${g}, ${b}`);
    }
  }
}
