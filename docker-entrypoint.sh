#!/bin/sh
set -eu
umask 027

app_root="${RUSTC_PERF_APP_ROOT:-/app}"
data_root="${RUSTC_PERF_DATA_DIR:-/var/lib/rustc-perf}"
db_name="${RUSTC_PERF_DB_FILENAME:-julia.db}"
julia_bin="${JULIA_BIN:-/usr/local/julia/bin/julia}"
state_root="${data_root}/.state"
default_state_root="${app_root}/.state.dist"

seed_julia_checkpoint_from_db() {
    checkpoint_path="$1"

    if [ ! -f "${data_root}/${db_name}" ]; then
        return 1
    fi

    checkpoint_value="$(sqlite3 "${data_root}/${db_name}" "SELECT name FROM artifact WHERE type='master' ORDER BY id DESC LIMIT 1;" 2>/dev/null | tr -d '\r')"
    if [ -z "${checkpoint_value}" ]; then
        return 1
    fi

    printf '%s\n' "${checkpoint_value}" > "${checkpoint_path}"
}

install -d -m 750 "${data_root}" "${state_root}"

if [ ! -s "${state_root}/julia_last_processed_commit.txt" ]; then
    if ! seed_julia_checkpoint_from_db "${state_root}/julia_last_processed_commit.txt"; then
        cp "${default_state_root}/julia_last_processed_commit.txt" "${state_root}/julia_last_processed_commit.txt"
    fi
fi

if [ ! -s "${state_root}/reports_last_processed_commit.txt" ]; then
    cp "${default_state_root}/reports_last_processed_commit.txt" "${state_root}/reports_last_processed_commit.txt"
fi

ln -sfn "${state_root}" "${app_root}/.state"
ln -sfn "${data_root}/${db_name}" "${app_root}/julia.db"

if [ ! -f "${data_root}/${db_name}" ]; then
    echo "Expected SQLite database at ${data_root}/${db_name}. Seed it before starting the container." >&2
    exit 1
fi

if [ ! -w "${app_root}" ] || [ ! -w "${state_root}" ] || [ ! -r "${data_root}/${db_name}" ] || [ ! -w "${data_root}/${db_name}" ]; then
    echo "The runtime needs read/write access to ${app_root}, ${state_root}, and ${data_root}/${db_name}." >&2
    exit 1
fi

# TODO: We should provide a github token so the app doesn't get rate limited
exec "${julia_bin}" --project="${app_root}" "${app_root}/run.jl" --install "$@"