#!/bin/bash

set -x

for dir in benchmarks/*; do
    pwd;
    if [[ -d "$dir" ]]; then
        cd "$dir";
        patches=`make patches`;
        for patch in "" $patches; do
            CARGO=cargo \
            RUSTC=rustc \
            make "all$patch";
        done;
        cd ../..;
    fi
done
