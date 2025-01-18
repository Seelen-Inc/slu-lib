import type { IconPack } from '@seelen-ui/types';
import { List } from '../utils/List.ts';
import { createInstanceInvoker, createInstanceOnEvent } from '../utils/State.ts';
import { invoke, SeelenCommand, SeelenEvent } from '../lib.ts';
import { path } from '@tauri-apps/api';
import { Settings } from './settings/mod.ts';
import { convertFileSrc } from '@tauri-apps/api/core';

export interface GetIconArgs {
  path?: string | null;
  umid?: string | null;
}

declare global {
  interface ArgsByCommand {
    [SeelenCommand.StateGetIconPacks]: null;
    [SeelenCommand.GetIcon]: GetIconArgs;
  }
  interface ReturnByCommand {
    [SeelenCommand.StateGetIconPacks]: IconPack;
    [SeelenCommand.GetIcon]: string | null;
  }
  interface PayloadByEvent {
    [SeelenEvent.StateIconPacksChanged]: IconPack;
  }
}

export class IconPackList extends List<IconPack> {
  static readonly getAsync = createInstanceInvoker(this, SeelenCommand.StateGetIconPacks);
  static readonly onChange = createInstanceOnEvent(this, SeelenEvent.StateIconPacksChanged);
}

/**
 * Class helper to allow easy use of icon packs
 */
export class IconPackManager {
  private callbacks: Set<() => void> = new Set();

  private constructor(
    private iconPackPath: string,
    private _iconPacks: IconPackList,
    private _actives: string[],
  ) {}

  public get iconPacks(): IconPackList {
    return this._iconPacks;
  }

  private set iconPacks(packs: IconPackList) {
    this._iconPacks = packs;
    this.callbacks.forEach((cb) => cb());
  }

  public get actives(): string[] {
    return this._actives;
  }

  private set actives(actives: string[]) {
    this._actives = actives;
    this.callbacks.forEach((cb) => cb());
  }

  /**
   * Creates an instance of IconPackManager. This intance will be updated when
   * the list of icon packs or the settings changes, so just having one global instance is enough.
   *
   * @returns A new instance of IconPackManager
   */
  public static async create(): Promise<IconPackManager> {
    const manager = new IconPackManager(
      await path.resolve(await path.appDataDir(), 'icons'),
      await IconPackList.getAsync(),
      (await Settings.getAsync()).inner.iconPacks,
    );
    IconPackList.onChange((list) => {
      manager.iconPacks = list;
    });
    Settings.onChange((settings) => {
      manager.actives = settings.inner.iconPacks;
    });
    return manager;
  }

  /**
   * Register a callback to be called when the list of active icon packs changes
   *
   * @param cb The callback to be called when the list of icon packs changes
   * @example
   * const manager = await IconPackManager.create();
   * manager.onChange(() => {
   *   console.log("Icon packs changed: ", manager.actives);
   * });
   */
  public onChange(cb: () => void): void {
    this.callbacks.add(cb);
  }

  /**
   * Return the icon URL for an app or file, in case of no icon available will return `null`
   *
   * @param filePath The path to the app could be umid, full path.
   * @example
   * const iconUrl = instance.getIcon({
   *   path: "C:\\Program Files\\Steam\\steam.exe"
   * });
   * const iconUrl = instance.getIcon({
   *   umid: "Seelen.SeelenUI_p6yyn03m1894e!App"
   * });
   */
  public getIcon({ path, umid }: GetIconArgs): string | null {
    if (!path && !umid) {
      return null;
    }

    for (const active of this.actives.toReversed()) {
      const pack = this._iconPacks.asArray().find((p) => p.info.filename === active);
      if (!pack) {
        continue;
      }

      let icon: string | undefined;
      if (umid) {
        icon = pack.apps[umid];
      }

      if (!icon && path) {
        const filename = path?.split(/[/\\]/g).pop();
        icon = filename ? pack.apps[filename] || pack.apps[path] : pack.apps[path];
      }

      if (icon) {
        return convertFileSrc(this.iconPackPath + '\\' + pack.info.filename + '\\' + icon);
      }
    }
    return null;
  }

  /**
   * Return the icon Path for an app or file, in case of no icon available will return `null`.
   * This method doesn't take in care icon packs, just extracts the inherited icon into system's icon pack.
   *
   * @param filePath The path to the app could be umid o full path
   * @example
   * const iconPath = instance.extractIcon({
   *   path: "C:\\Program Files\\Steam\\steam.exe"
   * });
   * const iconPath = instance.extractIcon({
   *   umid: "Seelen.SeelenUI_p6yyn03m1894e!App"
   * });
   */
  public static extractIcon(obj: GetIconArgs): Promise<string | null> {
    return invoke(SeelenCommand.GetIcon, obj);
  }
}
