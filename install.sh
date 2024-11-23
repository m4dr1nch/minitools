#!/bin/bash

tool_dirs=$(dir -d */)

cwd=$(pwd)

for tool_dir in $tool_dirs; do
    cd $tool_dir;

    cargo build
    cargo install --path .

    cd $cwd
done
