#!/bin/env bash

set -exo pipefail

rsw build

test -d node_modules || pnpm i

test -d dist && rm -rf dist
test -d example/dist && rm -rf example/dist

pnpm build
cp -r dist example

tar -czf docutil-dist.tar.gz dist
tar -czf docutil-example.tar.gz example