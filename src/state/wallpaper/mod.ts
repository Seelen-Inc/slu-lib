import { List } from '../../utils/List.ts';
import type { Wallpaper } from '@seelen-ui/types';
import { newFromInvoke, newOnEvent } from '../../utils/State.ts';
import { SeelenCommand, SeelenEvent, type UnSubscriber } from '../../handlers/mod.ts';

export class WallpaperList extends List<Wallpaper> {
  static getAsync(): Promise<WallpaperList> {
    return newFromInvoke(this, SeelenCommand.StateGetWallpapers);
  }

  static onChange(cb: (user: WallpaperList) => void): Promise<UnSubscriber> {
    return newOnEvent(cb, this, SeelenEvent.StateWallpapersChanged);
  }
}

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
