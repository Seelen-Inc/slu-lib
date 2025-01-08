import { SeelenCommand, SeelenEvent } from '../lib.ts';
import { createInstanceInvoker, createInstanceInvokerWithArgs, createInstanceOnEvent } from '../utils/State.ts';
import { List } from '../utils/List.ts';
import { enumFromUnion } from '../utils/enums.ts';
import type { File, FolderChangedArgs, FolderType, User } from '@seelen-ui/types';

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
    [SeelenEvent.UserFolderChanged]: FolderChangedArgs;
  }
}

const FolderType = enumFromUnion<FolderType>({
  Unknown: 'Unknown',
  Recent: 'Recent',
  Downloads: 'Downloads',
  Documents: 'Documents',
  Pictures: 'Pictures',
  Videos: 'Videos',
  Music: 'Music',
});

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
    (folderArgs: FolderChangedArgs) => folderArgs.ofFolder == FolderType.Recent,
    (result: FolderChangedArgs) => result.content,
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
    (folderArgs: FolderChangedArgs) => folderArgs.ofFolder == FolderType.Downloads,
    (result: FolderChangedArgs) => result.content,
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
    (folderArgs: FolderChangedArgs) => folderArgs.ofFolder == FolderType.Documents,
    (result: FolderChangedArgs) => result.content,
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
    (folderArgs: FolderChangedArgs) => folderArgs.ofFolder == FolderType.Pictures,
    (result: FolderChangedArgs) => result.content,
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
    (folderArgs: FolderChangedArgs) => folderArgs.ofFolder == FolderType.Videos,
    (result: FolderChangedArgs) => result.content,
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
    (folderArgs: FolderChangedArgs) => folderArgs.ofFolder == FolderType.Music,
    (result: FolderChangedArgs) => result.content,
  );

  static readonly setFolderLimit = (amount: number) =>
    MusicFolder._setFolderLimit({ folderType: FolderType.Music, amount });

  static default(): MusicFolder {
    return new this([]);
  }
}
