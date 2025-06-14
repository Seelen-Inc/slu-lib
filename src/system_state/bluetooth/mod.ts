import { invoke, SeelenCommand, SeelenEvent, type UnSubscriber } from '../../handlers/mod.ts';
import { List } from '../../utils/List.ts';
import type { Enum } from '../../utils/enums.ts';
import type { BluetoothDevice, BluetoothDevicePairShowPinRequest, BluetoothMajorClass } from '@seelen-ui/types';
import type { UnlistenFn } from '@tauri-apps/api/event';
import { subscribe } from '../../handlers/mod.ts';
import { newFromInvoke, newOnEvent } from '../../utils/State.ts';

export class BluetoothDevices extends List<BluetoothDevice> {
  static getAsync(): Promise<BluetoothDevices> {
    return newFromInvoke(this, SeelenCommand.GetConnectedBluetoothDevices);
  }

  static onChange(cb: (user: BluetoothDevices) => void): Promise<UnSubscriber> {
    return newOnEvent(cb, this, SeelenEvent.BluetoothDevicesChanged);
  }

  static onDiscoveredDevicesChange(cb: (user: BluetoothDevices) => void): Promise<UnSubscriber> {
    return newOnEvent(cb, this, SeelenEvent.BluetoothDiscoveredDevicesChanged);
  }

  static async discover(): Promise<void> {
    return await invoke(SeelenCommand.StartBluetoothScanning);
  }
  static async stopDiscovery(): Promise<void> {
    return await invoke(SeelenCommand.StopBluetoothScanning);
  }

  static async pairDevice(address: bigint): Promise<void> {
    return await invoke(SeelenCommand.PairBluetoothDevice, { address });
  }
  static async forgetDevice(id: string): Promise<void> {
    return await invoke(SeelenCommand.ForgetBluetoothDevice, { id });
  }
  static async confirmPair(accept: boolean, passphrase: string): Promise<void> {
    return await invoke(SeelenCommand.ConfirmBluetoothDevicePair, { accept, passphrase });
  }

  static async onPairRequest(
    cb: (param: BluetoothDevicePairShowPinRequest | null) => void,
  ): Promise<UnlistenFn> {
    //TODO(Eythaan): from here the process does not continues
    const unlistenShowPin = await subscribe(SeelenEvent.BluetoothPairShowPin, (param) => cb(param.payload));
    const unlistenRequestPin = await subscribe(SeelenEvent.BluetoothPairRequestPin, (param) => cb(param.payload));

    return () => {
      unlistenRequestPin();
      unlistenShowPin();
    };
  }

  static default(): BluetoothDevices {
    return new this([]);
  }
}

const BluetoothMajor: Enum<BluetoothMajorClass> = {
  Miscellaneous: 'Miscellaneous',
  Computer: 'Computer',
  Phone: 'Phone',
  NetworkAccessPoint: 'NetworkAccessPoint',
  AudioVideo: 'AudioVideo',
  Peripheral: 'Peripheral',
  Imaging: 'Imaging',
  Wearable: 'Wearable',
  Toy: 'Toy',
  Health: 'Health',
  Uncategorized: 'Uncategorized',
};

export { BluetoothMajor };
