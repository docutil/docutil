import { build } from 'esbuild';
import { lessLoader } from 'esbuild-plugin-less';
import { wasmLoader } from 'esbuild-plugin-wasm';
import fs from 'fs';

function installWasmFile() {
  const artifact = '@docutil/core/pkg/core_bg.wasm';
  const target = 'dist/core_bg.wasm';
  if (fs.existsSync(artifact)) {
    fs.copyFileSync(artifact, target);
  }
}

build({
  entryPoints: {
    index: 'src/index.js',
  },
  bundle: true,
  minify: true,
  outfile: 'dist/index.mjs',
  format: 'esm',
  plugins: [lessLoader(), wasmLoader()],
  loader: {
    '.svg': 'dataurl',
  },
}).then(() => {
  installWasmFile();
});
