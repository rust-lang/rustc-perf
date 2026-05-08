using Tar, CodecZlib, CodecXz, CodecZstd
using BenchmarkTools
using DataFrames, Dates, TimeZones
using SQLite
using HTTP, JSON3

const headers = if haskey(ENV, "GITHUB_TOKEN")
    Dict("Authorization" => "token " * ENV["GITHUB_TOKEN"])
else
    nothing
end

median(x) = sort(x)[div(length(x), 2)+1]
function process_benchmark_archive!(df, path, next_artifact_id, db, benchmark_to_pstat_series_id; return_group_only=false, install)
    println("Processing $path...")
    mktempdir() do dir
        # dir = mktempdir() # Debugger is unable to step into the above line
        # extract
        open(path) do io
            stream = if endswith(path, ".xz")
                XzDecompressorStream(io)
            elseif endswith(path, ".gz")
                GzipDecompressorStream(io)
            elseif endswith(path, ".zst")
                ZstdDecompressorStream(io)
            else
                error("Unknown file extension")
            end
            Tar.extract(stream, dir)
        end

        # TODO: Support non full benchmark suites, to do need to address TODO about overwriting data conditionally
        report_file = readuntil(joinpath(splitpath(path)[1:end-1]..., "report.md"), "## Results")
        tag_predicate = match(r"\*Tag Predicate:\* `(.*)`", report_file).captures[1]
        # if !return_group_only && !occursin("*Tag Predicate:* `ALL`", report_file)
        #     return
        # end

        # strip the contained "data" directory
        data = joinpath(dir, "data")
        if !isdir(data)
            data = dir
        end

        processed_commits = String[]

        for file in readdir(data)
            if !endswith(file, ".json") || !contains(file, r".(minimum|median|mean).json")
                continue
            end
            primary_metric = split(file, '.')[end-1]
            if primary_metric == "minimum"
                primary_metric = "min"
            end

            artifact_id = nothing
            sha = get_sha(file)

            # Sometimes the same commit is benchmarked multiple times, we just keep the original data
            if !return_group_only
                artifact_query = DBInterface.execute(db, "SELECT * FROM artifact WHERE name='$sha' LIMIT 1") |> DataFrame

                if sha ∉ processed_commits
                    # all files for this commit should also hit continue so good
                    if !isempty(artifact_query) && DataFrame(DBInterface.execute(db, "SELECT EXISTS (SELECT * FROM pstat WHERE aid=$(artifact_query[1, :id]))"))[1, 1] == 1
                        if tag_predicate == "ALL"
                            install && DBInterface.execute(db, "DELETE FROM pstat WHERE aid=$(artifact_query[1, "id"])")
                            install && DBInterface.execute(db, "DELETE FROM pull_request_build WHERE bors_sha='$sha'")
                        else
                            continue
                        end
                    end

                    local artifact_row, parent_sha
                    if isempty(artifact_query)
                        # For a pr, we want parent_sha to be the commit it's compared to
                        artifact_row, parent_sha = create_artifact_row(path, file, next_artifact_id[])
                        next_artifact_id[] += 1

                        install && DBInterface.execute(db, "INSERT INTO artifact (id, name, date, type) VALUES ($(artifact_row.id), '$(artifact_row.name)', $(artifact_row.date), '$(artifact_row.type)')")
                    else
                        artifact_row = (id=artifact_query[1, "id"], name=artifact_query[1, "name"], date=artifact_query[1, "date"], type=artifact_query[1, "type"])
                        parent_sha = create_artifact_row(path, file, artifact_row.id)[2]
                    end

                    if artifact_row.type == "try"
                        commit_sha = filter(file -> endswith(file, ".json") && contains(file, r".(minimum|median|mean).json"), readdir(data)) .|> get_sha |> unique
                        if length(commit_sha) == 1 # PRs can be benchmarked without comparing to anything
                            parent_sha = sha
                        else
                            parent_sha = commit_sha[1] == sha ? commit_sha[2] : commit_sha[1]
                        end
                    end

                    pr_details = HTTP.get("https://api.github.com/repos/JuliaLang/Julia/commits/$sha/pulls", headers=headers).body |> JSON3.read

                    min_created_date = nothing
                    mini = 0
                    if !isempty(pr_details)
                        for i in eachindex(pr_details)
                            if isnothing(min_created_date) || DateTime(pr_details[i].created_at, dateformat"yyyy-mm-ddTHH:MM:SS\Z") < min_created_date
                                min_created_date = DateTime(pr_details[i].created_at, dateformat"yyyy-mm-ddTHH:MM:SS\Z")
                                mini = i
                            end
                        end
                    end

                    if !isnothing(min_created_date)
                        pr_num = pr_details[mini].number
                        install && DBInterface.execute(db, "INSERT INTO pull_request_build (bors_sha, pr, parent_sha, commit_date, include) VALUES ('$sha', $pr_num, '$parent_sha', $(artifact_row.date), '$tag_predicate')")
                    else
                        regex_match = nothing
                        if artifact_row.type == "try"
                            regex_match = match(r"\*Triggered By:\* \[link\]\(https://github.com/JuliaLang/julia/pull/(\d+)", report_file)
                        end

                        if !isnothing(regex_match)
                            triggered_by_pr = regex_match.captures[1] |> x -> parse(Int, x)
                            println("Commit $sha has no PR using triggered by pr $triggered_by_pr instead")
                            install && DBInterface.execute(db, "INSERT INTO pull_request_build (bors_sha, pr, parent_sha, commit_date, include) VALUES ('$sha', $triggered_by_pr, '$parent_sha', $(artifact_row.date), '$tag_predicate')")
                        else
                            if artifact_row.type == "try"
                                printstyled("Commit $sha is a try build but was not triggered in a PR\n", color=:red)
                            else
                                printstyled("Commit $sha has no PR\n", color=:red)
                            end
                            install && DBInterface.execute(db, "INSERT INTO pull_request_build (bors_sha, parent_sha, commit_date, include) VALUES ('$sha', '$parent_sha', $(artifact_row.date), '$tag_predicate')")
                        end
                    end

                    artifact_id = artifact_row.id

                    # if artifact_row.type == "master"
                    #     artifact_size_df = DataFrame()
                    #     pstat_df = DataFrame()
                    #     process_commit!(artifact_size_df, pstat_df, artifact_row.id, artifact_row.name, "master", identity)
                    #     SQLite.load!(artifact_size_df, db, "artifact_size")
                    #     # for row in eachrow(pstat_df)
                    #     #     metric = row.series in ("minor", "major") ? "$(row.series)-pagefaults" : row.series
                    #     #     push_metric_to_pstat!(df, db, "init", "median-$metric", artifact_row.id, median(row.value), benchmark_to_pstat_series_id)
                    #     # end
                    # end
                else
                    artifact_id = artifact_query[1, "id"]
                end
            end

            group = BenchmarkTools.load(joinpath(data, file))[1]
            return_group_only && return group

            benchmark_data = rec_flatten_benchmarkgroup(group)

            for (benchmark_name, trial) in benchmark_data
                if sha ∉ processed_commits # get alloc and memory data, same for all files
                    for metric in (:allocs, :memory) # allocs is num of allocations, memory is in bytes
                        push_metric_to_pstat!(df, db, benchmark_name, string(metric), artifact_id, getfield(trial, metric), benchmark_to_pstat_series_id; install=install)
                    end
                end

                push_metric_to_pstat!(df, db, benchmark_name, primary_metric * "-wall-time", artifact_id, trial.time, benchmark_to_pstat_series_id; install=install)
                push_metric_to_pstat!(df, db, benchmark_name, primary_metric * "-gc-time", artifact_id, trial.gctime, benchmark_to_pstat_series_id; install=install)
            end

            if sha ∉ processed_commits
                push!(processed_commits, sha)
            end
        end
    end
end

function push_metric_to_pstat!(df::DataFrame, db::SQLite.DB, benchmark_name::String, metric::String, aid::Int64, value, benchmark_to_pstat_series_id; install)
    if !haskey(benchmark_to_pstat_series_id, (benchmark_name, metric))
        pstat_series_query = DBInterface.execute(db, "SELECT * FROM pstat_series ORDER BY id DESC LIMIT 1") |> DataFrame
        next_pid = 1 + pstat_series_query[1, "id"]

        temp_df = DataFrame(id=next_pid, crate=benchmark_name, profile="opt", scenario="full", backend="llvm", metric=metric)
        install && SQLite.load!(temp_df, db, "pstat_series")
        benchmark_to_pstat_series_id[(benchmark_name, metric)] = next_pid
    end
    series_id = benchmark_to_pstat_series_id[(benchmark_name, metric)]

    pstat_row = (series=series_id, aid=aid, value=Float64(value))
    push!(df, pstat_row)
end

function create_pstat_rows(series_id, aid, val)
    len = length(val)
    DataFrame(series=series_id, aid=fill(aid, len), value=val)
end

get_sha(file) = split(file, '_')[1]
function create_artifact_row(path, file, id)
    commit_sha = get_sha(file)

    date = nothing
    type = nothing
    parent_sha = nothing

    search_on_master = HTTP.get("https://api.github.com/search/commits?q=repo:JuliaLang/julia+hash:$commit_sha", headers=headers).body |> JSON3.read

    if search_on_master.total_count == 0 # commit not in master branch
        commit_details = HTTP.get("https://api.github.com/repos/JuliaLang/Julia/git/commits/$commit_sha", headers=headers).body |> JSON3.read # can remove /git/ not sure why it's there
        date = DateTime(commit_details.author.date, dateformat"yyyy-mm-ddTHH:MM:SS\Z") |> datetime2unix |> Int64
        type = "try"
        # parent_sha = commit_details.parents[1].sha # How do you deal with multiple parents? Presumably we want the one on master
    elseif search_on_master.total_count == 1
        # Occasionally the sha is incomplete, e.g. for 0b29986_vs_8dc69aa
        # We want to continue using the sha we get for the file, to make things easier
        # E.g. if we run this commit again we want to know the sha when we query artifacts and not require api call
        if search_on_master.items[1].sha != commit_sha
            printstyled("Commit: $commit_sha has different sha on search $(search_on_master.items[1].sha)\n", color=:red)
        end
        date = DateTime(ZonedDateTime(search_on_master.items[1].commit.author.date), UTC) |> datetime2unix |> Int64
        type = "master"
        parent_sha = search_on_master.items[1].parents[1].sha
    else
        println("Commit: $commit_sha")
        display(search_on_master)
        error("Found too many commits")
    end

    (id=id, name=commit_sha, date=date, type=type), parent_sha
end

function rec_flatten_benchmarkgroup(d)
    new_d = Dict{String,BenchmarkTools.TrialEstimate}()
    for (key, value) in pairs(d.data)
        if key isa Tuple
            key = "(" * join(key, ", ") * ")"
        end
        if isa(value, BenchmarkGroup)
            flattened_value = rec_flatten_benchmarkgroup(value)
            for (ikey, ivalue) in pairs(flattened_value)
                new_d["$key.$ikey"] = ivalue
            end
        else
            new_d[key] = value
        end
    end
    return new_d
end

function create_benchmark(data::AbstractDict)
    len = length(data)
    DataFrame(name=collect(keys(data)), stabilized=zeros(Int64, len), category=fill("primary", len))
end

function create_pstat_series(benchmark_table)
    df = DataFrame()

    pid = 1
    for row in eachrow(benchmark_table)
        for metric in ("min-wall-time", "median-wall-time", "mean-wall-time", "allocs", "memory", "min-gc-time", "median-gc-time", "mean-gc-time")
            push!(df, (id=pid, crate=row["name"], profile="opt", scenario="full", backend="llvm", metric=metric))
            pid += 1
        end
    end

    return df
end

# TODO: Seperate out setup and processing
# Don't pass a dir starting with ~, doesn't work
function process_benchmarks(dir, db::SQLite.DB; install)
    artifact_id_query = DBInterface.execute(db, "SELECT id FROM artifact ORDER BY id DESC LIMIT 1") |> DataFrame
    next_artifact_id = Ref{Int}((isempty(artifact_id_query) ? 0 : artifact_id_query[1, "id"]) + 1)

    pstat_series_table = DBInterface.execute(db, "SELECT * FROM pstat_series") |> DataFrame
    # need to tranform into vector as indexing into df extremely slow
    names_col = pstat_series_table[:, "crate"]
    metrics_col = pstat_series_table[:, "metric"]
    pstat_series_id_column = pstat_series_table[:, "id"]

    benchmark_to_pstat_series_id = Dict((name, metric) => id for (id, name, metric) in zip(pstat_series_id_column, names_col, metrics_col))

    # artifact_df = DataFrame()
    # pstat_df = DataFrame()

    for (root, dirs, files) in walkdir(dir)
        pstat_df = DataFrame()
        for file in files
            if contains(file, r"^data.tar.\w+$")
                @time "processing" process_benchmark_archive!(pstat_df, joinpath(root, file), next_artifact_id, db, benchmark_to_pstat_series_id; install=install)
            end
        end
        if install && !isempty(pstat_df)
            @time "loading" SQLite.load!(pstat_df, db, "pstat")
        end
    end

    # dir = "/home/rag/Documents/Code/NanosoldierReports/benchmark/by_date/2023-12/28/data.tar.zst"
    # pstat_df = DataFrame()
    # result = process_benchmark_archive!(pstat_df, dir, artifact_id, db, benchmark_to_pstat_series_id)
    # return pstat_df
    # artifact_id += 1

    # for file in readdir(dir)
    #     if contains(file, r"^data.tar.\w+$")
    #         result = process_benchmark_archive(joinpath(dir, file), id, db)
    #         id += 1
    #     end
    # end


    # DBInterface.execute(db, "DELETE FROM artifact")
    # artifact_df |> SQLite.load!(db, "artifact")
    # DBInterface.execute(db, "DELETE FROM pstat")
    # pstat_df |> SQLite.load!(db, "pstat")

    nothing
end

function process_benchmarks(dir, db_path::AbstractString; install)
    db = SQLite.DB(db_path)
    try
        process_benchmarks(dir, db; install=install)
    finally
        close(db)
    end
end

function auto_load(month=lpad(month(now()), 2, '0'), year=year(now()))
    process_benchmarks("/home/rag/Documents/Code/NanosoldierReports/benchmark/by_date/$year-$month", joinpath(@__DIR__, "julia.db"); install=true)
    # for dir in filter(isdir, readdir("/home/rag/Documents/Code/NanosoldierReports/benchmark/by_hash/", join=true))
    #     process_benchmarks(dir, joinpath(@__DIR__, "julia.db"); install=true)
    # end
end

function create_tables(db_path)
    dir = "/home/rag/Documents/Code/NanosoldierReports/benchmark/by_date"
    @show dir
    # dates = readdir(dir)
    # sort!(dates; by=x->DateTime(x))
    # last_date = dates[end]
    # dir = joinpath(dir, last_date)

    # days = readdir(dir)
    # sort!(days; by=x->parse(Int, x))
    # last_day = days[end]
    # dir = joinpath(dir, last_day)

    # for file in readdir(dir)
    #     if contains(file, r"^data.tar.\w+$")

    benchmark_table = nothing

    for (root, dirs, files) in walkdir(dir)
        for file in files
            if contains(file, r"^data.tar.\w+$")
                group = process_benchmark_archive!(nothing, joinpath(root, file), nothing, nothing, nothing, nothing; return_group_only=true, install=true)
                isnothing(group) && continue
                data = rec_flatten_benchmarkgroup(group)
                new_benchmark_table = create_benchmark(data)
                if isnothing(benchmark_table)
                    benchmark_table = new_benchmark_table
                else
                    benchmark_table = unique(append!(benchmark_table, new_benchmark_table))
                end
            end
        end
    end

    pstat_series_table = create_pstat_series(benchmark_table)

    return benchmark_table, pstat_series_table

    db = SQLite.DB(db_path)

    DBInterface.execute(db, "DELETE FROM benchmark")
    DBInterface.execute(db, "DELETE FROM pstat_series")
    benchmark_table |> SQLite.load!(db, "benchmark")
    pstat_series_table |> SQLite.load!(db, "pstat_series")

    nothing
end

using LibGit2
function fix_dates(db_path)
    db = SQLite.DB(db_path)
    artifacts_table = DBInterface.execute(db, "SELECT * FROM artifact") |> DataFrame

    mktempdir() do dir
        repo = LibGit2.clone("https://github.com/JuliaLang/julia.git", dir)
        for (i, row) in pairs(eachrow(artifacts_table))
            commit = LibGit2.GitCommit(repo, row.name)
            commit_date = LibGit2.author(commit).time
            artifacts_table[i, "date"] = commit_date
        end
    end

    DBInterface.execute(db, "DELETE FROM artifact")
    artifacts_table |> SQLite.load!(db, "artifact")
end

function fix_dates_pull_request_build(db_path)
    db = SQLite.DB(db_path)
    pull_request_build_table = DBInterface.execute(db, "SELECT * FROM pull_request_build") |> DataFrame

    # mktempdir() do dir
    #     repo = LibGit2.clone("https://github.com/JuliaLang/julia.git", dir)
    for (i, row) in pairs(eachrow(pull_request_build_table))
        if row["commit_date"] !== missing
            continue
        end
        try
            commit_details = HTTP.get("https://api.github.com/repos/JuliaLang/Julia/git/commits/$(row.bors_sha)", headers=headers).body |> JSON3.read # can remove /git/ not sure why it's there
            date = DateTime(commit_details.author.date, dateformat"yyyy-mm-ddTHH:MM:SS\Z") |> datetime2unix |> Int64
            # if (row["commit_date"] !== missing && row["commit_date"] != date)
            #     println("Commit date already set for $i, $(row.bors_sha) to $(row["commit_date"]), whilst we were going to set it to $date")
            # else
            pull_request_build_table[i, "commit_date"] = date
            # end
        catch
            println("failed for $i, $row")
        end
    end
    # end

    return pull_request_build_table
    # DBInterface.execute(db, "DELETE FROM pull_request_build")
    # pull_request_build_table |> SQLite.load!(db, "pull_request_build")
end

function add_old_by_hash_benchmarks()
    nanosoldier_dir = joinpath(@__DIR__, "..", "NanosoldierReports")
    # repo = LibGit2.GitRepo(nanosoldier_dir)

    # Many commits are unique, so just process any benchmark of theirs instead of trying to sort benchmarks by date, which seems hard
    # We've already processed the `ALL`s so we defo have those.
    # dirs = vcat((readdir(".") .|> x->split(x, "_vs_"))...)

    for dir in filter(isdir, readdir(joinpath(nanosoldier_dir, "benchmark", "by_hash/"), join=true))
        process_benchmarks(dir, joinpath(@__DIR__, "julia.db"); install=true)
    end
end

function add_commits_with_no_pr(db_path)
    db = SQLite.DB(db_path)
    artifacts = DBInterface.execute(db, "SELECT * FROM artifact") |> DataFrame
    pr_df = DataFrame()
    commits_to_process = DBInterface.execute(db, "SELECT name FROM artifact WHERE name NOT IN (SELECT bors_sha FROM pull_request_build)") |> DataFrame

    for sha in commits_to_process[!, "name"]
        push!(pr_df, (bors_sha=sha, pr=missing, parent_sha="", exclude=missing, complete=missing, runs=missing, include="ALL", commit_date=missing, requested=missing))
    end

    return pr_df
end

function fixup_parent_sha_and_pr_try_commits(db_path)
    db = SQLite.DB(db_path)
    artifacts = DBInterface.execute(db, "SELECT name FROM artifact WHERE type='try'") |> DataFrame

    root_dir = "$(@__DIR__)/../NanosoldierReports/benchmark/by_hash"
    dirs = readdir(root_dir)
    for sha in artifacts[!, "name"]
        matching_dirs = findall(dir -> dir[1:7] == sha[1:7] || dir[end-6:end] == sha[1:7], dirs)
        if isempty(matching_dirs)
            println("No matching dir for $sha")
        elseif length(matching_dirs) > 1
            println("Multiple matching dirs for $sha")
        else
            dir = dirs[matching_dirs[1]]
            if dir == sha[1:7]
                println("Dir is the same as sha for $sha")
                continue
            end
            println("Processing $sha in $dir")
            report_file = readuntil(joinpath(root_dir, dir, "report.md"), "## Results")
            commit1 = match(r"\*Commit(?:s|\(s\)):\* \[[^/]+/julia@([^\]]+)\]", report_file).captures[1]
            commit2 = match(r" vs \[[A-Za-z]+/julia@([^\]]+)\]", report_file).captures[1]
            parent_sha = commit1 == sha ? commit2 : commit1

            regex_match = match(r"\*Triggered By:\* \[link\]\(https://github.com/JuliaLang/julia/pull/(\d+)", report_file)

            if !isnothing(regex_match)
                triggered_by_pr = regex_match.captures[1] |> x -> parse(Int, x)
                DBInterface.execute(db, "UPDATE pull_request_build SET parent_sha='$parent_sha', pr=$triggered_by_pr WHERE bors_sha='$sha'") |> DataFrame
            else
                printstyled("Commit $sha is a try build but was not triggered in a PR\n", color=:red)
                DBInterface.execute(db, "UPDATE pull_request_build SET parent_sha='$parent_sha' WHERE bors_sha='$sha'") |> DataFrame
            end
        end
    end
end

# CAREFUL some of these commits might actually be daily
function fixup_tag_of_byhash(db_path)
    db = SQLite.DB(db_path)
    artifacts = DBInterface.execute(db, "SELECT name FROM artifact WHERE type='master'") |> DataFrame

    sha_to_tags = Dict{String,Vector{String}}()

    root_dir = "$(@__DIR__)/../NanosoldierReports/benchmark/by_hash"
    dirs = readdir(root_dir)
    for sha in artifacts[!, "name"]
        matching_dirs = findall(dir -> dir[1:7] == sha[1:7] || dir[end-6:end] == sha[1:7], dirs)
        if isempty(matching_dirs)
            # println("No matching dir for $sha")
        elseif length(matching_dirs) > 1
            println("Multiple matching dirs for $sha")
        else
            dir = dirs[matching_dirs[1]]
            if dir == sha[1:7]
                println("Dir is the same as sha for $sha")
                continue
            end
            println("Processing $sha in $dir")
            report_file = readuntil(joinpath(root_dir, dir, "report.md"), "## Results")
            tag_predicate = match(r"\*Tag Predicate:\* `(.*)`", report_file).captures[1]

            predicate_in_table_query = DBInterface.execute(db, "SELECT include FROM pull_request_build WHERE bors_sha='$sha'") |> DataFrame
            @assert size(predicate_in_table_query, 1) == 1
            predicate_in_table_query = predicate_in_table_query[1, "include"]

            if tag_predicate != predicate_in_table_query
                if sha ∉ keys(sha_to_tags)
                    sha_to_tags[sha] = []
                end
                push!(sha_to_tags[sha], string(tag_predicate))
            end
        end
    end

    daily_shas = Set()

    root_dir = "$(@__DIR__)/../NanosoldierReports/benchmark/by_date"
    dirs = readdir(root_dir)
    for yyyy_mm in dirs, day in readdir(joinpath(root_dir, yyyy_mm))
        dir = joinpath(yyyy_mm, day)
        # println("Processing $dir")
        report_file = readuntil(joinpath(root_dir, dir, "report.md"), "## Results")
        sha = match(r"\*Commit(?:s|\(s\)|):\* \[[^/]+/julia@([^\]]+)\]", report_file).captures[1]
        push!(daily_shas, sha)
    end

    println(length(keys(sha_to_tags) ∩ daily_shas))
    sha_to_tags
end

function fixup_tag_of_daily(db_path)
    db = SQLite.DB(db_path)

    sha_to_tags = Dict{String,Vector{String}}()

    root_dir = "$(@__DIR__)/../NanosoldierReports/benchmark/by_date"
    dirs = readdir(root_dir)
    for yyyy_mm in dirs, day in readdir(joinpath(root_dir, yyyy_mm))
        dir = joinpath(yyyy_mm, day)
        # println("Processing $dir")
        report_file = readuntil(joinpath(root_dir, dir, "report.md"), "## Results")
        sha = match(r"\*Commit(?:s|\(s\)|):\* \[[^/]+/julia@([^\]]+)\]", report_file).captures[1]
        tag_predicate = match(r"\*Tag Predicate:\* `(.*)`", report_file).captures[1]

        if tag_predicate != "ALL"
            push!(sha_to_tags[sha], string(tag_predicate))
        end
        println("$dir had $tag_predicate")
        # DBInterface.execute(db, "UPDATE pull_request_build SET include='$tag_predicate' WHERE bors_sha='$sha'") |> DataFrame
    end

    sha_to_tags
end

function add_pr_nums(db_path)
    db = SQLite.DB(db_path)
    artifacts = DBInterface.execute(db, "SELECT * FROM artifact") |> DataFrame
    pr_df = DataFrame()

    for row in eachrow(artifacts)
        if row["type"] == "master"
            sha = row["name"]
            pr_details = HTTP.get("https://api.github.com/repos/JuliaLang/Julia/commits/$sha/pulls", headers=headers).body |> JSON3.read
            if !isempty(pr_details) && haskey(pr_details[1], :number)
                pr_num = pr_details[1].number
                push!(pr_df, (bors_sha=sha, pr=pr_num, parent_sha="", exclude=missing, complete=missing, runs=missing, include="ALL", commit_date=missing, requested=missing))
                # DBInterface.execute(db, "INSERT INTO pull_request_build (bors_sha, pr) VALUES ('$sha', $pr_num)")
            else
                println("Commit $sha has no PR")
                push!(pr_df, (bors_sha=sha, pr=missing, parent_sha="", exclude=missing, complete=missing, runs=missing, include="ALL", commit_date=missing, requested=missing))
            end
        else
            # Doesn't really work due to force pushes I think, e.g. https://github.com/JuliaLang/julia/commit/4ff1f007974a4ea1d89e636d8feed83723bbb779 has no pr
            sha = row["name"]
            pr_details = HTTP.get("https://api.github.com/repos/JuliaLang/Julia/commits/$sha/pulls", headers=headers).body |> JSON3.read
            if !isempty(pr_details)
                mini = 0
                min_created_date = nothing
                for i in eachindex(pr_details)
                    if isnothing(min_created_date) || DateTime(pr_details[i].created_at, dateformat"yyyy-mm-ddTHH:MM:SS\Z") < min_created_date
                        min_created_date = DateTime(pr_details[i].created_at, dateformat"yyyy-mm-ddTHH:MM:SS\Z")
                        mini = i
                    end
                end

                pr_num = pr_details[mini].number

                commit_details = HTTP.get("https://api.github.com/repos/JuliaLang/Julia/git/commits/$sha").body |> JSON3.read # can remove /git/ not sure why it's there
                date = DateTime(commit_details.author.date, dateformat"yyyy-mm-ddTHH:MM:SS\Z") |> datetime2unix |> Int64
                parent_sha = commit_details.parents[1].sha # How do you deal with multiple parents? Presumably we want the one on master

                push!(pr_df, (bors_sha=sha, pr=pr_num, parent_sha=parent_sha, exclude=missing, complete=missing, runs=missing, include="ALL", commit_date=date, requested=missing))
                # DBInterface.execute(db, "INSERT INTO pull_request_build (bors_sha, pr, parent_sha, commit_date) VALUES ('$sha', $pr_num, '$parent_sha', $(artifact_row.date))")
            else
                println("Commit $sha has no PR")
                push!(pr_df, (bors_sha=sha, pr=missing, parent_sha=parent_sha, exclude=missing, complete=missing, runs=missing, include="ALL", commit_date=date, requested=missing))
            end
        end
    end

    return pr_df
    pr_df |> SQLite.load!(db, "pull_request_build")
end

function create_sample_pstat_df(db_path)
    @time begin
        _db = SQLite.DB(db_path)

        pstat_series_table = DBInterface.execute(_db, "SELECT * FROM pstat_series") |> DataFrame
        # need to tranform into vector as indexing into df extremely slow
        names_col = pstat_series_table[:, "crate"]
        metrics_col = pstat_series_table[:, "metric"]
        pstat_series_id_column = pstat_series_table[:, "id"]

        benchmark_to_pstat_series_id = Dict((name, metric) => id for (id, name, metric) in zip(pstat_series_id_column, names_col, metrics_col))

        # dir = "$(@__DIR__)/../NanosoldierReports/0a05a13_vs_13d3efb"
        dir = "$(@__DIR__)/../NanosoldierReports/13"
    end

    db = SQLite.DB("results.db")
    artifact_id_query = DBInterface.execute(db, "SELECT id FROM artifact ORDER BY id DESC LIMIT 1") |> DataFrame
    next_artifact_id = (isempty(artifact_id_query) ? 0 : artifact_id_query[1, "id"]) + 1

    @time begin
        df = DataFrame()

        processed_commits = Dict{String,Int}()

        for file in readdir(dir)
            if !endswith(file, ".json") || !contains(file, r".(minimum|median|mean).json")
                continue
            end
            primary_metric = split(file, '.')[end-1]
            if primary_metric == "minimum"
                primary_metric = "min"
            end

            sha = get_sha(file)

            group = BenchmarkTools.load(joinpath(dir, file))[1]
            benchmark_data = rec_flatten_benchmarkgroup(group)

            artifact_id = get(processed_commits, sha, next_artifact_id)

            for (benchmark_name, trial) in benchmark_data
                if sha ∉ keys(processed_commits) # get alloc and memory data, same for all files
                    for metric in (:allocs, :memory) # allocs is num of allocations, memory is in bytes
                        series_id = benchmark_to_pstat_series_id[(benchmark_name, string(metric))]
                        pstat_row = (series=series_id, aid=artifact_id, value=Float64(getfield(trial, metric)))
                        push!(df, pstat_row)
                    end
                end

                series_id = benchmark_to_pstat_series_id[(benchmark_name, primary_metric * "-wall-time")]
                pstat_row = (series=series_id, aid=artifact_id, value=Float64(trial.time))
                push!(df, pstat_row)

                series_id = benchmark_to_pstat_series_id[(benchmark_name, primary_metric * "-gc-time")]
                pstat_row = (series=series_id, aid=artifact_id, value=Float64(trial.gctime))
                push!(df, pstat_row)
            end

            if sha ∉ keys(processed_commits)
                processed_commits[sha] = artifact_id
                next_artifact_id += 1
                DBInterface.execute(db, "INSERT INTO artifact (id, name, type) VALUES ($artifact_id, '$artifact_id', 'master')")
            end
        end
    end

    @time begin
        SQLite.load!(df, db, "pstat")
    end

    return df
end

function create_tags_db(db_path)
    tags_details = HTTP.get("https://api.github.com/repos/JuliaLang/julia/git/refs/tags", headers=headers).body |> JSON3.read

    df = DataFrame((tag=tag_details.ref[length("refs/tags/")+1:end], sha=tag_details.object.sha) for tag_details in tags_details)

    db = SQLite.DB(db_path)
    @time SQLite.load!(df, db, "tags"; replace=true)
end

# Inserting into pstat under different metrics is undesirable as the ui only shows one metric at a time
# function process_log_commit(commit)
#     db = SQLite.DB("/media/rag/NVME/Code/rustc-perf-db/julia.db")

#     commit = "a14cc38512b6daab6b8417ebb8a64fc794ff89cc"
#     artifact_query = DBInterface.execute(db, "SELECT * FROM artifact WHERE name='$(commit)' LIMIT 1") |> DataFrame

#     pstat_series_table = DBInterface.execute(db, "SELECT * FROM pstat_series") |> DataFrame
#     # need to tranform into vector as indexing into df extremely slow
#     names_col = pstat_series_table[:, "crate"]
#     metrics_col = pstat_series_table[:, "metric"]
#     pstat_series_id_column = pstat_series_table[:, "id"]

#     benchmark_to_pstat_series_id = Dict((name, metric) => id for (id, name, metric) in zip(pstat_series_id_column, names_col, metrics_col))

#     artifact_size_df = DataFrame()
#     pstat_df = DataFrame()
#     process_commit!(artifact_size_df, pstat_df, artifact_query[1, :id], commit, "master", identity)
#     SQLite.load!(artifact_size_df, db, "artifact_size")

#     df = DataFrame()
#     for row in eachrow(pstat_df)
#         metric = row.series in ("minor", "major") ? "$(row.series)-pagefaults" : row.series
#         push_metric_to_pstat!(df, db, "init", "median-$metric", artifact_query[1, :id], median(row.value), benchmark_to_pstat_series_id)
#     end
#     SQLite.load!(df, db, "pstat")
# end
