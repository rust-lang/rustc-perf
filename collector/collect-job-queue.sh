#!/bin/bash

# This script expects DATABASE and COLLECTOR_NAME to be defined in the environment

set -u -o pipefail

echo "Running job queue collector"

export RUST_LOG=collector=trace,collector::sysroot=debug
export PATH="/home/collector/.cargo/bin:$PATH"

while : ; do
        # Update and rebuild the collector.
        git pull
        git reset --hard @{upstream}

        # Make sure we have a recent build, so that we can successfully build
        # the collector.
        rustup update stable
        cargo +stable build --release -p collector --features s3-sdk

        CURRENT_SHA=`git rev-parse HEAD`

        target/release/collector benchmark_job_queue \
          --db "${DATABASE}" \
          --check_git_sha \
          --git_sha "${CURRENT_SHA}" \
          --collector_name "${COLLECTOR_NAME}"

        STATUS=$?
        echo finished run at `date` with exit code $STATUS

        # Wait a bit if the command has failed.
        if [ $STATUS -ne 0 ]; then
            sleep 60
        fi
done
