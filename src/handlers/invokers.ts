import { invoke as tauriInvoke, type InvokeOptions } from '@tauri-apps/api/core';
import { type EventCallback, listen, type Options as ListenerOptions, type UnlistenFn } from '@tauri-apps/api/event';

import type { SeelenCommand } from './commands.ts';
import type { SeelenEvent } from './events.ts';

export * from './commands.ts';

declare global {
  interface ArgsByCommand extends Record<SeelenCommand, unknown> {
    [SeelenCommand.IsDevMode]: null;
    [SeelenCommand.CheckForUpdates]: null;
    [SeelenCommand.InstallLastAvailableUpdate]: null;
  }

  interface ReturnByCommand extends Record<SeelenCommand, unknown> {
    [SeelenCommand.IsDevMode]: boolean;
    [SeelenCommand.CheckForUpdates]: boolean;
    [SeelenCommand.InstallLastAvailableUpdate]: never; // this will close the app
  }

  interface PayloadByEvent extends Record<SeelenEvent, unknown> {}
}

/**
 * Will call to the background process
 * @return Result of the command
 */
export function invoke<T extends SeelenCommand>(
  command: T,
  args?: NonNullable<ArgsByCommand[T]>,
  options?: InvokeOptions,
): Promise<ReturnByCommand[T]> {
  return tauriInvoke(command, args, options);
}

export function subscribe<T extends SeelenEvent>(
  event: T,
  cb: EventCallback<PayloadByEvent[T]>,
  options?: ListenerOptions,
): Promise<UnlistenFn> {
  return listen(event, cb, options);
}
