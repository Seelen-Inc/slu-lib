/// <reference lib="deno.ns" />

import { build, type BuildOptions, emptyDir } from 'jsr:@deno/dnt@0.41.3';

import denoJson from '../deno.json' with { type: 'json' };

const { name, description, version, license } = denoJson;
const packageJson: BuildOptions['package'] = {
  name,
  description,
  version,
  license,
  repository: {
    type: 'git',
    url: 'git+https://github.com/Seelen-Inc/slu-lib.git',
  },
  bugs: {
    url: 'https://github.com/Seelen-Inc/slu-lib/issues',
  },
};

await emptyDir('./npm');

await build({
  typeCheck: 'both',
  compilerOptions: {
    lib: ['DOM', 'DOM.Iterable', 'ESNext'],
    target: 'ES2023',
  },
  test: false,
  entryPoints: [{
    name: '.',
    path: './src/lib.ts',
  }, {
    name: './types',
    path: './src/types/mod.ts',
  }, {
    name: './tauri',
    path: './src/re-exports/tauri.ts',
  }],
  outDir: './npm',
  shims: {},
  importMap: 'deno.json',
  package: packageJson,
  postBuild(): void {
    Deno.copyFileSync('LICENSE', 'npm/LICENSE');
    Deno.copyFileSync('readme.md', 'npm/readme.md');
  },
});
