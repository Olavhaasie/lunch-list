#!/usr/bin/env sh

set -e

deploy_dir="../target/deploy"

wasm-pack build --target web
# This step is necessary until wasm-pack has an option for this
rollup ./main.js --format iife --file ./pkg/bundle.js

mkdir -p "$deploy_dir"
cp -r index.html main.js pkg "$deploy_dir"

