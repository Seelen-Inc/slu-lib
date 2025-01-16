import type { UnlistenFn } from '@tauri-apps/api/event';
import { invoke, type SeelenCommand, type SeelenEvent, subscribe } from '../lib.ts';
import type { Options as ListenerOptions } from '@tauri-apps/api/event';

interface ConstructorWithSingleArg {
  // deno-lint-ignore no-explicit-any
  new (inner: any): any;
}

type InstanceInvoker<Instance> = () => Promise<Instance>;

export function createInstanceInvoker<This extends ConstructorWithSingleArg>(
  Class: This,
  command: SeelenCommand,
): InstanceInvoker<InstanceType<This>> {
  return async () => {
    return new Class(await invoke(command));
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
