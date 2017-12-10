#!/bin/bash

# fail if variables are unset
set -u
# exit if anything fails
set -e

cd benchmarks;
for dir in *; do
    if [[ -d "$dir" ]]; then
        cd "$dir"
        patches=(`make patches`);
        for patch in "${patches[@]:-}"; do
            test_name="$dir${patch/@/_}"
            echo "travis_fold:start:$test_name"
            echo "travis_time:start:$test_name"
            echo "Checking $dir$patch..."
            start_time=$(date -u '+%s%N')
            CARGO=cargo \
            RUSTC=rustc \
            CARGO_RUSTC_OPTS=--cap-lints=warn \
            make "all$patch";
            end_time=$(date -u '+%s%N')
            duration=$(($end_time-$start_time))
            echo
            echo "travis_fold:end:$test_name"
            echo "travis_time:end:$test_name:start=$start_time,finish=$end_time,duration=$duration"
        done;
        cd ..;
    fi
done
