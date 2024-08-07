import { build } from "esbuild";
import { lessLoader } from "esbuild-plugin-less";
import fs from "node:fs";

function installWasmFile() {
	const artifact = "core/pkg/core_bg.wasm";
	const target = "dist/core_bg.wasm";
	if (fs.existsSync(artifact)) {
		fs.copyFileSync(artifact, target);
	}
}

build({
	entryPoints: {
		index: "src/index.js",
	},
	bundle: true,
	minify: true,
	outfile: "dist/index.mjs",
	format: "esm",
	plugins: [lessLoader()],
	loader: {
		".svg": "dataurl",
		".wasm": "file",
	},
}).then(() => {
	installWasmFile();
});
