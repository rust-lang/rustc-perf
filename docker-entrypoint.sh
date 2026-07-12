#!/bin/sh
set -eu
umask 027

# All four are defined by the image's ENV block (the host env file may
# override the data dir and db name).
app_root="${RUSTC_PERF_APP_ROOT:?RUSTC_PERF_APP_ROOT is not set}"
data_root="${RUSTC_PERF_DATA_DIR:?RUSTC_PERF_DATA_DIR is not set}"
db_name="${RUSTC_PERF_DB_FILENAME:?RUSTC_PERF_DB_FILENAME is not set}"
julia_bin="${JULIA_BIN:?JULIA_BIN is not set}"
state_root="${data_root}/.state"

install -d -m 750 "${data_root}" "${state_root}"

# Both SQLite.jl and rusqlite silently create a missing database file, so
# without this guard a host whose restore found nothing would come up as a
# healthy-looking empty site instead of failing. Checkpoint files are the
# orchestrator's concern: it errors clearly if either checkpoint is missing.
if [ ! -s "${data_root}/${db_name}" ]; then
    echo "Expected SQLite database at ${data_root}/${db_name}. Seed it before starting the container." >&2
    exit 1
fi

ln -sfn "${state_root}" "${app_root}/.state"
ln -sfn "${data_root}/${db_name}" "${app_root}/julia.db"

if [ ! -w "${app_root}" ] || [ ! -w "${state_root}" ] || [ ! -r "${data_root}/${db_name}" ] || [ ! -w "${data_root}/${db_name}" ]; then
    echo "The runtime needs read/write access to ${app_root}, ${state_root}, and ${data_root}/${db_name}." >&2
    exit 1
fi

# TODO: We should provide a github token so the app doesn't get rate limited
exec "${julia_bin}" --project="${app_root}" "${app_root}/run.jl" --install "$@"
