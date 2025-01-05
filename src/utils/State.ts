import type { UnlistenFn } from '@tauri-apps/api/event';
import { invoke, type SeelenCommand, type SeelenEvent, subscribe } from '../lib.ts';

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
export type SeelenEventArg<T extends SeelenEvent> = PayloadByEvent[T];

export function createInstanceOnEvent<This extends ConstructorWithSingleArg, Event extends SeelenEvent>(
  Class: This,
  event: Event,
  filter?: (args: SeelenEventArg<Event>) => boolean,
): InstanceOnEvent<InstanceType<This>> {
  return (cb: (instance: InstanceType<This>) => void) => {
    return subscribe(event, (eventData) => {
      if (!filter || filter(eventData.payload)) {
        cb(new Class(eventData.payload));
      }
    });
  };
}
