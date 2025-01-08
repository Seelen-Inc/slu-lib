import { SeelenCommand, SeelenEvent } from '../lib.ts';
import { createInstanceInvoker, createInstanceInvokerWithArgs, createInstanceOnEvent } from '../utils/State.ts';
import { List } from '../utils/List.ts';

declare global {
  interface ArgsByCommand {
    [SeelenCommand.GetUser]: null;
    [SeelenCommand.GetUserFolderContent]: { folderType: FolderType };
    [SeelenCommand.SetUserFolderLimit]: { folderType: FolderType; amount: number };
  }
  interface ReturnByCommand {
    [SeelenCommand.GetUser]: User;
    [SeelenCommand.GetUserFolderContent]: File[];
    [SeelenCommand.SetUserFolderLimit]: null;
  }
  interface PayloadByEvent {
    [SeelenEvent.UserChanged]: User;
    [SeelenEvent.UserFolderChanged]: UserFolderContent;
  }
}

export interface UserFolderContent {
  ofFolder: FolderType;
  content: File[] | undefined;
}

export interface User {
  name: string;
  domain: string;
  profileHomePath: string;
  email: string | null;
  oneDrivePath: string | null;
  profilePicturePath: string;
}

export interface File {
  path: string;
  lastAccessTime: number;
}

export enum FolderType {
  Recent = 'Recent',
  Downloads = 'Downloads',
  Documents = 'Documents',
  Pictures = 'Pictures',
  Videos = 'Videos',
  Music = 'Music',
}

export class UserDetails {
  constructor(public user: User) {}

  static readonly getAsync = createInstanceInvoker(this, SeelenCommand.GetUser);
  static readonly onChange = createInstanceOnEvent(this, SeelenEvent.UserChanged);
}

export class RecentFolder extends List<File> {
  private static readonly _getAsync = createInstanceInvokerWithArgs(this, SeelenCommand.GetUserFolderContent);
  private static readonly _setFolderLimit = createInstanceInvokerWithArgs(this, SeelenCommand.SetUserFolderLimit);

  static readonly getAsync = () => RecentFolder._getAsync({ folderType: FolderType.Recent });
  static readonly onChange = createInstanceOnEvent(
    this,
    SeelenEvent.UserFolderChanged,
    (folderArgs: UserFolderContent) => folderArgs.ofFolder == FolderType.Recent,
  );

  static readonly setFolderLimit = (amount: number) =>
    RecentFolder._setFolderLimit({ folderType: FolderType.Recent, amount });

  static default(): RecentFolder {
    return new this([]);
  }
}

export class DownloadsFolder extends List<File> {
  private static readonly _getAsync = createInstanceInvokerWithArgs(this, SeelenCommand.GetUserFolderContent);
  private static readonly _setFolderLimit = createInstanceInvokerWithArgs(this, SeelenCommand.SetUserFolderLimit);

  static readonly getAsync = () => DownloadsFolder._getAsync({ folderType: FolderType.Downloads });
  static readonly onChange = createInstanceOnEvent(
    this,
    SeelenEvent.UserFolderChanged,
    (folderArgs: UserFolderContent) => folderArgs.ofFolder == FolderType.Downloads,
  );

  static readonly setFolderLimit = (amount: number) =>
    DownloadsFolder._setFolderLimit({ folderType: FolderType.Downloads, amount });

  static default(): DownloadsFolder {
    return new this([]);
  }
}

export class DocumentsFolder extends List<File> {
  private static readonly _getAsync = createInstanceInvokerWithArgs(this, SeelenCommand.GetUserFolderContent);
  private static readonly _setFolderLimit = createInstanceInvokerWithArgs(this, SeelenCommand.SetUserFolderLimit);

  static readonly getAsync = () => DocumentsFolder._getAsync({ folderType: FolderType.Documents });
  static readonly onChange = createInstanceOnEvent(
    this,
    SeelenEvent.UserFolderChanged,
    (folderArgs: UserFolderContent) => folderArgs.ofFolder == FolderType.Documents,
  );

  static readonly setFolderLimit = (amount: number) =>
    DocumentsFolder._setFolderLimit({ folderType: FolderType.Documents, amount });

  static default(): DocumentsFolder {
    return new this([]);
  }
}

export class PicturesFolder extends List<File> {
  private static readonly _getAsync = createInstanceInvokerWithArgs(this, SeelenCommand.GetUserFolderContent);
  private static readonly _setFolderLimit = createInstanceInvokerWithArgs(this, SeelenCommand.SetUserFolderLimit);

  static readonly getAsync = () => PicturesFolder._getAsync({ folderType: FolderType.Pictures });
  static readonly onChange = createInstanceOnEvent(
    this,
    SeelenEvent.UserFolderChanged,
    (folderArgs: UserFolderContent) => folderArgs.ofFolder == FolderType.Pictures,
  );

  static readonly setFolderLimit = (amount: number) =>
    PicturesFolder._setFolderLimit({ folderType: FolderType.Pictures, amount });

  static default(): PicturesFolder {
    return new this([]);
  }
}

export class VideosFolder extends List<File> {
  private static readonly _getAsync = createInstanceInvokerWithArgs(this, SeelenCommand.GetUserFolderContent);
  private static readonly _setFolderLimit = createInstanceInvokerWithArgs(this, SeelenCommand.SetUserFolderLimit);

  static readonly getAsync = () => VideosFolder._getAsync({ folderType: FolderType.Videos });
  static readonly onChange = createInstanceOnEvent(
    this,
    SeelenEvent.UserFolderChanged,
    (folderArgs: UserFolderContent) => folderArgs.ofFolder == FolderType.Videos,
  );

  static readonly setFolderLimit = (amount: number) =>
    VideosFolder._setFolderLimit({ folderType: FolderType.Videos, amount });

  static default(): VideosFolder {
    return new this([]);
  }
}

export class MusicFolder extends List<File> {
  private static readonly _getAsync = createInstanceInvokerWithArgs(this, SeelenCommand.GetUserFolderContent);
  private static readonly _setFolderLimit = createInstanceInvokerWithArgs(this, SeelenCommand.SetUserFolderLimit);

  static readonly getAsync = () => MusicFolder._getAsync({ folderType: FolderType.Music });
  static readonly onChange = createInstanceOnEvent(
    this,
    SeelenEvent.UserFolderChanged,
    (folderArgs: UserFolderContent) => folderArgs.ofFolder == FolderType.Music,
  );

  static readonly setFolderLimit = (amount: number) =>
    MusicFolder._setFolderLimit({ folderType: FolderType.Music, amount });

  static default(): MusicFolder {
    return new this([]);
  }
}
