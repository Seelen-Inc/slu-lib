import { SeelenCommand, SeelenEvent } from '../handlers/mod.ts';
import { createInstanceInvoker, createInstanceOnEvent } from '../utils/State.ts';
import type { Color as IColor, UIColors as IUIColors } from '@seelen-ui/types';

export type { IColor, IUIColors };

declare global {
  interface ArgsByCommand {
    [SeelenCommand.SystemGetColors]: null;
    [SeelenCommand.SystemGetForegroundWindowColor]: null;
  }
  interface ReturnByCommand {
    [SeelenCommand.SystemGetColors]: IUIColors;
    [SeelenCommand.SystemGetForegroundWindowColor]: IColor;
  }
  interface PayloadByEvent {
    [SeelenEvent.ColorsChanged]: IUIColors;
  }
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

  setAsCssVariables(): void {
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

export class Color {
  constructor(public inner: IColor) {}

  private getDynamicStyleSheet(): HTMLStyleElement {
    const styleId = 'sl-lib-dynamic-color-variables';
    let styleElement = document.getElementById(styleId) as HTMLStyleElement;
    if (!styleElement) {
      styleElement = document.createElement('style');
      styleElement.id = styleId;
      styleElement.textContent = ':root {\n}';
      document.head.appendChild(styleElement);
    }
    return styleElement;
  }

  private insertIntoStyleSheet(obj: Record<string, string>): void {
    const sheet = this.getDynamicStyleSheet();
    const lines = sheet.textContent!.split('\n');
    lines.pop(); // remove the closing brace

    for (const [key, value] of Object.entries(obj)) {
      const old = lines.findIndex((line) => line.startsWith(key));
      if (old !== -1) {
        lines[old] = `${key}: ${value};`;
      } else {
        lines.push(`${key}: ${value};`);
      }
    }

    lines.push('}');
    sheet.textContent = lines.join('\n');
  }

  asHex(): string {
    return (
      '#' +
      this.inner.r.toString(16).padStart(2, '0') +
      this.inner.g.toString(16).padStart(2, '0') +
      this.inner.b.toString(16).padStart(2, '0') +
      this.inner.a.toString(16).padStart(2, '0')
    );
  }

  /**
   * @param name the name of the color
   * the name will be parsed to lower kebab case and remove non-alphanumeric characters
   * this will create some css variables as:\
   * `--color-{name}` -> #RRGGBBAA\
   * `--color-{name}-rgb` -> R, G, B
   * `--color-{name}-rgba` -> R, G, B, A
   */
  setAsCssVariable(name: string): void {
    const parsedName = name
      .replace('_', '-')
      .replace(/[^a-zA-Z0-9\-]/g, '')
      .toLowerCase();

    this.insertIntoStyleSheet({
      [`--color-${parsedName}`]: this.asHex(),
      [`--color-${parsedName}-rgb`]: `${this.inner.r}, ${this.inner.g}, ${this.inner.b}`,
      [`--color-${parsedName}-rgba`]: `${this.inner.r}, ${this.inner.g}, ${this.inner.b}, ${this.inner.a}`,
    });
  }

  /**
   * https://stackoverflow.com/questions/596216/formula-to-determine-perceived-brightness-of-rgb-color
   *
   * @param accuracy if true will use an expensive but more accurate algorithm
   * @returns a number between 0 and 255
   */
  calcLuminance(accuracy?: boolean): number {
    const { r, g, b } = this.inner;
    if (accuracy) {
      const gR = r ** 2.2;
      const gG = g ** 2.2;
      const gB = b ** 2.2;
      return (0.299 * gR + 0.587 * gG + 0.114 * gB) ** (1 / 2.2);
    }
    return 0.2126 * r + 0.7152 * g + 0.0722 * b;
  }
}
