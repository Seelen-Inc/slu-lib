export * from './layered_hitbox.ts';

export class Rect {
  left = 0;
  top = 0;
  right = 0;
  bottom = 0;
}

export function disableWebviewShortcutsAndContextMenu(): void {
  globalThis.addEventListener('keydown', function (event): void {
    // prevent refresh
    if (event.key === 'F5') {
      event.preventDefault();
    }

    // prevent close
    if (event.altKey && event.key === 'F4') {
      event.preventDefault();
    }

    // others
    if (event.ctrlKey) {
      switch (event.key) {
        case 'r': // reload
        case 'f': // search
        case 'g': // find
        case 'p': // print
        case 'j': // downloads
        case 'u': // source
          event.preventDefault();
          break;
      }
    }
  });

  // prevent browser context menu
  globalThis.addEventListener('contextmenu', (e) => e.preventDefault(), {
    capture: true,
  });

  // prevent drop files in webview
  globalThis.addEventListener('drop', (e) => e.preventDefault());
  globalThis.addEventListener('dragover', (e) => e.preventDefault());

  // prevent dragging images
  globalThis.addEventListener('dragstart', (event) => {
    const target = event.target as HTMLElement | null;
    if (target?.tagName?.toLowerCase() === 'img') {
      event.preventDefault();
    }
  });
}
