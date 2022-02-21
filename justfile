all: build-core build-js

debug: build-core-debug build-js

build-loader:
    #!/bin/sh
    cd loader
    test -d dist || npm i
    npm run build
    npm run install-wasm

build-foxtail-docutil:
    #!/bin/sh
    cd foxtail-docutil
    test -d dist || npm i
    npm run build

install-foxtail-docutil:
    find foxtail-docutil/dist/*.js | xargs -i cp {} loader/dist/

build-default-theme:
    cd loader && sass theme/style.scss src/style.css

build-js: build-default-theme build-foxtail-docutil build-loader

build-core:
    cd core && wasm-pack build --release --out-dir pkg --target web

build-core-debug:
    cd core && wasm-pack build --out-dir pkg --target web


