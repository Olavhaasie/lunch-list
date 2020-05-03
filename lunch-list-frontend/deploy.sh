#!/usr/bin/env sh

set -e

wasm-pack build --target web
# This step is necessary until wasm-pack has an option for this
rollup ./main.js --format iife --file ./pkg/bundle.js

cp -r static .. 
cp -r pkg ../static

