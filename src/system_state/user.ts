import { SeelenCommand, SeelenEvent } from '../lib.ts';
import { createInstanceInvoker, createInstanceInvokerWithArgs, createInstanceOnEvent } from '../utils/State.ts';
import { List } from '../utils/List.ts';

declare global {
  interface ArgsByCommand {
    [SeelenCommand.GetUser]: null;
    [SeelenCommand.GetUserRecentFolderContent]: null;
    [SeelenCommand.SetUserRecentFolderLimit]: { amount: number };
  }
  interface ReturnByCommand {
    [SeelenCommand.GetUser]: User;
    [SeelenCommand.GetUserRecentFolderContent]: RecentFile[];
    [SeelenCommand.SetUserRecentFolderLimit]: null;
  }
  interface PayloadByEvent {
    [SeelenEvent.UserChanged]: User;
    [SeelenEvent.UserRecentFolderChanged]: RecentFile[];
  }
}

export interface User {
  name: string;
  domain: string;
  profileHomePath: string;
  email: string | null;
  oneDrivePath: string | null;
  profilePicturePath: string;
}

export interface RecentFile {
  name: string;
  path: string;
  iconLocation: string;
  lastAccessTime: number;
}

export class UserDetails {
  constructor(public user: User) {}

  static readonly getAsync = createInstanceInvoker(this, SeelenCommand.GetUser);
  static readonly onChange = createInstanceOnEvent(this, SeelenEvent.UserChanged);
}

export class RecentFolder extends List<RecentFile> {
  static readonly getAsync = createInstanceInvoker(this, SeelenCommand.GetUserRecentFolderContent);
  static readonly onChange = createInstanceOnEvent(this, SeelenEvent.UserRecentFolderChanged);

  static readonly setRecentFolderLimit = createInstanceInvokerWithArgs(this, SeelenCommand.SetUserRecentFolderLimit);

  static default(): RecentFolder {
    return new this([]);
  }
}
