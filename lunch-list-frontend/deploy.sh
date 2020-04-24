#!/usr/bin/env sh

deploy_dir="../target/deploy"

mkdir -p "$deploy_dir"
cp -r index.html main.js pkg "$deploy_dir"

