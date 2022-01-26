all: build-core build-default-theme build-loader

build-loader:
    #!/bin/sh
    cd loader
    test -d dist || npm i
    npm run build
    npm run install-wasm

build-core:
    cd core && wasm-pack build --release --out-dir pkg --target web

build-default-theme:
    cd loader && sass theme/style.scss src/style.css
