// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { StyleValue } from './StyleValue.ts';

export type TextToolbarItem = {
  /**
   * Id to identify the item, should be unique.
   */
  id: string;
  /**
   * Content to display in the item.
   *
   * Should follow the [mathjs expression syntax](https://mathjs.org/docs/expressions/syntax.html).
   *
   * ## Base Item Scope
   * Have all icons defined on [React Icons](https://react-icons.github.io/react-icons) as properties of the object.
   * ```js
   * const icon: object;
   * ```
   * Haves all environment variables defined on the system as properties of the object.
   * ```js
   * const env: object;
   * ```
   * Functions to add images to the item.
   * ```js
   * function getIcon(name: string, size: number = 16): string
   * function imgFromUrl (url: string, size: number = 16): string
   * function imgFromPath (path: string, size: number = 16): string
   * function imgFromExe (exe_path: string, size: number = 16): string
   * function t(path: string): string
   * ```
   */
  template: string;
  /**
   * Content to display in tooltip of the item.
   *
   * Should follow the [mathjs expression syntax](https://mathjs.org/docs/expressions/syntax.html).
   *
   * ## Base Item Scope
   * Have all icons defined on [React Icons](https://react-icons.github.io/react-icons) as properties of the object.
   * ```js
   * const icon: object;
   * ```
   * Haves all environment variables defined on the system as properties of the object.
   * ```js
   * const env: object;
   * ```
   * Functions to add images to the item.
   * ```js
   * function getIcon(name: string, size: number = 16): string
   * function imgFromUrl (url: string, size: number = 16): string
   * function imgFromPath (path: string, size: number = 16): string
   * function imgFromExe (exe_path: string, size: number = 16): string
   * function t(path: string): string
   * ```
   */
  tooltip: string | null;
  /**
   * Badge will be displayed over the item, useful as notifications.
   *
   * Should follow the [mathjs expression syntax](https://mathjs.org/docs/expressions/syntax.html).
   *
   * ## Base Item Scope
   * Have all icons defined on [React Icons](https://react-icons.github.io/react-icons) as properties of the object.
   * ```js
   * const icon: object;
   * ```
   * Haves all environment variables defined on the system as properties of the object.
   * ```js
   * const env: object;
   * ```
   * Functions to add images to the item.
   * ```js
   * function getIcon(name: string, size: number = 16): string
   * function imgFromUrl (url: string, size: number = 16): string
   * function imgFromPath (path: string, size: number = 16): string
   * function imgFromExe (exe_path: string, size: number = 16): string
   * function t(path: string): string
   * ```
   */
  badge: string | null;
  /**
   * Deprecated use `onClickV2` instead.
   */
  onClick: string | null;
  /**
   * This code will be parsed and executed when the item is clicked.
   *
   * Should follow the [mathjs expression syntax](https://mathjs.org/docs/expressions/syntax.html).
   *
   * ## Base Item Scope
   * Have all icons defined on [React Icons](https://react-icons.github.io/react-icons) as properties of the object.
   * ```js
   * const icon: object;
   * ```
   * Haves all environment variables defined on the system as properties of the object.
   * ```js
   * const env: object;
   * ```
   * Functions to add images to the item.
   * ```js
   * function getIcon(name: string, size: number = 16): string
   * function imgFromUrl (url: string, size: number = 16): string
   * function imgFromPath (path: string, size: number = 16): string
   * function imgFromExe (exe_path: string, size: number = 16): string
   * function t(path: string): string
   * ```
   */
  onClickV2: string | null;
  /**
   * Styles to be added to the item. This follow the same interface of React's `style` prop.
   */
  style: { [key in string]?: StyleValue | null };
};
