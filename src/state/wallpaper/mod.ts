import { List } from '../../utils/List.ts';
import type { Wallpaper as IWallpaper } from '@seelen-ui/types';
import { newFromInvoke, newOnEvent } from '../../utils/State.ts';
import { SeelenCommand, SeelenEvent, type UnSubscriber } from '../../handlers/mod.ts';
import { Wrapper } from '../../utils/Wrapper.ts';
import type { WallpaperInstanceSettings } from '../../types/WallpaperInstanceSettings.ts';

export const SUPPORTED_IMAGE_WALLPAPER_EXTENSIONS = [
  'apng',
  'avif',
  'gif',
  'jpg',
  'jpeg',
  'png',
  'svg',
  'webp',
  'bmp',
  'ico',
  'tiff',
];

export const SUPPORTED_VIDEO_WALLPAPER_EXTENSIONS = ['mp4', 'webm', 'ogg', 'avi', 'mov', 'mkv', 'mpeg'];

export class WallpaperList extends List<IWallpaper> {
  static getAsync(): Promise<WallpaperList> {
    return newFromInvoke(this, SeelenCommand.StateGetWallpapers);
  }

  static onChange(cb: (user: WallpaperList) => void): Promise<UnSubscriber> {
    return newOnEvent(cb, this, SeelenEvent.StateWallpapersChanged);
  }
}

export class WallpaperConfiguration extends Wrapper<WallpaperInstanceSettings> {
  static default(): Promise<WallpaperConfiguration> {
    return newFromInvoke(this, SeelenCommand.StateGetDefaultWallpaperSettings);
  }
}
