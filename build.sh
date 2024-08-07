#!/bin/env bash

set -exo pipefail

wasm-pack build --release core --mode no-install --scope docutil --target web

test -d node_modules || bun i

test -d dist && rm -rf dist
test -d example/dist && rm -rf example/dist

bun run build.mjs
cp -r dist example

tar -czf docutil-dist.tar.gz dist
tar -czf docutil-example.tar.gz example