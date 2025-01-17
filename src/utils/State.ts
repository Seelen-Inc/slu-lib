import type { UnlistenFn } from '@tauri-apps/api/event';
import { invoke, type SeelenCommand, type SeelenEvent, subscribe } from '../lib.ts';
import type { Options as ListenerOptions } from '@tauri-apps/api/event';

interface ConstructorWithSingleArg {
  // deno-lint-ignore no-explicit-any
  new (inner: any): any;
}

type InstanceInvoker<Instance> = () => Promise<Instance>;
type InstanceInvokerWithArg<Instance, Command extends SeelenCommand> = (
  args: SeelenCommandArg<Command>,
) => Promise<Instance>;
export type SeelenCommandArg<T extends SeelenCommand> = NonNullable<ArgsByCommand[T]>;

export function createInstanceInvoker<This extends ConstructorWithSingleArg>(
  Class: This,
  command: SeelenCommand,
): InstanceInvoker<InstanceType<This>> {
  return async () => {
    return new Class(await invoke(command));
  };
}

export function createInstanceInvokerWithArgs<
  This extends ConstructorWithSingleArg,
  Command extends SeelenCommand,
>(
  Class: This,
  command: Command,
): InstanceInvokerWithArg<InstanceType<This>, Command> {
  return async (args: NonNullable<ArgsByCommand[Command]>) => {
    return new Class(await invoke(command, args));
  };
}

type InstanceOnEvent<Instance> = (cb: (instance: Instance) => void) => Promise<UnlistenFn>;

export function createInstanceOnEvent<This extends ConstructorWithSingleArg>(
  Class: This,
  event: SeelenEvent,
  options?: ListenerOptions,
): InstanceOnEvent<InstanceType<This>> {
  return (cb: (instance: InstanceType<This>) => void) => {
    return subscribe(event, (eventData) => {
      cb(new Class(eventData.payload));
    }, options);
  };
}
