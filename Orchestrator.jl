using Pkg
Pkg.activate(@__DIR__)
using LibGit2, Dates
using HTTP, JSON3
include("upload_nanosoldier_to_db.jl")
include("buildkite_logs.jl")

const sleep_time = Dates.Minute(5)
const db_path = joinpath(@__DIR__, "julia.db")
const state_dir = joinpath(@__DIR__, ".state")
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
    proc[] = open(pipeline(`$(joinpath(@__DIR__, "prod_site")) $db_path`, stdout=stdout), read=false)
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

function get_changed_benchmark_dirs(from_entries, to_entries)
    changed_dirs = Set{String}()
    for relpath in union(keys(from_entries), keys(to_entries))
        get(from_entries, relpath, nothing) == get(to_entries, relpath, nothing) && continue
        if startswith(relpath, "benchmark/by_date/")
            parts = splitpath(relpath)
            length(parts) >= 3 || continue
            push!(changed_dirs, joinpath(parts[1], parts[2], parts[3]))
        elseif startswith(relpath, "benchmark/by_hash/")
            parts = splitpath(relpath)
            length(parts) >= 3 || continue
            push!(changed_dirs, joinpath(parts[1], parts[2], parts[3]))
        end
    end

    sort!(collect(changed_dirs))
end

function get_benchmark_tree_entries(owner, repo, commit_sha)
    commit = github_get_json("https://api.github.com/repos/$owner/$repo/git/commits/$commit_sha")
    tree_sha = String(commit.tree.sha)
    tree = github_get_json("https://api.github.com/repos/$owner/$repo/git/trees/$tree_sha?recursive=1")
    tree.truncated && error("Git tree for $owner/$repo at $commit_sha is truncated")

    entries = Dict{String,String}()
    for entry in tree.tree
        String(entry.type) == "blob" || continue
        path = String(entry.path)
        if startswith(path, "benchmark/by_date/") || startswith(path, "benchmark/by_hash/")
            entries[path] = String(entry.sha)
        end
    end
    entries
end

function github_get_bytes(url; retries=5, initial_delay=1.0)
    delay = initial_delay
    last_error = nothing

    for attempt in 1:retries
        try
            resp = HTTP.get(url; headers=headers, status_exception=false)
            if 200 <= resp.status < 300
                return resp.body
            end
            if resp.status in (403, 429) || 500 <= resp.status < 600
                last_error = ErrorException("GitHub request returned HTTP $(resp.status) for $url")
            else
                error("GitHub request returned HTTP $(resp.status) for $url")
            end
        catch err
            last_error = err
        end

        if attempt < retries
            sleep(delay)
            delay *= 2
        end
    end

    throw(last_error)
end

function get_reports_head_sha()
    response = github_get_json(
        "https://api.github.com/repos/$reports_repo_owner/$reports_repo_name/commits?sha=$reports_repo_branch&per_page=1",
    )
    isempty(response) && error("No commits found for $reports_repo_owner/$reports_repo_name on $reports_repo_branch")
    String(response[1].sha)
end

function with_downloaded_benchmark_dir(to_entries, commit_sha, benchmark_dir, f)
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

function get_master_commits_since(checkpoint_sha)
    commits = String[]
    commit_times = Dict{String,Int64}()
    found_checkpoint = false

    page = 1
    while true
        response = github_get_json(
            "https://api.github.com/repos/JuliaLang/Julia/commits?sha=master&per_page=100&page=$page",
        )

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
            commit_times[sha] = DateTime(ZonedDateTime(commit.commit.author.date), UTC) |> datetime2unix |> Int64
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

function github_get_json(url; retries=5, initial_delay=1.0)
    delay = initial_delay
    last_error = nothing

    for attempt in 1:retries
        try
            resp = HTTP.get(url; headers=headers, status_exception=false)
            if 200 <= resp.status < 300
                return JSON3.read(resp.body)
            end
            if resp.status in (403, 429) || 500 <= resp.status < 600
                last_error = ErrorException("GitHub API returned HTTP $(resp.status) for $url")
            else
                error("GitHub API returned HTTP $(resp.status) for $url")
            end
        catch err
            last_error = err
        end

        if attempt < retries
            sleep(delay)
            delay *= 2
        end
    end

    throw(last_error)
end

function main()
    db = SQLite.DB(db_path)

    start_server()
    atexit(kill_server)

    while true
        changed = false

        julia_fetched = false
        julia_last_processed = read_checkpoint(julia_checkpoint_file)
        try
            shas, commit_times = get_master_commits_since(julia_last_processed)
            julia_fetched = true
        catch err
            println("Error fetching Julia commits from GitHub API: $err")
            Base.showerror(stdout, err, Base.catch_backtrace())
        end

        try
            if julia_fetched
                DBInterface.execute(db, "BEGIN TRANSACTION")

                artifact_size_df, pstat_df, first_unfinished_commit = process_logs(db, shas, commit_times)
                changed |= !isempty(artifact_size_df)
                if !isempty(artifact_size_df)
                    kill_server()
                    SQLite.load!(artifact_size_df, db, "artifact_size")
                end

                if !isnothing(first_unfinished_commit)
                    println("Commit $first_unfinished_commit not finished!")
                    idx = findfirst(x -> x == first_unfinished_commit, shas) - 1
                    last_finished_commit = idx == 0 ? julia_last_processed : shas[idx]
                    next_julia_checkpoint = last_finished_commit
                else
                    next_julia_checkpoint = isempty(shas) ? julia_last_processed : shas[end]
                end

                DBInterface.execute(db, "COMMIT")
                write_checkpoint(julia_checkpoint_file, next_julia_checkpoint)
            end
        catch
            println("Error processing logs")
            DBInterface.execute(db, "ROLLBACK")
            rethrow()
        end

        fetched = false
        reports_last_processed = read_checkpoint(reports_checkpoint_file)
        try
            reports_head = get_reports_head_sha()
            from_entries = get_benchmark_tree_entries(reports_repo_owner, reports_repo_name, reports_last_processed)
            to_entries = get_benchmark_tree_entries(reports_repo_owner, reports_repo_name, reports_head)
            fetched = true
        catch err
            println("Error fetching reports repo metadata from GitHub: $err")
            Base.showerror(stdout, err, Base.catch_backtrace())
        end

        try
            if fetched
                changed_benchmark_dirs = get_changed_benchmark_dirs(
                    from_entries,
                    to_entries,
                )

                DBInterface.execute(db, "BEGIN TRANSACTION")

                for benchmark_dir in changed_benchmark_dirs
                    benchmark_dir_exists(to_entries, benchmark_dir) || continue
                    changed = true
                    kill_server()
                    println("$(benchmark_dir) changed")
                    with_downloaded_benchmark_dir(to_entries, reports_head, benchmark_dir) do local_benchmark_dir
                        process_benchmarks(local_benchmark_dir, db)
                    end
                end

                DBInterface.execute(db, "COMMIT")
                write_checkpoint(reports_checkpoint_file, reports_head)
            end
        catch
            println("Error processing benchmarks")
            DBInterface.execute(db, "ROLLBACK")
            rethrow()
        end

        if changed
            start_server()
        end
        sleep(sleep_time)
    end
end

function process_logs(db::SQLite.DB, shas, commit_times)
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

            DBInterface.execute(db, "INSERT INTO artifact (id, name, date, type) VALUES ($(artifact_row.id), '$(artifact_row.name)', $(artifact_row.date), '$(artifact_row.type)')")
            artifact_row.id
        end

        local res
        try
            commit_time = commit_times[sha]
            res = process_commit!(artifact_size_df, pstat_df, artifact_id, sha, "master", identity, commit_time)
        catch err
            println("Error processing $sha logs")
            Base.showerror(stdout, err, Base.catch_backtrace())
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

isinteractive() || main()
