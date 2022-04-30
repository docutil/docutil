all: build-core build-js

debug: build-core-debug build-js

build-loader:
    #!/bin/sh
    cd loader
    test -d dist || npm i
    npm run build
    npm run install-wasm

build-default-theme:
    cd loader && sass theme/style.scss src/style.css

build-js: build-default-theme  build-loader

build-core:
    cd core && wasm-pack build --release --out-dir pkg --target web

build-core-debug:
    cd core && wasm-pack build --debug --out-dir pkg --target web


