import { SeelenCommand, SeelenEvent } from '../../handlers/mod.ts';
import { List } from '../../utils/List.ts';
import { createInstanceInvoker, createInstanceOnEvent } from '../../utils/State.ts';
import type { Plugin } from '@seelen-ui/types';
import { getCurrentWidget } from '../widget.ts';

export class PluginList extends List<Plugin> {
  static readonly getAsync = createInstanceInvoker(this, SeelenCommand.StateGetPlugins);
  static readonly onChange = createInstanceOnEvent(this, SeelenEvent.StatePluginsChanged);

  forCurrentWidget(): Plugin[] {
    const target = getCurrentWidget().id;
    return this.inner.filter((plugin) => plugin.target === target);
  }
}
