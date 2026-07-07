#!/bin/sh
set -eu
umask 027

app_root="${RUSTC_PERF_APP_ROOT:-/app}"
data_root="${RUSTC_PERF_DATA_DIR:-/var/lib/rustc-perf}"
db_name="${RUSTC_PERF_DB_FILENAME:-julia.db}"
julia_bin="${JULIA_BIN:-/usr/local/julia/bin/julia}"
state_root="${data_root}/.state"

install -d -m 750 "${data_root}" "${state_root}"

# Both SQLite.jl and rusqlite silently create a missing database file, so
# without this guard a host whose restore found nothing would come up as a
# healthy-looking empty site instead of failing. Checkpoint files are the
# orchestrator's concern: it derives the Julia checkpoint from the database
# and errors clearly if the reports checkpoint is missing.
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
