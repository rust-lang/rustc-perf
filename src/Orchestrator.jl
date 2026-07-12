module Orchestrator

using Dates
using TimeZones
using SQLite, DataFrames

include("GitHubHelpers.jl")
include("UploadNanosoldierToDb.jl")
include("BuildkiteLogs.jl")

using .GitHubHelpers: github_commits, github_get_bytes, github_gitcommit, github_tree
using .UploadNanosoldierToDb: process_benchmarks
using .BuildkiteLogs: process_commit!

export main

const REPO_ROOT = normpath(joinpath(@__DIR__, ".."))

const sleep_time = Dates.Minute(5)
const db_path = joinpath(REPO_ROOT, "julia.db")
const state_dir = joinpath(REPO_ROOT, ".state")
const julia_checkpoint_file = joinpath(state_dir, "julia_last_processed_commit.txt")
const reports_checkpoint_file = joinpath(state_dir, "reports_last_processed_commit.txt")
const reports_repo_owner = "JuliaCI"
const reports_repo_name = "NanosoldierReports"
const reports_repo_branch = "master"

# Taken from PProf.jl
const proc = Ref{Union{Base.Process,Nothing}}(nothing)
function start_server()
    if !isnothing(proc[])
        error("Server already running")
    end
    proc[] = open(pipeline(`$(joinpath(REPO_ROOT, "prod_site")) $db_path`, stdout=stdout), read=false)
end
function kill_server()
    if !isnothing(proc[])
        kill(proc[])
        proc[] = nothing
    end
end

function read_checkpoint(path)
    if !isfile(path)
        error("Checkpoint file missing: $path. Fill it in manually before running.")
    end
    checkpoint = strip(read(path, String))
    if isempty(checkpoint)
        error("Checkpoint file is empty: $path. Fill it in manually before running.")
    end
    return checkpoint
end

function write_checkpoint(path, checkpoint)
    mkpath(dirname(path))
    write(path, "$checkpoint\n")
end

function report_error(context, err, backtrace)
    println(context)
    Base.showerror(stdout, err, backtrace)
    println()
end

function rollback_transaction(db, context)
    try
        DBInterface.execute(db, "ROLLBACK")
    catch err
        report_error("Error rolling back $context transaction", err, Base.catch_backtrace())
    end
end

function get_changed_benchmark_dirs(from_entries, to_entries)
    changed_dirs = Set{String}()
    for relpath in union(keys(from_entries), keys(to_entries))
        get(from_entries, relpath, nothing) == get(to_entries, relpath, nothing) && continue
        if startswith(relpath, "benchmark/by_date/")
            parts = splitpath(relpath)
            length(parts) >= 4 || continue
            push!(changed_dirs, joinpath(parts[1], parts[2], parts[3], parts[4]))
        elseif startswith(relpath, "benchmark/by_hash/")
            parts = splitpath(relpath)
            length(parts) >= 3 || continue
            push!(changed_dirs, joinpath(parts[1], parts[2], parts[3]))
        end
    end

    sort!(collect(changed_dirs))
end

function get_benchmark_tree_entries(owner, repo, commit_sha)
    commit = github_gitcommit(owner, repo, commit_sha)
    tree_sha = String(commit.tree["sha"])
    tree = github_tree(owner, repo, tree_sha; recursive=true)
    tree.truncated && error("Git tree for $owner/$repo at $commit_sha is truncated")

    entries = Dict{String,String}()
    for entry in tree.tree
        String(entry["type"]) == "blob" || continue
        path = String(entry["path"])
        if startswith(path, "benchmark/by_date/") || startswith(path, "benchmark/by_hash/")
            entries[path] = String(entry["sha"])
        end
    end
    entries
end

function get_reports_head_sha()
    response = github_commits(reports_repo_owner, reports_repo_name; sha=reports_repo_branch, per_page=1)
    isempty(response) && error("No commits found for $reports_repo_owner/$reports_repo_name on $reports_repo_branch")
    String(response[1].sha)
end

function with_downloaded_benchmark_dir(f, to_entries, commit_sha, benchmark_dir)
    prefix = benchmark_dir * "/"
    mktempdir() do tmpdir
        local_benchmark_dir = joinpath(tmpdir, splitpath(benchmark_dir)...)

        for relpath in keys(to_entries)
            startswith(relpath, prefix) || continue
            local_path = joinpath(tmpdir, splitpath(relpath)...)
            mkpath(dirname(local_path))
            write(
                local_path,
                github_get_bytes(
                    "https://raw.githubusercontent.com/$reports_repo_owner/$reports_repo_name/$commit_sha/$relpath",
                ),
            )
        end

        f(local_benchmark_dir)
    end
end

function benchmark_dir_exists(entries, benchmark_dir)
    prefix = benchmark_dir * "/"
    any(startswith(path, prefix) for path in keys(entries))
end

function commit_time_to_unix(dt::Union{DateTime,Nothing})
    isnothing(dt) && error("Missing commit author date")
    datetime2unix(dt) |> Int64
end

function get_master_commits_since(checkpoint_sha)
    commits = String[]
    commit_times = Dict{String,Int64}()
    found_checkpoint = false

    page = 1
    while true
        response = github_commits("JuliaLang", "Julia"; sha="master", per_page=100, page=page)

        if isempty(response)
            break
        end

        for commit in response
            sha = String(commit.sha)
            if sha == checkpoint_sha
                found_checkpoint = true
                break
            end
            push!(commits, sha)
            commit_times[sha] = commit_time_to_unix(commit.commit.author.date)
        end

        if found_checkpoint
            break
        end
        page += 1
    end

    if !found_checkpoint
        error("Last processed Julia commit $checkpoint_sha not found on origin/master history. Update $julia_checkpoint_file manually.")
    end

    reverse!(commits)
    return commits, commit_times
end

function main(install)
    db = SQLite.DB(db_path)
    julia_last_processed = read_checkpoint(julia_checkpoint_file)
    reports_last_processed = read_checkpoint(reports_checkpoint_file)

    println(install ? "Running in install mode" : "Running in dry-run mode")
    start_server()
    atexit(kill_server)

    while true
        if install
            read_checkpoint(julia_checkpoint_file) == julia_last_processed || error("Julia checkpoint file changed unexpectedly")
            read_checkpoint(reports_checkpoint_file) == reports_last_processed || error("Reports checkpoint file changed unexpectedly")
        end

        shas = String[]
        commit_times = Dict{String,Int64}()
        julia_fetched = false
        try
            shas, commit_times = get_master_commits_since(julia_last_processed)
            julia_fetched = true
        catch err
            report_error("Error fetching Julia commits from GitHub API", err, Base.catch_backtrace())
        end

        # Set before BEGIN so that a BEGIN rejected because of a transaction
        # leaked from a failed ROLLBACK still triggers the rollback that clears
        # it; cleared after COMMIT so errors past that point (e.g. in
        # write_checkpoint) don't issue a spurious no-op ROLLBACK.
        txn_open = false
        try
            if julia_fetched
                if install
                    txn_open = true
                    DBInterface.execute(db, "BEGIN TRANSACTION")
                end

                # I think I decided not to upload whatever timing stats we get from build logs
                artifact_size_df, pstat_df, first_unfinished_commit = process_logs(db, shas, commit_times; install=install)

                if !isempty(artifact_size_df)
                    kill_server()
                    if install
                        # Re-processed artifacts may already have rows; replace them.
                        # This runs inside the surrounding transaction, so the
                        # delete and load commit or roll back together.
                        aids = join(unique(artifact_size_df.aid), ",")
                        DBInterface.execute(db, "DELETE FROM artifact_size WHERE aid IN ($aids)")
                        SQLite.load!(artifact_size_df, db, "artifact_size")
                    end
                end

                if !isnothing(first_unfinished_commit)
                    println("Commit $first_unfinished_commit not finished!")
                    idx = findfirst(x -> x == first_unfinished_commit, shas) - 1
                    last_finished_commit = idx == 0 ? julia_last_processed : shas[idx]
                    next_julia_checkpoint = last_finished_commit
                else
                    next_julia_checkpoint = isempty(shas) ? julia_last_processed : shas[end]
                end

                if install
                    DBInterface.execute(db, "COMMIT")
                    txn_open = false
                end
                install && write_checkpoint(julia_checkpoint_file, next_julia_checkpoint)
                julia_last_processed = next_julia_checkpoint
            end
        catch err
            report_error("Error processing logs", err, Base.catch_backtrace())
            txn_open && rollback_transaction(db, "logs")
        end

        reports_head = ""
        from_entries = Dict{String,String}()
        to_entries = Dict{String,String}()
        fetched = false
        try
            reports_head = get_reports_head_sha()
            from_entries = get_benchmark_tree_entries(reports_repo_owner, reports_repo_name, reports_last_processed)
            to_entries = get_benchmark_tree_entries(reports_repo_owner, reports_repo_name, reports_head)
            fetched = true
        catch err
            report_error("Error fetching reports repo metadata from GitHub", err, Base.catch_backtrace())
        end

        txn_open = false
        try
            if fetched
                changed_benchmark_dirs = get_changed_benchmark_dirs(
                    from_entries,
                    to_entries,
                )

                if install
                    txn_open = true
                    DBInterface.execute(db, "BEGIN TRANSACTION")
                end

                for benchmark_dir in changed_benchmark_dirs
                    benchmark_dir_exists(to_entries, benchmark_dir) || continue
                    kill_server()
                    println("$(benchmark_dir) changed")
                    with_downloaded_benchmark_dir(to_entries, reports_head, benchmark_dir) do local_benchmark_dir
                        process_benchmarks(local_benchmark_dir, db; install=install)
                    end
                end

                if install
                    DBInterface.execute(db, "COMMIT")
                    txn_open = false
                end
                install && write_checkpoint(reports_checkpoint_file, reports_head)
                reports_last_processed = reports_head
            end
        catch err
            report_error("Error processing benchmarks", err, Base.catch_backtrace())
            txn_open && rollback_transaction(db, "benchmark")
        end

        # The server can also die on its own; a dead handle must not block the
        # restart.
        if !isnothing(proc[]) && !process_running(proc[])
            proc[] = nothing
        end
        if isnothing(proc[])
            start_server()
        end
        sleep(sleep_time)
    end
end

function process_logs(db::SQLite.DB, shas, commit_times; install)
    artifact_id_query = DBInterface.execute(db, "SELECT id FROM artifact ORDER BY id DESC LIMIT 1") |> DataFrame
    next_artifact_id = Ref{Int}((isempty(artifact_id_query) ? 0 : artifact_id_query[1, "id"]) + 1)

    artifact_size_df = DataFrame()
    pstat_df = DataFrame()

    first_unfinished_commit = nothing
    for sha in shas
        println("Processing $sha log")
        artifact_query = DBInterface.execute(db, "SELECT * FROM artifact WHERE name='$(sha)' LIMIT 1") |> DataFrame
        artifact_id = if !isempty(artifact_query)
            artifact_query[1, "id"]
        else
            date = commit_times[sha]
            artifact_row = (id=next_artifact_id[], name=sha, date=date, type="master")

            next_artifact_id[] += 1

            install && DBInterface.execute(db, "INSERT INTO artifact (id, name, date, type) VALUES ($(artifact_row.id), '$(artifact_row.name)', $(artifact_row.date), '$(artifact_row.type)')")
            artifact_row.id
        end

        local res
        try
            commit_time = commit_times[sha]
            res = process_commit!(artifact_size_df, pstat_df, artifact_id, sha, "master", identity, commit_time)
        catch err
            report_error("Error processing $sha logs", err, Base.catch_backtrace())
            rethrow()
        end

        if res == :no_ci
            println("Commit $sha has no CI")
            continue
        end

        if res == :not_finished
            println("Commit $sha not finished")
            first_unfinished_commit = sha
            break
        end

        # for row in eachrow(pstat_df)
        #     metric = row.series in ("minor", "major") ? "$(row.series)-pagefaults" : row.series
        #     push_metric_to_pstat!(df, db, "init", "median-$metric", artifact_row.id, median(row.value), benchmark_to_pstat_series_id)
        # end
    end

    return artifact_size_df, pstat_df, first_unfinished_commit
end


end # module Orchestrator
