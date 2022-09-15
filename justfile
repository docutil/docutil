all: build-core build-js

debug: build-core-debug build-js

build-loader:
    #!/bin/sh
    cd loader
    test -d dist || npm i
    npm run build
    npm run install-wasm

build-default-theme:
    #!/bin/sh
    cd loader
    npm run styles

build-js: build-default-theme  build-loader

build-core:
    cd core && wasm-pack build --release --out-dir pkg --target web

build-core-debug:
    cd core && wasm-pack build --dev --out-dir pkg --target web

make-example-site-package:
    #!/bin/sh
    test -d mysite && rm -rf mysite
    mkdir -p mysite
    cp -r loader/dist mysite
    cp loader/index.html mysite
    cp loader/README.md mysite
    cp loader/SIDEBAR.md mysite
    cp -r loader/docs mysite
    tar -czvf docutil-example-site.tar.gz mysite

make-dist-package:
    #!/bin/sh
    cd loader
    tar -czvf ../docutil-dist.tar.gz dist

make-package: make-example-site-package make-dist-package

